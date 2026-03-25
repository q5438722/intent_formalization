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

// ===== Behavioral Mutation Tests =====

// Test 1: Mutate conclusion — always(p) on one execution does NOT imply valid(p).
// valid(p) requires p to hold on ALL executions, not just one.
// SHOULD FAIL
proof fn test_mutation_always_does_not_imply_valid<T>(p: TempPred<T>, ex: Execution<T>)
    requires
        always(p).satisfied_by(ex),
    ensures
        valid(p),
{
}

// Test 2: Mutate the theorem — try to prove valid(stable(p)) for arbitrary p.
// The theorem only proves always(p) is stable, NOT arbitrary p.
// SHOULD FAIL
proof fn test_mutation_arbitrary_p_not_stable<T>(p: TempPred<T>)
    ensures
        valid(stable(p)),
{
}

// Test 3: Mutate predicate — always(p) on ex does NOT imply always(q) on ex
// for an unrelated predicate q.
// SHOULD FAIL
proof fn test_mutation_always_p_does_not_imply_always_q<T>(
    p: TempPred<T>,
    q: TempPred<T>,
    ex: Execution<T>,
)
    requires
        always(p).satisfied_by(ex),
    ensures
        always(q).satisfied_by(ex),
{
}

}
