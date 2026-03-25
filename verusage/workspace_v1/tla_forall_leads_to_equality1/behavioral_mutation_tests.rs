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

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn tla_exists<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
pub proof fn tla_forall_always_equality_variant<T, A>(a_to_always: spec_fn(A) -> TempPred<T>, a_to_p: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #![trigger a_to_always(a)] a_to_always(a).entails((|a: A| always(a_to_p(a)))(a)) && ((|a: A| always(a_to_p(a)))(a)).entails(a_to_always(a)),
    ensures tla_forall(a_to_always) == always(tla_forall(a_to_p)),
{
    unimplemented!()
}

#[verifier::external_body]
proof fn tla_forall_implies_equality1<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    ensures tla_forall(|a: A| a_to_p(a).implies(q)) == tla_exists(a_to_p).implies(q),
{
    unimplemented!()
}

proof fn tla_forall_leads_to_equality1<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    ensures tla_forall(|a: A| a_to_p(a).leads_to(q)) == tla_exists(a_to_p).leads_to(q),
{
    tla_forall_always_equality_variant::<T, A>(|a: A| a_to_p(a).leads_to(q), |a: A| a_to_p(a).implies(eventually(q)));
    tla_forall_implies_equality1::<T, A>(a_to_p, eventually(q));
}

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Mutate RHS — replace tla_exists with tla_forall
// Correct:  tla_forall(|a| p(a) ~> q) == tla_exists(p) ~> q
// Mutated:  tla_forall(|a| p(a) ~> q) == tla_forall(p) ~> q
// tla_forall(p) is strictly stronger than tla_exists(p), so equality should not hold
// SHOULD FAIL
proof fn mutation_test_forall_replaces_exists_rhs<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>) {
    tla_forall_leads_to_equality1::<T, A>(a_to_p, q);
    assert(tla_forall(|a: A| a_to_p(a).leads_to(q)) == tla_forall(a_to_p).leads_to(q));
}

// Test 2: Mutate LHS — replace tla_forall with tla_exists
// Correct:  tla_forall(|a| p(a) ~> q) == tla_exists(p) ~> q
// Mutated:  tla_exists(|a| p(a) ~> q) == tla_exists(p) ~> q
// Existential quantification over leads-to is weaker than universal
// SHOULD FAIL
proof fn mutation_test_exists_replaces_forall_lhs<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>) {
    tla_forall_leads_to_equality1::<T, A>(a_to_p, q);
    assert(tla_exists(|a: A| a_to_p(a).leads_to(q)) == tla_exists(a_to_p).leads_to(q));
}

// Test 3: Mutate RHS operator — replace leads_to with implies
// Correct:  tla_forall(|a| p(a) ~> q) == tla_exists(p) ~> q
// Mutated:  tla_forall(|a| p(a) ~> q) == tla_exists(p).implies(q)
// leads_to wraps in always+eventually; plain implies is much weaker
// SHOULD FAIL
proof fn mutation_test_implies_replaces_leads_to_rhs<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>) {
    tla_forall_leads_to_equality1::<T, A>(a_to_p, q);
    assert(tla_forall(|a: A| a_to_p(a).leads_to(q)) == tla_exists(a_to_p).implies(q));
}

}
