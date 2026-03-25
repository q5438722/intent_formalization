use vstd::prelude::*;

fn main() {}

verus!{

// === Definitions from source ===

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
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

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn entails_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    unimplemented!()
}

pub proof fn use_tla_forall<T, A>(spec: TempPred<T>, a_to_p: spec_fn(A) -> TempPred<T>, a: A)
    requires spec.entails(tla_forall(a_to_p)),
    ensures spec.entails(a_to_p(a)),
{
    assert forall |ex: Execution<T>| #[trigger] spec.satisfied_by(ex) implies (a_to_p(a)).satisfied_by(ex) by {
        entails_apply(ex, spec, tla_forall(a_to_p));
        assert(spec.implies(tla_forall(a_to_p)).satisfied_by(ex));
    };
}

// ============================================================
// BOUNDARY TESTS: Violate preconditions / use invalid inputs
// ============================================================

// Test 1: Call use_tla_forall with NO precondition established
// The precondition spec.entails(tla_forall(a_to_p)) is never assumed or proved.
// SHOULD FAIL
proof fn test_boundary_1_no_precondition(spec: TempPred<int>, a_to_p: spec_fn(int) -> TempPred<int>)
{
    use_tla_forall::<int, int>(spec, a_to_p, 0int);
}

// Test 2: Call entails_apply with p.satisfied_by(ex) but WITHOUT p.entails(q)
// p(ex) is satisfied but p does not entail q.
// SHOULD FAIL
proof fn test_boundary_2_missing_entails(p: TempPred<int>, q: TempPred<int>, ex: Execution<int>)
    requires p.satisfied_by(ex),
{
    entails_apply::<int>(ex, p, q);
}

// Test 3: Call entails_apply with p.entails(q) but WITHOUT p.satisfied_by(ex)
// The entailment holds universally, but p is not satisfied on this specific execution.
// SHOULD FAIL
proof fn test_boundary_3_missing_satisfaction(p: TempPred<int>, q: TempPred<int>, ex: Execution<int>)
    requires p.entails(q),
{
    entails_apply::<int>(ex, p, q);
}

// Test 4: Concrete contradictory case — spec entails specific instances but not the universal.
// spec says first state > 5, a_to_p(a) says first state > a.
// spec does NOT entail tla_forall(a_to_p) since nat_to_state(0) > 5 does not imply > all a.
// SHOULD FAIL
proof fn test_boundary_4_instance_not_universal()
{
    let spec = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 5);
    let a_to_p = |a: int| TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > a);
    use_tla_forall::<int, int>(spec, a_to_p, 0int);
}

}
