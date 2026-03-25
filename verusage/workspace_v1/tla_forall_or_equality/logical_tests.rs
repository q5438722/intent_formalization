use vstd::prelude::*;

fn main() {}

verus!{

// === Shared Definitions (from target file) ===

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

#[verifier(reject_recursive_types(T))]
pub struct TempPred<T> {
    pub pred: spec_fn(Execution<T>) -> bool,
}

impl<T> TempPred<T> {

    pub open spec fn new(pred: spec_fn(Execution<T>) -> bool) -> Self {
        TempPred {
            pred: pred,
        }
    }

    pub open spec fn satisfied_by(self, execution: Execution<T>) -> bool {
        (self.pred)(execution)
    }

    pub open spec fn or(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) || other.satisfied_by(ex))
    }

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }

}


pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}


#[verifier::external_body]
proof fn tla_forall_unfold<T, A>(ex: Execution<T>, a_to_p: spec_fn(A) -> TempPred<T>)
    requires tla_forall(a_to_p).satisfied_by(ex),
    ensures forall |a| #[trigger] a_to_p(a).satisfied_by(ex),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn temp_pred_equality<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        q.entails(p),
    ensures p == q,
{
    unimplemented!()
}

proof fn tla_forall_or_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    ensures tla_forall(|a: A| a_to_p(a).or(q)) == tla_forall(a_to_p).or(q),
{
    let a_to_p_or_q = |a: A| a_to_p(a).or(q);
    assert forall |ex| #[trigger] tla_forall(a_to_p_or_q).satisfied_by(ex)
    implies (tla_forall(a_to_p).or(q)).satisfied_by(ex) by {
        tla_forall_unfold::<T, A>(ex, a_to_p_or_q);
        if !q.satisfied_by(ex) {
            assert forall |a| #[trigger] a_to_p(a).satisfied_by(ex) by {
                assert(a_to_p_or_q(a).satisfied_by(ex));
            };
        }
    };

    temp_pred_equality::<T>(tla_forall(|a: A| a_to_p(a).or(q)), tla_forall(a_to_p).or(q));
}

// === Logical Tests ===

// Test 7: Try to derive valid(p) from valid(p.or(q)).
// This is a STRONGER property than the spec guarantees.
// Disjunction does not imply either disjunct: if Q always holds
// but P never does, valid(P.or(Q)) is true but valid(P) is false.
// SHOULD FAIL
proof fn test_valid_disjunction_implies_valid_left(p: TempPred<int>, q: TempPred<int>)
    requires valid(p.or(q)),
{
    assert(valid(p));
}

// Test 8: Try to derive structural equality p == q from mutual entailment
// WITHOUT calling the temp_pred_equality axiom.
// The spec only provides equality through the explicit axiom call.
// SHOULD FAIL
proof fn test_extensional_equality_without_axiom(p: TempPred<int>, q: TempPred<int>)
    requires
        p.entails(q),
        q.entails(p),
{
    assert(p == q);
}

// Test 9: From the forall-or equality theorem, try to derive
// component-level equality: P(a).or(Q) == P(a) for a specific a.
// The theorem operates on the tla_forall level, NOT on individual components.
// SHOULD FAIL
proof fn test_component_equality_from_forall(a_to_p: spec_fn(int) -> TempPred<int>, q: TempPred<int>)
{
    tla_forall_or_equality::<int, int>(a_to_p, q);
    assert(a_to_p(0int).or(q) == a_to_p(0int));
}

}
