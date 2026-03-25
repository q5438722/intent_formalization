use vstd::prelude::*;

fn main() {}

verus!{

// ===== Definitions from target file =====

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
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    unimplemented!()
}

pub proof fn spec_entails_tla_forall<T, A>(spec: TempPred<T>, a_to_p: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| spec.entails(#[trigger] a_to_p(a)),
    ensures spec.entails(tla_forall(a_to_p)),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies tla_forall(a_to_p).satisfied_by(ex) by {
        assert forall |a| #[trigger] a_to_p(a).satisfied_by(ex) by {
            implies_apply::<T>(ex, spec, a_to_p(a));
        };
    };
}

// ===== LOGICAL TESTS =====

// Test 1: Entailment does NOT imply validity of spec.
// Just because spec entails something doesn't make spec itself valid (true on all executions).
// SHOULD FAIL
proof fn test_logical_entailment_implies_spec_valid<T, A>(
    spec: TempPred<T>,
    a_to_p: spec_fn(A) -> TempPred<T>,
)
    requires forall |a: A| spec.entails(#[trigger] a_to_p(a)),
{
    spec_entails_tla_forall(spec, a_to_p);
    assert(valid(spec)); // unwarranted: entailing things doesn't make spec universally true
}

// Test 2: Try to derive false from implies_apply (soundness test).
// Modus ponens with valid premises should not yield absurdity.
// SHOULD FAIL
proof fn test_logical_implies_apply_derive_false<T>(
    ex: Execution<T>,
    p: TempPred<T>,
    q: TempPred<T>,
)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
{
    implies_apply(ex, p, q);
    assert(false); // absurdity: should not be derivable from valid modus ponens
}

// Test 3: Individual entailment does NOT imply the converse for a specific element.
// forall a. spec.entails(a_to_p(a)) does NOT mean a_to_p(a0).entails(spec).
// SHOULD FAIL
proof fn test_logical_individual_converse<T, A>(
    spec: TempPred<T>,
    a_to_p: spec_fn(A) -> TempPred<T>,
    a0: A,
)
    requires forall |a: A| spec.entails(#[trigger] a_to_p(a)),
{
    assert(a_to_p(a0).entails(spec)); // converse for individual element is not guaranteed
}

// Test 4: spec.entails(tla_forall(a_to_p)) does NOT imply valid(tla_forall(a_to_p)).
// Entailment is conditional on spec; validity is unconditional.
// SHOULD FAIL
proof fn test_logical_entailment_not_validity<T, A>(
    spec: TempPred<T>,
    a_to_p: spec_fn(A) -> TempPred<T>,
)
    requires
        spec.entails(tla_forall(a_to_p)),
{
    assert(valid(tla_forall(a_to_p))); // lifting entailment to validity is invalid without valid(spec)
}

}
