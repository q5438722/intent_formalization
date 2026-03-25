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

// === Behavioral Mutation Tests ===
// Each test starts from valid inputs but asserts an incorrect output/relation.
// All tests SHOULD FAIL verification.

// SHOULD FAIL: Negate the postcondition — assert !p at suffix(i)
proof fn test_mutation_1_negated_postcondition()
{
    let ex = Execution::<int> { nat_to_state: |i: nat| 0int };
    let p = TempPred::<int> { pred: |e: Execution<int>| true };
    let q = TempPred::<int> { pred: |e: Execution<int>| false };
    let next = TempPred::<int> { pred: |e: Execution<int>| true };
    // All preconditions satisfied (p=true, q=false, next=true)
    always_p_or_eventually_q_rec::<int>(ex, next, p, q, 3);
    // Postcondition gives p.satisfied_by(ex.suffix(3)) = true
    // Mutated: assert the negation
    assert(!p.satisfied_by(ex.suffix(3))); // SHOULD FAIL
}

// SHOULD FAIL: Assert q holds at suffix(i) instead of p
proof fn test_mutation_2_q_instead_of_p()
{
    let ex = Execution::<int> { nat_to_state: |i: nat| 0int };
    let p = TempPred::<int> { pred: |e: Execution<int>| true };
    let q = TempPred::<int> { pred: |e: Execution<int>| false };
    let next = TempPred::<int> { pred: |e: Execution<int>| true };
    // All preconditions satisfied
    always_p_or_eventually_q_rec::<int>(ex, next, p, q, 3);
    // Mutated: assert q instead of p at suffix(i)
    assert(q.satisfied_by(ex.suffix(3))); // SHOULD FAIL: q is always false
}

// SHOULD FAIL: Assert wrong logical relationship — next implies q
proof fn test_mutation_3_wrong_relationship()
{
    let ex = Execution::<int> { nat_to_state: |i: nat| 0int };
    let p = TempPred::<int> { pred: |e: Execution<int>| true };
    let q = TempPred::<int> { pred: |e: Execution<int>| false };
    let next = TempPred::<int> { pred: |e: Execution<int>| true };
    // All preconditions satisfied
    always_p_or_eventually_q_rec::<int>(ex, next, p, q, 3);
    // Mutated: assert that next being true implies q is true (wrong entailment)
    assert(next.satisfied_by(ex.suffix(3)) ==> q.satisfied_by(ex.suffix(3))); // SHOULD FAIL
    // next.satisfied_by(...) = true, q.satisfied_by(...) = false
    // true ==> false = false
}

}
