use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions (from source) =====

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
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
}

proof fn implies_apply_with_always<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
        always(p).satisfied_by(ex),
    ensures always(q).satisfied_by(ex),
{
    always_unfold::<T>(ex, p.implies(q));
    always_unfold::<T>(ex, p);
}

// ===== LOGICAL TESTS =====
// Properties NOT explicitly guaranteed — check if spec allows unintended reasoning.

// SHOULD FAIL: Transitivity without the base case — □(p→q) ∧ □(q→r) does NOT imply □r without □p
proof fn logical_test_transitivity_without_base<T>(
    ex: Execution<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        always(p.implies(q)).satisfied_by(ex),
        always(q.implies(r)).satisfied_by(ex),
    ensures
        always(r).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
    implies_apply_with_always::<T>(ex, q, r);
}

// SHOULD FAIL: Transfer to unrelated execution — □q on ex1 does NOT transfer to ex2
proof fn logical_test_execution_transfer<T>(
    ex1: Execution<T>, ex2: Execution<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        always(p.implies(q)).satisfied_by(ex1),
        always(p).satisfied_by(ex1),
    ensures
        always(q).satisfied_by(ex2),
{
    implies_apply_with_always::<T>(ex1, p, q);
}

// SHOULD FAIL: Converse without base — □(p→q) alone does NOT imply □(q→p)
proof fn logical_test_converse_without_base<T>(
    ex: Execution<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        always(p.implies(q)).satisfied_by(ex),
    ensures
        always(q.implies(p)).satisfied_by(ex),
{
}

// SHOULD FAIL: Cross-function misuse — using always_unfold on a non-always predicate
proof fn logical_test_unfold_non_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires
        p.satisfied_by(ex),
    ensures
        forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    always_unfold::<T>(ex, p);
}

// SHOULD FAIL: Monotonicity assumption — □p does NOT imply □□p is "stronger" than □p
// Specifically: from □p on ex, try to conclude □p on an arbitrary suffix without using always_unfold
proof fn logical_test_arbitrary_predicate_always<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p).satisfied_by(ex),
    ensures
        always(q).satisfied_by(ex),
{
}

}
