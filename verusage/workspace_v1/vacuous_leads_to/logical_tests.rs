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

// ===== LOGICAL TESTS =====
// These tests attempt to derive properties NOT guaranteed by the specification.
// Each SHOULD FAIL verification.

// SHOULD FAIL: Assert valid(p.leads_to(q)) (for ALL executions, not just those satisfying spec).
// The theorem only gives spec.entails(p.leads_to(q)), i.e., restricted to spec-executions.
// This test checks if the spec leaks to unrestricted validity.
proof fn test_logical_unrestricted_validity<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(always(r)),
        p.and(r) == false_pred::<T>(),
    ensures
        valid(p.leads_to(q)),
{
    vacuous_leads_to(spec, p, q, r);
}

// SHOULD FAIL: Attempt to derive spec.entails(always(not(p))) solely from the ensures of
// vacuous_leads_to. While this IS derivable from the preconditions directly, the
// function's ensures clause (p ~> q) alone does not give always(not(p)).
// This tests whether the spec's output is stronger than intended.
proof fn test_logical_always_not_p_from_ensures<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(p.leads_to(q)),
    ensures
        spec.entails(always(not(p))),
{
}

// SHOULD FAIL: Cross-function misuse — use implies_apply without establishing its precondition.
// Try to derive q.satisfied_by(ex) from only p.satisfied_by(ex), without p => q.
proof fn test_logical_implies_apply_missing_implication<T>(
    ex: Execution<T>, p: TempPred<T>, q: TempPred<T>
)
    requires
        p.satisfied_by(ex),
    ensures
        q.satisfied_by(ex),
{
    implies_apply::<T>(ex, p, q);
}

// SHOULD FAIL: Try to conclude spec == true_pred (that spec is trivially true).
// Having spec.entails(always(r)) does not mean spec is universally satisfied.
proof fn test_logical_spec_is_true<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(always(r)),
        p.and(r) == false_pred::<T>(),
    ensures
        spec == true_pred::<T>(),
{
    vacuous_leads_to(spec, p, q, r);
}

// SHOULD FAIL: Try to derive that r entails not(p) globally (outside of spec context).
// The precondition p.and(r)==false_pred uses extensional equality on TempPred which
// is a structural/representation claim; try to extract semantic consequence without spec.
proof fn test_logical_r_entails_not_p_globally<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>
)
    requires
        spec.entails(always(r)),
        p.and(r) == false_pred::<T>(),
    ensures
        valid(lift_state(|s: T| true).implies(not(p))),
{
    vacuous_leads_to(spec, p, q, r);
}


}
