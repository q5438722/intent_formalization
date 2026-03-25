use vstd::prelude::*;

fn main() {}

verus! {

pub type StatePred<T> = spec_fn(T) -> bool;

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

impl<T> Execution<T> {
    pub open spec fn head(self) -> T {
        (self.nat_to_state)(0)
    }

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

pub open spec fn lift_state<T>(state_pred: StatePred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| state_pred(ex.head()))
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

proof fn always_lift_state_unfold<T>(ex: Execution<T>, p: StatePred<T>)
    requires always(lift_state(p)).satisfied_by(ex),
    ensures forall |i: nat| p(#[trigger] ex.suffix(i).head()),
{
    always_unfold::<T>(ex, lift_state(p));
}

// ============================================================
// BOUNDARY TESTS — violate preconditions
// ============================================================

// Test B1: No precondition established at all
// SHOULD FAIL
proof fn test_boundary_no_precondition(ex: Execution<int>, p: StatePred<int>)
{
    always_lift_state_unfold::<int>(ex, p); // SHOULD FAIL
}

// Test B2: Only the head satisfies p — not sufficient for always
// SHOULD FAIL
proof fn test_boundary_only_head(ex: Execution<int>, p: StatePred<int>)
    requires p(ex.head()),
{
    always_lift_state_unfold::<int>(ex, p); // SHOULD FAIL
}

// Test B3: Finite prefix (positions 0,1,2) satisfies p — not sufficient for always
// SHOULD FAIL
proof fn test_boundary_finite_prefix(ex: Execution<int>, p: StatePred<int>)
    requires
        p(ex.suffix(0 as nat).head()),
        p(ex.suffix(1 as nat).head()),
        p(ex.suffix(2 as nat).head()),
{
    always_lift_state_unfold::<int>(ex, p); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TESTS — mutate expected outputs/relations
// ============================================================

// Test M1: Negate the postcondition at position 0
// SHOULD FAIL
proof fn test_mutation_negate_at_zero(ex: Execution<int>, p: StatePred<int>)
    requires always(lift_state(p)).satisfied_by(ex),
{
    always_lift_state_unfold::<int>(ex, p);
    assert(!p(ex.suffix(0 as nat).head())); // SHOULD FAIL
}

// Test M2: Assert postcondition for an unrelated predicate q
// SHOULD FAIL
proof fn test_mutation_wrong_predicate(ex: Execution<int>, p: StatePred<int>, q: StatePred<int>)
    requires always(lift_state(p)).satisfied_by(ex),
{
    always_lift_state_unfold::<int>(ex, p);
    assert(forall |i: nat| q(#[trigger] ex.suffix(i).head())); // SHOULD FAIL
}

// Test M3: Assert postcondition for a different execution
// SHOULD FAIL
proof fn test_mutation_wrong_execution(ex1: Execution<int>, ex2: Execution<int>, p: StatePred<int>)
    requires always(lift_state(p)).satisfied_by(ex1),
{
    always_lift_state_unfold::<int>(ex1, p);
    assert(forall |i: nat| p(#[trigger] ex2.suffix(i).head())); // SHOULD FAIL
}

// ============================================================
// LOGICAL TESTS — properties NOT guaranteed by the spec
// ============================================================

// Test L1: Determinism — same always property does NOT imply same head
// SHOULD FAIL
proof fn test_logical_determinism(ex1: Execution<int>, ex2: Execution<int>, p: StatePred<int>)
    requires
        always(lift_state(p)).satisfied_by(ex1),
        always(lift_state(p)).satisfied_by(ex2),
{
    always_lift_state_unfold::<int>(ex1, p);
    always_lift_state_unfold::<int>(ex2, p);
    assert(ex1.head() == ex2.head()); // SHOULD FAIL
}

// Test L2: Structural collapse — always(lift_state(p)) does NOT imply all states equal
// SHOULD FAIL
proof fn test_logical_all_states_equal(ex: Execution<int>, p: StatePred<int>)
    requires always(lift_state(p)).satisfied_by(ex),
{
    always_lift_state_unfold::<int>(ex, p);
    assert(forall |i: nat, j: nat|
        #[trigger] (ex.nat_to_state)(i) == #[trigger] (ex.nat_to_state)(j)); // SHOULD FAIL
}

// Test L3: Stronger unrelated property — always(lift_state(p)) does NOT imply states > 0
// SHOULD FAIL
proof fn test_logical_stronger_property(ex: Execution<int>, p: StatePred<int>)
    requires always(lift_state(p)).satisfied_by(ex),
{
    always_lift_state_unfold::<int>(ex, p);
    assert(forall |i: nat| (#[trigger] ex.suffix(i).head()) > 0); // SHOULD FAIL
}

}
