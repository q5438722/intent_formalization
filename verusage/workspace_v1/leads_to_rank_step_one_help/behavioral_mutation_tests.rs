use vstd::prelude::*;

fn main() {}

verus!{

// ===== Type Definitions =====

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

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ===== Axioms =====

#[macro_export]
macro_rules! leads_to_trans_n {
    [$($tail:tt)*] => {
        verus_proof_macro_exprs!(leads_to_trans_n_internal!($($tail)*));
    };
}

#[macro_export]
macro_rules! leads_to_trans_n_internal {
    ($spec:expr, $p1:expr, $p2:expr, $p3:expr) => {
        leads_to_trans($spec, $p1, $p2, $p3);
    };
    ($spec:expr, $p1:expr, $p2:expr, $p3:expr, $($tail:tt)*) => {
        leads_to_trans($spec, $p1, $p2, $p3);
        leads_to_trans_n_internal!($spec, $p1, $p3, $($tail)*);
    };
}

#[verifier::external_body]
pub proof fn leads_to_self_temp<T>(p: TempPred<T>)
    ensures valid(p.leads_to(p)),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn leads_to_trans<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        spec.entails(p.leads_to(q)),
        spec.entails(q.leads_to(r)),
    ensures spec.entails(p.leads_to(r)),
{
    unimplemented!()
}

// ===== Function Under Test =====

proof fn leads_to_rank_step_one_help<T>(spec: TempPred<T>, p: spec_fn(nat) -> TempPred<T>, n: nat)
    requires
        forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as nat)))),
    ensures
        spec.entails(p(n).leads_to(p(0))),
    decreases n,
{
    if n > 0 {
        leads_to_rank_step_one_help(spec, p, (n - 1) as nat);
        leads_to_trans_n!(spec, p(n), p((n - 1) as nat), p(0));
    } else {
        leads_to_self_temp(p(0));
    }
}

// ===== BEHAVIORAL MUTATION TESTS =====
// These tests mutate expected outputs or relations.
// All tests SHOULD FAIL verification.

// SHOULD FAIL
// Test 1: Reverse direction — conclude p(0) ~> p(3) from a downward chain.
// The chain only guarantees p(n) ~> p(0), not p(0) ~> p(n).
proof fn test_reverse_direction<T>(spec: TempPred<T>, p: spec_fn(nat) -> TempPred<T>)
    requires
        forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as nat)))),
    ensures spec.entails(p(0).leads_to(p(3))), // mutated: reverse direction
{
    leads_to_rank_step_one_help(spec, p, 3);
}

// SHOULD FAIL
// Test 2: Wrong target — conclude p(3) leads to an arbitrary unrelated predicate q.
// The function only guarantees p(n) ~> p(0), not p(n) ~> q for arbitrary q.
proof fn test_leads_to_arbitrary_target<T>(spec: TempPred<T>, p: spec_fn(nat) -> TempPred<T>, q: TempPred<T>)
    requires
        forall |n: nat| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as nat)))),
    ensures spec.entails(p(3).leads_to(q)), // mutated: arbitrary target
{
    leads_to_rank_step_one_help(spec, p, 3);
}

// SHOULD FAIL
// Test 3: Reversed transitivity conclusion — from p~>q and q~>r, conclude r~>p.
// leads_to_trans only guarantees p~>r, not r~>p.
proof fn test_trans_reverse_conclusion<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        spec.entails(p.leads_to(q)),
        spec.entails(q.leads_to(r)),
    ensures spec.entails(r.leads_to(p)), // mutated: reversed conclusion
{
    leads_to_trans(spec, p, q, r);
}

}
