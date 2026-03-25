use vstd::prelude::*;

fn main() {}

verus!{

// ========== Definitions ==========

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

impl<T> Execution<T> {
    pub open spec fn suffix(self, pos: nat) -> Self {
        Execution {
            nat_to_state: |i: nat| (self.nat_to_state)(i + pos),
        }
    }
}

#[verifier(reject_recursive_types(T))]
pub struct TempPred<T> {
    pub pred: spec_fn(Execution<T>) -> bool,
}

impl<T> TempPred<T> {
    pub open spec fn new(pred: spec_fn(Execution<T>) -> bool) -> Self {
        TempPred { pred: pred }
    }

    pub open spec fn satisfied_by(self, execution: Execution<T>) -> bool {
        (self.pred)(execution)
    }

    pub open spec fn and(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) && other.satisfied_by(ex))
    }

    pub open spec fn or(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) || other.satisfied_by(ex))
    }

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn later<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| temp_pred.satisfied_by(ex.suffix(1)))
}

pub open spec fn not<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| !temp_pred.satisfied_by(ex))
}

// ========== Trusted Lemmas ==========

#[verifier::external_body]
proof fn later_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires later(p).satisfied_by(ex),
    ensures p.satisfied_by(ex.suffix(1)),
{ unimplemented!() }

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{ unimplemented!() }

#[verifier::external_body]
proof fn not_eventually_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires not(eventually(p)).satisfied_by(ex),
    ensures forall |i| !p.satisfied_by(#[trigger] ex.suffix(i))
{ unimplemented!() }

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
proof fn always_propagate_forwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires always(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex.suffix(i)),
{ unimplemented!() }

#[verifier::external_body]
proof fn always_p_or_eventually_q_rec<T>(ex: Execution<T>, next: TempPred<T>, p: TempPred<T>, q: TempPred<T>, i: nat)
    requires
        forall |idx| p.satisfied_by(ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx)) ==> p.satisfied_by(ex.suffix(idx + 1)) || q.satisfied_by(ex.suffix(idx + 1)),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| !q.satisfied_by(#[trigger] ex.suffix(idx)),
        p.satisfied_by(ex),
    ensures p.satisfied_by(ex.suffix(i)),
    decreases i,
{ unimplemented!() }

proof fn always_p_or_eventually_q<T>(ex: Execution<T>, next: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.and(next).implies(later(p).or(later(q)))).satisfied_by(ex),
        always(next).satisfied_by(ex),
    ensures always(p.implies(always(p).or(eventually(q)))).satisfied_by(ex),
{
    assert forall |i| p.satisfied_by(#[trigger] ex.suffix(i)) implies
    always(p).satisfied_by(ex.suffix(i)) || eventually(q).satisfied_by(ex.suffix(i)) by {
        always_propagate_forwards::<T>(ex, next, i);
        always_unfold::<T>(ex.suffix(i), next);
        assert forall |idx| p.satisfied_by(#[trigger] ex.suffix(i).suffix(idx)) && next.satisfied_by(ex.suffix(i).suffix(idx))
        implies p.satisfied_by(ex.suffix(i).suffix(idx + 1)) || q.satisfied_by(ex.suffix(i).suffix(idx + 1)) by {
            always_propagate_forwards::<T>(ex, p.and(next).implies(later(p).or(later(q))), i);
            always_propagate_forwards::<T>(ex.suffix(i), p.and(next).implies(later(p).or(later(q))), idx);
            implies_apply::<T>(ex.suffix(i).suffix(idx), p.and(next), later(p).or(later(q)));
            if later(p).satisfied_by(ex.suffix(i).suffix(idx)) {
                later_unfold::<T>(ex.suffix(i).suffix(idx), p);
                execution_equality::<T>(ex.suffix(i).suffix(idx).suffix(1), ex.suffix(i).suffix(idx + 1));
            } else {
                later_unfold::<T>(ex.suffix(i).suffix(idx), q);
                execution_equality::<T>(ex.suffix(i).suffix(idx).suffix(1), ex.suffix(i).suffix(idx + 1));
            }
        };
        if !eventually(q).satisfied_by(ex.suffix(i)) {
            not_eventually_unfold::<T>(ex.suffix(i), q);
            assert forall |j| p.satisfied_by(#[trigger] ex.suffix(i).suffix(j)) by {
                always_p_or_eventually_q_rec::<T>(ex.suffix(i), next, p, q, j);
            };
        }
    };
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{ unimplemented!() }

// ========== LOGICAL TESTS ==========

// Test 1: always(p) alone implies eventually(q) — unrelated predicates
// There is no semantic connection between p and q here.
// SHOULD FAIL
proof fn test_logical_always_implies_eventually<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures eventually(q).satisfied_by(ex),
{
}

// Test 2: Derive always(q) from the main theorem's premises plus p holding initially
// The premises allow q to eventually hold OR p to always hold — not always(q).
// SHOULD FAIL
proof fn test_logical_always_q_from_premises<T>(ex: Execution<T>, next: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.and(next).implies(later(p).or(later(q)))).satisfied_by(ex),
        always(next).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures always(q).satisfied_by(ex),
{
    always_p_or_eventually_q(ex, next, p, q);
}

// Test 3: execution_equality with only partial agreement (at position 0)
// The lemma requires agreement at ALL positions, not just one.
// SHOULD FAIL
proof fn test_logical_partial_execution_equality(ex1: Execution<int>, ex2: Execution<int>)
    requires
        (ex1.nat_to_state)(0) == (ex2.nat_to_state)(0),
    ensures ex1 == ex2,
{
    execution_equality(ex1, ex2);
}

// Test 4: Apply the main theorem's conclusion to a DIFFERENT execution
// The conclusion should only hold for the execution in the premises.
// SHOULD FAIL
proof fn test_logical_wrong_execution<T>(ex: Execution<T>, ex2: Execution<T>, next: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.and(next).implies(later(p).or(later(q)))).satisfied_by(ex),
        always(next).satisfied_by(ex),
    ensures always(p.implies(always(p).or(eventually(q)))).satisfied_by(ex2),
{
    always_p_or_eventually_q(ex, next, p, q);
}

// Test 5: Use always_p_or_eventually_q_rec without the crucial not-q constraint
// Without ruling out q, the induction argument for p's persistence breaks.
// SHOULD FAIL
proof fn test_logical_rec_without_not_q<T>(ex: Execution<T>, next: TempPred<T>, p: TempPred<T>, q: TempPred<T>, i: nat)
    requires
        forall |idx| p.satisfied_by(ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx)) ==> p.satisfied_by(ex.suffix(idx + 1)) || q.satisfied_by(ex.suffix(idx + 1)),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        // Missing: forall |idx| !q.satisfied_by(#[trigger] ex.suffix(idx)),
        p.satisfied_by(ex),
    ensures p.satisfied_by(ex.suffix(i)),
{
    always_p_or_eventually_q_rec(ex, next, p, q, i);
}

}
