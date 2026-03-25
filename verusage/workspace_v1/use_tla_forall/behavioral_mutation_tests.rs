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
// BEHAVIORAL MUTATION TESTS: Mutate expected outputs/relations
// ============================================================

// Test 1: Strengthen conclusion from spec.entails(a_to_p(a)) to valid(a_to_p(a))
// valid(p) means p holds on ALL executions; entails only under spec.
// SHOULD FAIL
proof fn test_mutation_1_strengthen_to_valid(spec: TempPred<int>, a_to_p: spec_fn(int) -> TempPred<int>, a: int)
    requires spec.entails(tla_forall(a_to_p)),
    ensures valid(a_to_p(a)),
{
    use_tla_forall::<int, int>(spec, a_to_p, a);
}

// Test 2: After use_tla_forall with mapping a_to_p, assert result about a DIFFERENT mapping b_to_p
// SHOULD FAIL
proof fn test_mutation_2_wrong_mapping(spec: TempPred<int>, a_to_p: spec_fn(int) -> TempPred<int>, b_to_p: spec_fn(int) -> TempPred<int>, a: int)
    requires spec.entails(tla_forall(a_to_p)),
    ensures spec.entails(b_to_p(a)),
{
    use_tla_forall::<int, int>(spec, a_to_p, a);
}

// Test 3: Reverse entailment — conclude a_to_p(a).entails(spec) from spec.entails(a_to_p(a))
// Entailment is NOT symmetric in general.
// SHOULD FAIL
proof fn test_mutation_3_reverse_entailment(spec: TempPred<int>, a_to_p: spec_fn(int) -> TempPred<int>, a: int)
    requires spec.entails(tla_forall(a_to_p)),
    ensures a_to_p(a).entails(spec),
{
    use_tla_forall::<int, int>(spec, a_to_p, a);
}

// Test 4: Assert spec entails the NEGATION of a_to_p(a) — directly contradicts correct postcondition
// SHOULD FAIL
proof fn test_mutation_4_negate_conclusion(spec: TempPred<int>, a_to_p: spec_fn(int) -> TempPred<int>, a: int)
    requires spec.entails(tla_forall(a_to_p)),
    ensures spec.entails(TempPred::<int>::new(|ex: Execution<int>| !a_to_p(a).satisfied_by(ex))),
{
    use_tla_forall::<int, int>(spec, a_to_p, a);
}

}
