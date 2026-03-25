use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions (copied from target) =====

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
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall|i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn stable<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| temp_pred.implies(always(temp_pred)).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall|ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn always_propagate_forwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires
        always(p).satisfied_by(ex),
    ensures
        always(p).satisfied_by(ex.suffix(i)),
{
    unimplemented!()
}

pub proof fn always_p_is_stable<T>(p: TempPred<T>)
    ensures
        valid(stable(always(p))),
{
    assert forall|ex| #[trigger] always(p).satisfied_by(ex) implies always(always(p)).satisfied_by(ex) by {
        assert forall|i| #[trigger] always(p).satisfied_by(ex.suffix(i)) by {
            always_propagate_forwards::<T>(ex, p, i);
        }
    }
}

// ===== Logical Tests =====

// Test 1: Determinism — two executions satisfying always(p) need NOT be equal.
// The spec says nothing about uniqueness of executions.
// SHOULD FAIL
proof fn test_logical_determinism<T>(
    p: TempPred<T>,
    ex1: Execution<T>,
    ex2: Execution<T>,
)
    requires
        always(p).satisfied_by(ex1),
        always(p).satisfied_by(ex2),
    ensures
        ex1 == ex2,
{
}

// Test 2: Implies is NOT symmetric — p.implies(q) ≠ q.implies(p) in general.
// The spec defines implies as one-directional; symmetry should not hold.
// SHOULD FAIL
proof fn test_logical_implies_not_symmetric<T>(
    p: TempPred<T>,
    q: TempPred<T>,
    ex: Execution<T>,
)
    requires
        p.implies(q).satisfied_by(ex),
    ensures
        q.implies(p).satisfied_by(ex),
{
}

// Test 3: Modus ponens is incomplete without the antecedent.
// always(p => q) does NOT imply always(q) without knowing always(p).
// SHOULD FAIL
proof fn test_logical_modus_ponens_incomplete<T>(
    p: TempPred<T>,
    q: TempPred<T>,
    ex: Execution<T>,
)
    requires
        always(p.implies(q)).satisfied_by(ex),
    ensures
        always(q).satisfied_by(ex),
{
}

}
