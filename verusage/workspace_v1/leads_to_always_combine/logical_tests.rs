use vstd::prelude::*;

fn main() {}

verus! {

// ========== Definitions from source ==========

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

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall|i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists|i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall|ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ========== Helper lemmas (axioms) ==========

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures
        q.satisfied_by(ex),
{
    unimplemented!()
}

#[verifier::external_body]
proof fn always_propagate_forwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires
        always(p).satisfied_by(ex),
    ensures
        always(p).satisfied_by(ex.suffix(i)),
{
    unimplemented!()
}

#[verifier::external_body]
proof fn eventually_proved_by_witness<T>(ex: Execution<T>, p: TempPred<T>, witness_idx: nat)
    requires
        p.satisfied_by(ex.suffix(witness_idx)),
    ensures
        eventually(p).satisfied_by(ex),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires
        forall|i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures
        ex1 == ex2,
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn always_and_equality<T>(p: TempPred<T>, q: TempPred<T>)
    ensures
        always(p.and(q)) == always(p).and(always(q)),
{
    unimplemented!()
}

// ========== LOGICAL TESTS ==========
// Each test asserts a property NOT guaranteed by the specification. SHOULD FAIL.

// SHOULD FAIL: eventually(p) does NOT imply always(p) — a fundamental temporal logic distinction
proof fn test_logical_1_eventually_does_not_imply_always(
    ex: Execution<int>,
    p: TempPred<int>,
)
    requires
        eventually(p).satisfied_by(ex),
{
    assert(always(p).satisfied_by(ex));
}

// SHOULD FAIL: leads_to is NOT symmetric — p~>q does NOT imply q~>p
proof fn test_logical_2_leads_to_not_symmetric(
    spec: TempPred<int>,
    p: TempPred<int>,
    q: TempPred<int>,
)
    requires
        spec.entails(p.leads_to(q)),
{
    assert(spec.entails(q.leads_to(p)));
}

// SHOULD FAIL: p~>always(q) does NOT imply always(q) without p ever holding
proof fn test_logical_3_leads_to_always_not_direct(
    spec: TempPred<int>,
    p: TempPred<int>,
    q: TempPred<int>,
)
    requires
        spec.entails(p.leads_to(always(q))),
{
    assert(spec.entails(always(q)));
}

// SHOULD FAIL: always(p) at a suffix does NOT imply always(p) at the original execution
proof fn test_logical_4_always_no_reverse_propagation(
    ex: Execution<int>,
    p: TempPred<int>,
    i: nat,
)
    requires
        always(p).satisfied_by(ex.suffix(i)),
        i > 0nat,
{
    assert(always(p).satisfied_by(ex));
}

// SHOULD FAIL: valid(eventually(p)) does NOT imply valid(always(p))
proof fn test_logical_5_valid_eventually_not_valid_always(p: TempPred<int>)
    requires
        valid(eventually(p)),
{
    assert(valid(always(p)));
}

// SHOULD FAIL: entails is NOT symmetric — spec.entails(q) does NOT imply q.entails(spec)
proof fn test_logical_6_entails_not_symmetric(
    spec: TempPred<int>,
    q: TempPred<int>,
)
    requires
        spec.entails(q),
{
    assert(q.entails(spec));
}

}
