use vstd::prelude::*;

fn main() {}

verus!{

// === Definitions (copied from target) ===

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

    pub open spec fn leads_to(self, other: Self) -> Self {
        always(self.implies(eventually(other)))
    }
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn stable<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| temp_pred.implies(always(temp_pred)).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// === Target function (with full proof) ===

pub proof fn tla_forall_a_p_leads_to_q_a_is_stable<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #[trigger] valid(stable(p.leads_to(a_to_q(a)))),
    ensures valid(stable(tla_forall(|a: A| p.leads_to(a_to_q(a))))),
{
    let target = tla_forall(|a: A| p.leads_to(a_to_q(a)));
    assert forall |ex| (forall |a: A| #[trigger] valid(stable(p.leads_to(a_to_q(a))))) implies #[trigger] stable(target).satisfied_by(ex) by {
        assert forall |i| (forall |a: A| #[trigger] valid(stable(p.leads_to(a_to_q(a))))) implies
                    (target.satisfied_by(ex) ==> #[trigger] target.satisfied_by(ex.suffix(i))) by {
            assert forall |a: A| (forall |a: A| #[trigger] valid(stable(p.leads_to(a_to_q(a))))) implies
                        (p.leads_to(a_to_q(a)).satisfied_by(ex) ==> #[trigger] p.leads_to(a_to_q(a)).satisfied_by(ex.suffix(i))) by {
                assert(valid(stable(p.leads_to(a_to_q(a)))));
                assert(stable(p.leads_to(a_to_q(a))).satisfied_by(ex));
                if (p.leads_to(a_to_q(a)).satisfied_by(ex)) {
                    assert(p.leads_to(a_to_q(a)).satisfied_by(ex.suffix(i)));
                }
            }
        }
    }
}

// === Boundary Tests ===

// Test B1: Derive false from the lemma with correct precondition
// The postcondition (valid(stable(...))) should not introduce inconsistency
// SHOULD FAIL
proof fn test_boundary_derive_false<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #[trigger] valid(stable(p.leads_to(a_to_q(a)))),
    ensures false,
{
    tla_forall_a_p_leads_to_q_a_is_stable::<T, A>(p, a_to_q);
}

// Test B2: Call the lemma without establishing the precondition
// The requires clause demands valid(stable(leads_to)) for all a;
// without any assumptions, SMT cannot prove this
// SHOULD FAIL
proof fn test_boundary_no_precondition<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    ensures valid(stable(tla_forall(|a: A| p.leads_to(a_to_q(a))))),
{
    tla_forall_a_p_leads_to_q_a_is_stable::<T, A>(p, a_to_q);
}

// Test B3: Partial precondition - stability for a single value (a=0), not universal
// The lemma requires stability for ALL a, not just one specific value
// SHOULD FAIL
proof fn test_boundary_single_instance(p: TempPred<int>, a_to_q: spec_fn(int) -> TempPred<int>)
    requires valid(stable(p.leads_to(a_to_q(0)))),
    ensures valid(stable(tla_forall(|a: int| p.leads_to(a_to_q(a))))),
{
    tla_forall_a_p_leads_to_q_a_is_stable::<int, int>(p, a_to_q);
}

}
