use vstd::prelude::*;

fn main() {}

verus!{

// === Definitions ===

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

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// === Axioms ===

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{ unimplemented!() }

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{ unimplemented!() }

// Include the original theorem for tests that attempt to misuse it
pub proof fn always_implies_preserved_by_always<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p.implies(q))),
    ensures spec.entails(always(always(p).implies(always(q)))),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies always(always(p).implies(always(q))).satisfied_by(ex) by {
        assert forall |i| #[trigger] always(p).satisfied_by(ex.suffix(i)) implies always(q).satisfied_by(ex.suffix(i)) by {
            assert forall |j| #[trigger] q.satisfied_by(ex.suffix(i).suffix(j)) by {
                implies_apply::<T>(ex, spec, always(p.implies(q)));
                always_unfold::<T>(ex, p.implies(q));
                execution_equality::<T>(ex.suffix(i + j), ex.suffix(i).suffix(j));
                always_unfold::<T>(ex.suffix(i), p);
                implies_apply::<T>(ex.suffix(i).suffix(j), p, q);
            };
        };
    };
}

// === Logical Tests ===

// Test 1: Drop spec - promote entailment to validity
// spec.entails(X) does NOT imply valid(X). The spec restricts which executions matter.
// SHOULD FAIL
proof fn test_drop_spec_to_valid<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p.implies(q))),
    ensures valid(always(always(p).implies(always(q)))),
{
    always_implies_preserved_by_always::<T>(spec, p, q);
}

// Test 2: Different suffixes are not equal
// ex.suffix(0) and ex.suffix(1) are extensionally different in general.
// SHOULD FAIL
proof fn test_different_suffixes_equal<T>(ex: Execution<T>)
    ensures ex.suffix(0) == ex.suffix(1),
{
}

// Test 3: Entailment is not symmetric
// spec.entails(X) does not imply X.entails(spec).
// SHOULD FAIL
proof fn test_entailment_symmetric<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p.implies(q))),
    ensures always(p.implies(q)).entails(spec),
{
}

// Test 4: valid(p) from always(p) satisfaction on one execution
// Knowing always(p) on a single execution does not make p universally valid.
// SHOULD FAIL
proof fn test_single_execution_to_valid<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures valid(p),
{
    always_unfold::<T>(ex, p);
}

}
