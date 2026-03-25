use vstd::prelude::*;

fn main() {}

verus!{

// === Type Definitions (from target) ===

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
    pub open spec fn and(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) && other.satisfied_by(ex))
    }
    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }
    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
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

pub proof fn entails_and_temp<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(p),
        spec.entails(q),
    ensures spec.entails(p.and(q)),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies p.and(q).satisfied_by(ex) by {
        implies_apply::<T>(ex, spec, p);
        implies_apply::<T>(ex, spec, q);
    };
}

// ============================================================
// BEHAVIORAL MUTATION TESTS — Mutate expected outputs/relations
// ============================================================

// Test 1: Mutate postcondition to global validity
// valid(p.and(q)) is strictly stronger than spec.entails(p.and(q)):
// entails only guarantees the conjunction under spec, not universally.
// SHOULD FAIL
proof fn mutation_valid_instead_of_entails(
    s: TempPred<int>, p: TempPred<int>, q: TempPred<int>
)
    requires
        s.entails(p),
        s.entails(q),
{
    entails_and_temp::<int>(s, p, q);
    assert(valid(p.and(q)));
}

// Test 2: Mutate entailment direction — reversed
// p.and(q).entails(spec) does NOT follow from spec.entails(p.and(q)).
// SHOULD FAIL
proof fn mutation_reversed_entailment(
    s: TempPred<int>, p: TempPred<int>, q: TempPred<int>
)
    requires
        s.entails(p),
        s.entails(q),
{
    entails_and_temp::<int>(s, p, q);
    assert(p.and(q).entails(s));
}

// Test 3: Mutate to include an extra unrelated conjunct
// spec.entails(p.and(q).and(r)) requires spec.entails(r) too, which is absent.
// SHOULD FAIL
proof fn mutation_extra_conjunct(
    s: TempPred<int>, p: TempPred<int>, q: TempPred<int>, r: TempPred<int>
)
    requires
        s.entails(p),
        s.entails(q),
{
    entails_and_temp::<int>(s, p, q);
    assert(s.entails(p.and(q).and(r)));
}

// Test 4: Mutate to claim individual validity from entailment
// valid(p) does NOT follow from spec.entails(p): validity is universal,
// entailment is conditional on spec.
// SHOULD FAIL
proof fn mutation_individual_validity(
    s: TempPred<int>, p: TempPred<int>, q: TempPred<int>
)
    requires
        s.entails(p),
        s.entails(q),
{
    entails_and_temp::<int>(s, p, q);
    assert(valid(p));
}

}
