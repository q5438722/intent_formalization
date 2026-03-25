use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions (from target: always_double.rs) =====

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

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
}

#[verifier::external_body]
proof fn always_propagate_forwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires always(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex.suffix(i)),
{
    unimplemented!()
}

proof fn always_double<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures always(always(p)).satisfied_by(ex),
{
    always_unfold::<T>(ex, p);
    assert forall |i| always(p).satisfied_by(#[trigger] ex.suffix(i)) by {
        always_propagate_forwards::<T>(ex, p, i);
    };
}

// ============================================================
// BOUNDARY TESTS — violate preconditions
// ============================================================

// Test B1: always_unfold called without always(p) precondition
// SHOULD FAIL
proof fn boundary_test_unfold_no_precondition<T>(ex: Execution<T>, p: TempPred<T>)
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    always_unfold::<T>(ex, p);
}

// Test B2: always_propagate_forwards called without always(p) precondition
// SHOULD FAIL
proof fn boundary_test_propagate_no_precondition<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    ensures always(p).satisfied_by(ex.suffix(i)),
{
    always_propagate_forwards::<T>(ex, p, i);
}

// Test B3: always_double called without always(p) precondition
// SHOULD FAIL
proof fn boundary_test_double_no_precondition<T>(ex: Execution<T>, p: TempPred<T>)
    ensures always(always(p)).satisfied_by(ex),
{
    always_double::<T>(ex, p);
}

// ============================================================
// BEHAVIORAL MUTATION TESTS — mutate expected relations
// ============================================================

// Test M1: p at one point does NOT imply always(p) (reverse implication)
// SHOULD FAIL
proof fn mutation_test_single_implies_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires p.satisfied_by(ex),
    ensures always(p).satisfied_by(ex),
{
}

// Test M2: always(p) on ex does NOT transfer to unrelated ex2
// SHOULD FAIL
proof fn mutation_test_always_transfers_execution<T>(ex: Execution<T>, ex2: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex2),
{
}

// Test M3: always(p) does NOT imply negation of p at suffix(0)
// SHOULD FAIL
proof fn mutation_test_always_negated_output<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures !p.satisfied_by(ex.suffix(0)),
{
    always_unfold::<T>(ex, p);
}

// ============================================================
// LOGICAL TESTS — properties not entailed by the spec
// ============================================================

// Test L1: Finitely many satisfactions do NOT imply always
// SHOULD FAIL
proof fn logical_test_finite_implies_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires
        p.satisfied_by(ex.suffix(0)),
        p.satisfied_by(ex.suffix(1)),
        p.satisfied_by(ex.suffix(2)),
    ensures always(p).satisfied_by(ex),
{
}

// Test L2: always(p) at a suffix does NOT imply always(p) at the original (no backward propagation)
// SHOULD FAIL
proof fn logical_test_no_backward_propagation<T>(ex: Execution<T>, p: TempPred<T>, j: nat)
    requires
        j > 0,
        always(p).satisfied_by(ex.suffix(j)),
    ensures always(p).satisfied_by(ex),
{
}

// Test L3: always(p) does NOT imply always(q) for unrelated q
// SHOULD FAIL
proof fn logical_test_always_unrelated_predicate<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures always(q).satisfied_by(ex),
{
}

}
