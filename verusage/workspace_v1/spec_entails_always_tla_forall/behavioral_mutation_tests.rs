use vstd::prelude::*;

fn main() {}

verus!{

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

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ===== Helper functions (from source) =====

#[verifier::external_body]
pub proof fn tla_forall_always_equality_variant<T, A>(a_to_always: spec_fn(A) -> TempPred<T>, a_to_p: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #![trigger a_to_always(a)] a_to_always(a).entails((|a: A| always(a_to_p(a)))(a)) && ((|a: A| always(a_to_p(a)))(a)).entails(a_to_always(a)),
    ensures tla_forall(a_to_always) == always(tla_forall(a_to_p)),
{
    unimplemented!()
}

pub proof fn spec_entails_always_tla_forall<T, A>(spec: TempPred<T>, a_to_p: spec_fn(A)->TempPred<T>)
    requires forall |a: A| spec.entails(always(#[trigger] a_to_p(a))),
    ensures spec.entails(always(tla_forall(a_to_p))),
{
    let a_to_always = |a: A| always(a_to_p(a));
    spec_entails_tla_forall(spec, a_to_always);
    tla_forall_always_equality_variant::<T, A>(a_to_always, a_to_p);
}

#[verifier::external_body]
pub proof fn spec_entails_tla_forall<T, A>(spec: TempPred<T>, a_to_p: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| spec.entails(#[trigger] a_to_p(a)),
    ensures spec.entails(tla_forall(a_to_p)),
{
    unimplemented!()
}

// ===== Behavioral Mutation Tests =====

// SHOULD FAIL: Mutated output — asserts valid() instead of spec.entails()
// Drops the spec constraint entirely, claiming the property holds universally.
proof fn test_mutation_drop_spec(spec: TempPred<int>, a_to_p: spec_fn(nat) -> TempPred<int>)
    requires forall |a: nat| spec.entails(always(#[trigger] a_to_p(a))),
{
    spec_entails_always_tla_forall::<int, nat>(spec, a_to_p);
    // SHOULD FAIL: valid() is much stronger than spec.entails()
    assert(valid(always(tla_forall(a_to_p))));
}

// SHOULD FAIL: Reversed entailment direction
// The function guarantees spec => always(tla_forall(a_to_p)),
// but we try to assert the reverse direction.
proof fn test_mutation_reverse_entailment(spec: TempPred<int>, a_to_p: spec_fn(nat) -> TempPred<int>)
    requires forall |a: nat| spec.entails(always(#[trigger] a_to_p(a))),
{
    spec_entails_always_tla_forall::<int, nat>(spec, a_to_p);
    // SHOULD FAIL: reverse direction not guaranteed
    assert(always(tla_forall(a_to_p)).entails(spec));
}

// SHOULD FAIL: Weaker precondition but same strong conclusion
// Only has spec.entails(a_to_p(a)) (no always in precondition),
// but tries to conclude spec.entails(always(tla_forall(a_to_p))).
proof fn test_mutation_upgrade_without_always(spec: TempPred<int>, a_to_p: spec_fn(nat) -> TempPred<int>)
    requires forall |a: nat| spec.entails(#[trigger] a_to_p(a)),
{
    // SHOULD FAIL: cannot conclude always(...) without always in precondition
    assert(spec.entails(always(tla_forall(a_to_p))));
}

}
