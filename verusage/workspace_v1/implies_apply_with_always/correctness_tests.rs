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

// ==========================================================================
// BOUNDARY TESTS — violate preconditions to check invalid inputs are rejected
// ==========================================================================

// SHOULD FAIL: Missing □(p→q) — only □p provided
proof fn boundary_test_missing_implication<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p).satisfied_by(ex),
    ensures
        always(q).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// SHOULD FAIL: Missing □p — only □(p→q) provided
proof fn boundary_test_missing_antecedent<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
    ensures
        always(q).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// SHOULD FAIL: Implication holds only at current state, not always
proof fn boundary_test_non_always_implication<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        always(p).satisfied_by(ex),
    ensures
        always(q).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// SHOULD FAIL: p holds only at current state, not always
proof fn boundary_test_non_always_p<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures
        always(q).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// SHOULD FAIL: No preconditions at all
proof fn boundary_test_no_preconditions<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    ensures
        always(q).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// ==========================================================================
// BEHAVIORAL MUTATION TESTS — valid inputs, mutated conclusions
// ==========================================================================

// SHOULD FAIL: Global equivalence — □(p→q) ∧ □p does NOT mean p↔q universally
proof fn mutation_test_global_equivalence<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
        always(p).satisfied_by(ex),
    ensures
        forall |ex2: Execution<T>| p.satisfied_by(ex2) <==> q.satisfied_by(ex2),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// SHOULD FAIL: Arbitrary third predicate r — □(p→q) ∧ □p does NOT imply □r
proof fn mutation_test_arbitrary_conclusion<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
        always(p).satisfied_by(ex),
    ensures
        always(r).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// SHOULD FAIL: Reversed roles — □(p→q) ∧ □q does NOT imply □p
proof fn mutation_test_reversed_roles<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
        always(q).satisfied_by(ex),
    ensures
        always(p).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// SHOULD FAIL: Swapped implication — □(q→p) ∧ □p does NOT imply □q
proof fn mutation_test_swapped_implication<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(q.implies(p)).satisfied_by(ex),
        always(p).satisfied_by(ex),
    ensures
        always(q).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, q, p);
}

// SHOULD FAIL: Weaken conclusion — □(p→q) ∧ □p does NOT imply □(p→r) for arbitrary r
proof fn mutation_test_weaken_conclusion<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
        always(p).satisfied_by(ex),
    ensures
        always(p.implies(r)).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// ==========================================================================
// LOGICAL TESTS — properties NOT explicitly guaranteed by the specification
// ==========================================================================

// SHOULD FAIL: Transitivity without base — □(p→q) ∧ □(q→r) does NOT imply □r without □p
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

// SHOULD FAIL: Execution transfer — □q on ex1 does NOT transfer to unrelated ex2
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

// SHOULD FAIL: Cross-function misuse — always_unfold requires □p, not just p
proof fn logical_test_unfold_non_always<T>(ex: Execution<T>, p: TempPred<T>)
    requires
        p.satisfied_by(ex),
    ensures
        forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    always_unfold::<T>(ex, p);
}

// SHOULD FAIL: Arbitrary predicate — □p does NOT imply □q for unrelated q
proof fn logical_test_arbitrary_predicate_always<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p).satisfied_by(ex),
    ensures
        always(q).satisfied_by(ex),
{
}

}
