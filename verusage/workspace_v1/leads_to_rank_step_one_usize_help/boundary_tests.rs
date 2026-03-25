use vstd::prelude::*;

fn main() {}

verus!{

// === Definitions from target file ===

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

proof fn leads_to_rank_step_one_usize_help<T>(spec: TempPred<T>, p: spec_fn(usize) -> TempPred<T>, n: usize)
    requires
        forall |n: usize| #![trigger p(n)] (n > 0 ==> spec.entails(p(n).leads_to(p((n - 1) as usize)))),
    ensures
        spec.entails(p(n).leads_to(p(0))),
    decreases n,
{
    if n > 0 {
        leads_to_rank_step_one_usize_help(spec, p, (n - 1) as usize);
        leads_to_trans_n!(spec, p(n), p((n - 1) as usize), p(0));
    } else {
        leads_to_self_temp(p(0));
    }
}

// === BOUNDARY TESTS ===

// Test 1: No precondition assumed at all
// SHOULD FAIL: calling the function without the universal step-down precondition
proof fn test_boundary_no_precondition<T>(spec: TempPred<T>, p: spec_fn(usize) -> TempPred<T>)
{
    leads_to_rank_step_one_usize_help(spec, p, 5); // SHOULD FAIL
}

// Test 2: Reversed chain direction in precondition
// SHOULD FAIL: provides p(n-1) ~> p(n) instead of the required p(n) ~> p(n-1)
proof fn test_boundary_reversed_precondition<T>(spec: TempPred<T>, p: spec_fn(usize) -> TempPred<T>, n: usize)
    requires
        forall |n: usize| #![trigger p(n)] (n > 0 ==> spec.entails(p((n - 1) as usize).leads_to(p(n)))),
{
    leads_to_rank_step_one_usize_help(spec, p, n); // SHOULD FAIL
}

// Test 3: Only specific step instances, not universal quantification
// SHOULD FAIL: two concrete steps do not satisfy the forall requirement
proof fn test_boundary_partial_precondition<T>(spec: TempPred<T>, p: spec_fn(usize) -> TempPred<T>)
    requires
        spec.entails(p(3).leads_to(p(2))),
        spec.entails(p(2).leads_to(p(1))),
        // Missing: spec.entails(p(1).leads_to(p(0)))
{
    leads_to_rank_step_one_usize_help(spec, p, 3); // SHOULD FAIL
}

}
