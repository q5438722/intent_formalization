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

// ========== Boundary Tests ==========

// Test 1: No precondition at all — call not_eventually_by_always_not without any assumption
// SHOULD FAIL
proof fn boundary_no_precondition<T>(ex: Execution<T>, p: TempPred<T>)
    ensures not(eventually(p)).satisfied_by(ex),
{
    not_eventually_by_always_not(ex, p);
}

// Test 2: Only know not(p) at the initial state, not always(not(p))
// SHOULD FAIL
proof fn boundary_only_initial_not<T>(ex: Execution<T>, p: TempPred<T>)
    requires not(p).satisfied_by(ex),
    ensures not(eventually(p)).satisfied_by(ex),
{
    not_eventually_by_always_not(ex, p);
}

// Test 3: Opposite precondition — always(p) instead of always(not(p))
// SHOULD FAIL
proof fn boundary_opposite_precondition<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures not(eventually(p)).satisfied_by(ex),
{
    not_eventually_by_always_not(ex, p);
}

// Test 4: eventually(not(p)) instead of always(not(p)) — weaker precondition
// SHOULD FAIL
proof fn boundary_eventually_not_instead_of_always_not<T>(ex: Execution<T>, p: TempPred<T>)
    requires eventually(not(p)).satisfied_by(ex),
    ensures not(eventually(p)).satisfied_by(ex),
{
    not_eventually_by_always_not(ex, p);
}

// Test 5: Call always_unfold without its precondition
// SHOULD FAIL
proof fn boundary_always_unfold_no_precondition<T>(ex: Execution<T>, p: TempPred<T>)
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    always_unfold(ex, p);
}

}
