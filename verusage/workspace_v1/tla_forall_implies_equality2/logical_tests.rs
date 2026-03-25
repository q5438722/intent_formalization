use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions from target file =====

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

    pub open spec fn or(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) || other.satisfied_by(ex))
    }

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn not<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| !temp_pred.satisfied_by(ex))
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
proof fn tla_forall_or_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    ensures tla_forall(|a: A| a_to_p(a).or(q)) == tla_forall(a_to_p).or(q),
{
    unimplemented!()
}

proof fn tla_forall_implies_equality2<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| p.implies(a_to_q(a))) == p.implies(tla_forall(a_to_q)),
{
    a_to_temp_pred_equality::<T, A>(|a: A| p.implies(a_to_q(a)), |a: A| a_to_q(a).or(not(p)));
    temp_pred_equality::<T>(tla_forall(|a: A| p.implies(a_to_q(a))), tla_forall(|a: A| a_to_q(a).or(not(p))));
    tla_forall_or_equality::<T, A>(a_to_q, not(p));
    temp_pred_equality::<T>(tla_forall(a_to_q).or(not(p)), p.implies(tla_forall(a_to_q)));
}

// ===== Logical Tests =====

// SHOULD FAIL: Commutativity of implies — p→q ≠ q→p in general.
// Implies is NOT symmetric; asserting structural equality of reversed implications
// should be rejected.
proof fn logical_test_implies_not_commutative(p: TempPred<int>, q: TempPred<int>)
    ensures p.implies(q) == q.implies(p),
{
}

// SHOULD FAIL: One-way entailment does NOT imply equality.
// temp_pred_equality requires BOTH directions. Asserting p == q from
// only p.entails(q) is unsound.
proof fn logical_test_one_way_entailment_not_equality(p: TempPred<int>, q: TempPred<int>)
    requires p.entails(q),
    ensures p == q,
{
}

// SHOULD FAIL: Universal quantification is NOT the same as a single instance.
// tla_forall(q) ≠ q(0) in general — the spec should not allow collapsing
// ∀ to a single point.
proof fn logical_test_forall_not_instance(a_to_q: spec_fn(int) -> TempPred<int>)
    ensures tla_forall(a_to_q) == a_to_q(0int),
{
}

// SHOULD FAIL: valid(p) does NOT imply valid(not(p)).
// If p holds on all executions, then not(p) holds on none.
// Testing whether the spec conflates validity with its negation.
proof fn logical_test_valid_not_contradiction(p: TempPred<int>)
    requires valid(p),
    ensures valid(not(p)),
{
}

}
