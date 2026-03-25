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

// ===== Boundary Tests =====

// SHOULD FAIL: Calling temp_pred_equality with contradictory predicates (true vs false).
// Violates both directions of entailment.
proof fn boundary_test_contradictory_predicates() {
    let p = TempPred::<int>::new(|ex: Execution<int>| true);
    let q = TempPred::<int>::new(|ex: Execution<int>| false);
    temp_pred_equality(p, q);
    assert(p == q);
}

// SHOULD FAIL: Calling temp_pred_equality with only one-way entailment.
// p entails q (anything implies true), but q does NOT entail p.
proof fn boundary_test_one_way_entailment() {
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 5);
    let q = TempPred::<int>::new(|ex: Execution<int>| true);
    // p.entails(q) holds: (state(0) > 5) ==> true is always true
    // q.entails(p) FAILS: true ==> (state(0) > 5) is NOT always true
    temp_pred_equality(p, q);
    assert(p == q);
}

// SHOULD FAIL: Calling a_to_temp_pred_equality with pointwise non-equivalent functions.
// f(a) is always-true, g(a) is always-false — they disagree everywhere.
proof fn boundary_test_nonequivalent_function_families() {
    let f = |a: int| TempPred::<int>::new(|ex: Execution<int>| true);
    let g = |a: int| TempPred::<int>::new(|ex: Execution<int>| false);
    a_to_temp_pred_equality::<int, int>(f, g);
    assert(f == g);
}

// SHOULD FAIL: Calling temp_pred_equality where entailment depends on
// a specific state value that doesn't hold universally.
proof fn boundary_test_nonuniversal_entailment() {
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) >= 0);
    // p.entails(q) holds: (state(0) == 0) ==> (state(0) >= 0) is always true
    // q.entails(p) FAILS: (state(0) >= 0) ==> (state(0) == 0) is NOT always true
    temp_pred_equality(p, q);
    assert(p == q);
}

}
