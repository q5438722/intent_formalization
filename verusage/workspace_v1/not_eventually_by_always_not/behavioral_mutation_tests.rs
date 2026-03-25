use vstd::prelude::*;

fn main() {}

verus! {

// ========== Definitions from target file ==========

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
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn not<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| !temp_pred.satisfied_by(ex))
}

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
}

proof fn not_eventually_by_always_not<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(not(p)).satisfied_by(ex),
    ensures not(eventually(p)).satisfied_by(ex),
{
    always_unfold::<T>(ex, not(p));
}

// ========== Behavioral Mutation Tests ==========

// Test 1: Valid precondition, but assert the OPPOSITE of postcondition
// eventually(p) should NOT hold when always(not(p)) holds
// SHOULD FAIL
proof fn mutation_assert_opposite_postcondition<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(not(p)).satisfied_by(ex),
    ensures eventually(p).satisfied_by(ex),
{
    not_eventually_by_always_not(ex, p);
}

// Test 2: Valid precondition, but claim p holds at the initial state
// The spec ensures not(eventually(p)), not that p holds
// SHOULD FAIL
proof fn mutation_assert_p_holds<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(not(p)).satisfied_by(ex),
    ensures p.satisfied_by(ex),
{
    not_eventually_by_always_not(ex, p);
}

// Test 3: Valid precondition, but claim always(p) — completely wrong direction
// SHOULD FAIL
proof fn mutation_assert_always_p<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(not(p)).satisfied_by(ex),
    ensures always(p).satisfied_by(ex),
{
    not_eventually_by_always_not(ex, p);
}

// Test 4: Valid precondition, mutate to claim not(always(not(p))) — negate the precondition
// SHOULD FAIL
proof fn mutation_negate_precondition_as_postcondition<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(not(p)).satisfied_by(ex),
    ensures not(always(not(p))).satisfied_by(ex),
{
    not_eventually_by_always_not(ex, p);
}

// Test 5: Valid precondition, mutate postcondition to not(always(p)) — different from not(eventually(p))
// not(always(p)) is weaker than not(eventually(p)); this might pass if spec is too loose
// SHOULD FAIL
proof fn mutation_weaker_postcondition<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(not(p)).satisfied_by(ex),
    ensures not(always(p)).satisfied_by(ex),
{
    not_eventually_by_always_not(ex, p);
}

}
