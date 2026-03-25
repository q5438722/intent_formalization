use vstd::prelude::*;

fn main() {}

verus!{

// ===== Definitions (from source) =====

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
    pub open spec fn and(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) && other.satisfied_by(ex))
    }
    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }
    pub open spec fn leads_to(self, other: Self) -> Self {
        always(self.implies(eventually(other)))
    }
    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn lift_state<T>(state_pred: StatePred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| state_pred(ex.head()))
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn not<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| !temp_pred.satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

pub open spec fn true_pred<T>() -> TempPred<T> {
    lift_state(|s: T| true)
}

pub open spec fn false_pred<T>() -> TempPred<T> {
    not(true_pred())
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

pub proof fn vacuous_leads_to<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        spec.entails(always(r)),
        p.and(r) == false_pred::<T>(),
    ensures
        spec.entails(p.leads_to(q)),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies p.leads_to(q).satisfied_by(ex) by {
        assert forall |i| #[trigger] p.satisfied_by(ex.suffix(i)) implies eventually(q).satisfied_by(ex.suffix(i)) by {
            assert_by(!p.satisfied_by(ex.suffix(i)), {
                implies_apply::<T>(ex, spec, always(r));
                assert(r.satisfied_by(ex.suffix(i)));
                if p.satisfied_by(ex.suffix(i)) {
                    assert(p.and(r).satisfied_by(ex.suffix(i)));
                    assert(p.and(r) != false_pred::<T>());
                }
            });
        }
    }
}

// ===== BEHAVIORAL MUTATION TESTS =====
// These tests use valid preconditions but assert WRONG conclusions.
// Each SHOULD FAIL verification.

// SHOULD FAIL: Swapped leads_to direction — assert q ~> p instead of p ~> q.
// The theorem says p leads_to q, not the reverse.
proof fn test_mutation_swap_leads_to<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(always(r)),
        p.and(r) == false_pred::<T>(),
    ensures
        spec.entails(q.leads_to(p)),
{
    vacuous_leads_to(spec, p, q, r);
}

// SHOULD FAIL: Strengthened conclusion — assert always(q) instead of p ~> q.
// The theorem only gives vacuous leads_to; q need not always hold.
proof fn test_mutation_always_q<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(always(r)),
        p.and(r) == false_pred::<T>(),
    ensures
        spec.entails(always(q)),
{
    vacuous_leads_to(spec, p, q, r);
}

// SHOULD FAIL: Mutated conclusion — assert spec.entails(eventually(q)) instead of leads_to.
// p ~> q says "whenever p holds, q eventually holds" but doesn't guarantee q ever occurs.
proof fn test_mutation_eventually_q<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(always(r)),
        p.and(r) == false_pred::<T>(),
    ensures
        spec.entails(eventually(q)),
{
    vacuous_leads_to(spec, p, q, r);
}

// SHOULD FAIL: Replace leads_to with entails — assert p directly entails q.
// p.leads_to(q) is temporal; p.entails(q) is pointwise, a different claim.
proof fn test_mutation_entails_instead_of_leads_to<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(always(r)),
        p.and(r) == false_pred::<T>(),
    ensures
        p.entails(q),
{
    vacuous_leads_to(spec, p, q, r);
}

// SHOULD FAIL: Negate the predicate in leads_to — assert p ~> not(q) instead of p ~> q.
// Even though p never holds under spec, we should not be able to derive p ~> not(q)
// from the ensures of vacuous_leads_to alone.
proof fn test_mutation_leads_to_not_q<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(always(r)),
        p.and(r) == false_pred::<T>(),
    ensures
        spec.entails(p.leads_to(not(q))),
{
    vacuous_leads_to(spec, p, q, r);
}


}
