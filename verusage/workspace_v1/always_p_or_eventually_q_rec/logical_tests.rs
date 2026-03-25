use vstd::prelude::*;

fn main() {}

verus!{

// === Definitions from target file ===

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
    pub open spec fn satisfied_by(self, execution: Execution<T>) -> bool {
        (self.pred)(execution)
    }
}

proof fn always_p_or_eventually_q_rec<T>(ex: Execution<T>, next: TempPred<T>, p: TempPred<T>, q: TempPred<T>, i: nat)
    requires
        forall |idx| p.satisfied_by(ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx)) ==> p.satisfied_by(ex.suffix(idx + 1)) || q.satisfied_by(ex.suffix(idx + 1)),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| !q.satisfied_by(#[trigger] ex.suffix(idx)),
        p.satisfied_by(ex),
    ensures p.satisfied_by(ex.suffix(i)),
    decreases i,
{
    if i == 0 {
        execution_equality::<T>(ex, ex.suffix(0));
    } else {
        always_p_or_eventually_q_rec::<T>(ex, next, p, q, (i-1) as nat);
    }
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{
    unimplemented!()
}

// === Logical Tests ===
// Each test asserts a property NOT explicitly guaranteed by the spec.
// All tests SHOULD FAIL verification.

// SHOULD FAIL: The postcondition only guarantees p on ex.suffix(i), not on arbitrary executions
proof fn test_logical_1_p_on_different_execution()
{
    let ex = Execution::<int> { nat_to_state: |i: nat| 0int };
    let p = TempPred::<int> { pred: |e: Execution<int>| (e.nat_to_state)(0) == 0 };
    let q = TempPred::<int> { pred: |e: Execution<int>| false };
    let next = TempPred::<int> { pred: |e: Execution<int>| true };
    // All preconditions satisfied for constant-0 execution with p = (first state == 0)
    always_p_or_eventually_q_rec::<int>(ex, next, p, q, 0);
    // The postcondition gives p.satisfied_by(ex.suffix(0))
    // Try to apply p to a DIFFERENT execution — not guaranteed by spec
    let other_ex = Execution::<int> { nat_to_state: |i: nat| 1int };
    assert(p.satisfied_by(other_ex)); // SHOULD FAIL: (1 == 0) = false
}

// SHOULD FAIL: The spec doesn't imply p and next are equivalent predicates
proof fn test_logical_2_p_equiv_next()
{
    let ex = Execution::<int> { nat_to_state: |i: nat| 0int };
    let p = TempPred::<int> { pred: |e: Execution<int>| (e.nat_to_state)(0) == 0 };
    let q = TempPred::<int> { pred: |e: Execution<int>| false };
    let next = TempPred::<int> { pred: |e: Execution<int>| true };
    // All preconditions satisfied
    always_p_or_eventually_q_rec::<int>(ex, next, p, q, 0);
    // Try to derive: p <==> next on a witness execution where they differ
    let witness = Execution::<int> { nat_to_state: |i: nat| 1int };
    // p.satisfied_by(witness) = (1 == 0) = false
    // next.satisfied_by(witness) = true
    assert(p.satisfied_by(witness) <==> next.satisfied_by(witness)); // SHOULD FAIL
}

// SHOULD FAIL: The spec doesn't guarantee p holds universally on ALL executions
proof fn test_logical_3_p_universal()
{
    let ex = Execution::<int> { nat_to_state: |i: nat| 0int };
    let p = TempPred::<int> { pred: |e: Execution<int>| (e.nat_to_state)(0) == 0 };
    let q = TempPred::<int> { pred: |e: Execution<int>| false };
    let next = TempPred::<int> { pred: |e: Execution<int>| true };
    // All preconditions satisfied
    always_p_or_eventually_q_rec::<int>(ex, next, p, q, 0);
    // Try to globalize: assert p holds on ALL executions (not just ex and its suffixes)
    assert(forall |e: Execution<int>| p.satisfied_by(e)); // SHOULD FAIL
}

}
