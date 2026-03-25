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

// === Boundary Tests ===
// Each test violates a different precondition of always_p_or_eventually_q_rec.
// All tests SHOULD FAIL verification.

// SHOULD FAIL: R4 violated — p does not hold at the initial state
proof fn test_boundary_1_missing_initial_p()
{
    let ex = Execution::<int> { nat_to_state: |i: nat| 0int };
    let p = TempPred::<int> { pred: |e: Execution<int>| (e.nat_to_state)(0) == 42 };
    let q = TempPred::<int> { pred: |e: Execution<int>| false };
    let next = TempPred::<int> { pred: |e: Execution<int>| true };
    // p.satisfied_by(ex) = (0 == 42) = false → R4 violated
    always_p_or_eventually_q_rec::<int>(ex, next, p, q, 0);
}

// SHOULD FAIL: R3 violated — q holds at suffix(3)
proof fn test_boundary_2_q_sometimes_true()
{
    let ex = Execution::<int> { nat_to_state: |i: nat| i as int };
    let p = TempPred::<int> { pred: |e: Execution<int>| true };
    let q = TempPred::<int> { pred: |e: Execution<int>| (e.nat_to_state)(0) == 3 };
    let next = TempPred::<int> { pred: |e: Execution<int>| true };
    // q.satisfied_by(ex.suffix(3)) = ((ex.nat_to_state)(3) == 3) = (3 == 3) = true
    // So forall |idx| !q.satisfied_by(ex.suffix(idx)) is violated at idx=3
    always_p_or_eventually_q_rec::<int>(ex, next, p, q, 0);
}

// SHOULD FAIL: R2 violated — next is always false
proof fn test_boundary_3_next_always_false()
{
    let ex = Execution::<int> { nat_to_state: |i: nat| 0int };
    let p = TempPred::<int> { pred: |e: Execution<int>| true };
    let q = TempPred::<int> { pred: |e: Execution<int>| false };
    let next = TempPred::<int> { pred: |e: Execution<int>| false };
    // next.satisfied_by(ex.suffix(idx)) = false for all idx → R2 violated
    always_p_or_eventually_q_rec::<int>(ex, next, p, q, 0);
}

// SHOULD FAIL: R1 violated — step implication fails at idx=0
proof fn test_boundary_4_step_implication_violated()
{
    let ex = Execution::<int> { nat_to_state: |i: nat| i as int };
    let p = TempPred::<int> { pred: |e: Execution<int>| (e.nat_to_state)(0) == 0 };
    let q = TempPred::<int> { pred: |e: Execution<int>| false };
    let next = TempPred::<int> { pred: |e: Execution<int>| true };
    // R4: p(ex) = (0 == 0) = true ✓
    // R2: next everywhere ✓
    // R3: q never ✓
    // R1 at idx=0: p(suffix(0))=true ∧ next(suffix(0))=true ⟹ p(suffix(1))∨q(suffix(1))
    //   = true ⟹ (1==0)∨false = true ⟹ false → VIOLATED
    always_p_or_eventually_q_rec::<int>(ex, next, p, q, 1);
}

}
