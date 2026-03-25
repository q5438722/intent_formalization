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

// ===== BEHAVIORAL MUTATION TESTS =====
// Valid inputs, but mutated conclusions to check incorrect outputs are rejected.

// SHOULD FAIL: Global equivalence — □(p→q) ∧ □p does NOT mean p↔q holds universally
proof fn mutation_test_global_equivalence<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
        always(p).satisfied_by(ex),
    ensures
        forall |ex2: Execution<T>| p.satisfied_by(ex2) <==> q.satisfied_by(ex2),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// SHOULD FAIL: Arbitrary third predicate — from □(p→q) ∧ □p, try to conclude □r
proof fn mutation_test_arbitrary_conclusion<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
        always(p).satisfied_by(ex),
    ensures
        always(r).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// SHOULD FAIL: Reversed roles — from □(p→q) ∧ □q, try to conclude □p
proof fn mutation_test_reversed_roles<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
        always(q).satisfied_by(ex),
    ensures
        always(p).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
}

// SHOULD FAIL: Mutate implication direction — from □(q→p) ∧ □p, try to conclude □q
proof fn mutation_test_swapped_implication<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        always(q.implies(p)).satisfied_by(ex),
        always(p).satisfied_by(ex),
    ensures
        always(q).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, q, p);
}

// SHOULD FAIL: Weaken conclusion — from □(p→q) ∧ □p, try to conclude □(p→r) for unrelated r
proof fn mutation_test_weaken_conclusion<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        always(p.implies(q)).satisfied_by(ex),
        always(p).satisfied_by(ex),
    ensures
        always(p.implies(r)).satisfied_by(ex),
{
    implies_apply_with_always::<T>(ex, p, q);
}

}
