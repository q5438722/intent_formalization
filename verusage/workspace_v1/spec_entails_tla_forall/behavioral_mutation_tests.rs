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

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Mutate conclusion - conclude valid(tla_forall(a_to_p)) instead of spec.entails(tla_forall(a_to_p))
// valid(X) is strictly stronger than spec.entails(X) because it removes the spec guard.
// SHOULD FAIL
proof fn test_mutation_conclude_valid_instead_of_entails<T, A>(
    spec: TempPred<T>,
    a_to_p: spec_fn(A) -> TempPred<T>,
)
    requires forall |a: A| spec.entails(#[trigger] a_to_p(a)),
{
    spec_entails_tla_forall(spec, a_to_p);
    assert(valid(tla_forall(a_to_p))); // mutated: valid instead of entails
}

// Test 2: Mutate conclusion - reverse entailment direction
// spec.entails(tla_forall(a_to_p)) does NOT imply tla_forall(a_to_p).entails(spec).
// SHOULD FAIL
proof fn test_mutation_reversed_entailment<T, A>(
    spec: TempPred<T>,
    a_to_p: spec_fn(A) -> TempPred<T>,
)
    requires forall |a: A| spec.entails(#[trigger] a_to_p(a)),
{
    spec_entails_tla_forall(spec, a_to_p);
    assert(tla_forall(a_to_p).entails(spec)); // mutated: reversed direction
}

// Test 3: Mutate conclusion - conclude spec entails an unrelated predicate
// Just because spec entails tla_forall(a_to_p) doesn't mean it entails anything else.
// SHOULD FAIL
proof fn test_mutation_entails_arbitrary<T, A>(
    spec: TempPred<T>,
    a_to_p: spec_fn(A) -> TempPred<T>,
    other: TempPred<T>,
)
    requires forall |a: A| spec.entails(#[trigger] a_to_p(a)),
{
    spec_entails_tla_forall(spec, a_to_p);
    assert(spec.entails(other)); // mutated: arbitrary predicate
}

// Test 4: Mutate implies_apply conclusion - conclude p instead of q
// After modus ponens p => q with p, conclude p.satisfied_by(ex) is already known,
// but here we try to conclude q.entails(p) which is not guaranteed.
// SHOULD FAIL
proof fn test_mutation_implies_apply_wrong_conclusion<T>(
    ex: Execution<T>,
    p: TempPred<T>,
    q: TempPred<T>,
)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
{
    implies_apply(ex, p, q);
    // q.satisfied_by(ex) is established, but q.entails(p) is not
    assert(q.entails(p)); // mutated: reversed global entailment from pointwise fact
}

}
