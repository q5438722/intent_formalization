use vstd::prelude::*;

fn main() {}

verus!{

// ========== Definitions (from target file) ==========

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

#[verifier::external_body]
pub proof fn temp_pred_equality<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        q.entails(p),
    ensures p == q,
{
    unimplemented!()
}

#[verifier::external_body]
proof fn a_to_temp_pred_equality<T, A>(p: spec_fn(A) -> TempPred<T>, q: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #[trigger] p(a).entails(q(a)) && q(a).entails(p(a)),
    ensures p == q,
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn tla_forall_always_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| always(a_to_p(a))) == always(tla_forall(a_to_p)),
{
    unimplemented!()
}

pub proof fn tla_forall_always_equality_variant<T, A>(a_to_always: spec_fn(A) -> TempPred<T>, a_to_p: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #![trigger a_to_always(a)] a_to_always(a).entails((|a: A| always(a_to_p(a)))(a)) && ((|a: A| always(a_to_p(a)))(a)).entails(a_to_always(a)),
    ensures tla_forall(a_to_always) == always(tla_forall(a_to_p)),
{
    a_to_temp_pred_equality::<T, A>(a_to_always, |a: A| always(a_to_p(a)));
    temp_pred_equality::<T>(tla_forall(a_to_always), tla_forall(|a: A| always(a_to_p(a))));
    tla_forall_always_equality::<T, A>(a_to_p);
}

// ========== Boundary Tests ==========

// Test 1: a_to_always is trivially true, a_to_p is trivially false.
// Precondition violated: "true" does not entail "always(false)" = "false".
// SHOULD FAIL
proof fn boundary_test_true_vs_false() {
    let a_to_always: spec_fn(nat) -> TempPred<nat> =
        |a: nat| TempPred::<nat>::new(|ex: Execution<nat>| true);
    let a_to_p: spec_fn(nat) -> TempPred<nat> =
        |a: nat| TempPred::<nat>::new(|ex: Execution<nat>| false);
    tla_forall_always_equality_variant::<nat, nat>(a_to_always, a_to_p);
}

// Test 2: Only forward entailment holds (a_to_always = false, always(a_to_p) = true).
// false ==> true holds, but true ==> false does not.
// Precondition partially violated.
// SHOULD FAIL
proof fn boundary_test_one_direction_only() {
    let a_to_always: spec_fn(nat) -> TempPred<nat> =
        |a: nat| TempPred::<nat>::new(|ex: Execution<nat>| false);
    let a_to_p: spec_fn(nat) -> TempPred<nat> =
        |a: nat| TempPred::<nat>::new(|ex: Execution<nat>| true);
    tla_forall_always_equality_variant::<nat, nat>(a_to_always, a_to_p);
}

// Test 3: a_to_always(a) is NOT always(a_to_p(a)) — a_to_always = a_to_p directly (no always wrapper).
// Precondition violated: a_to_p(a) does not entail always(a_to_p(a)) in general
// (a property holding at step 0 does not mean it holds at all steps).
// SHOULD FAIL
proof fn boundary_test_missing_always_wrapper() {
    let a_to_p: spec_fn(nat) -> TempPred<nat> =
        |a: nat| TempPred::<nat>::new(|ex: Execution<nat>| (ex.nat_to_state)(0) == a);
    // Passing a_to_p itself as a_to_always (missing the always wrapper)
    tla_forall_always_equality_variant::<nat, nat>(a_to_p, a_to_p);
}

}
