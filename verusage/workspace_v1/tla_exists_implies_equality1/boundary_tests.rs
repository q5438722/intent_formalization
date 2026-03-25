use vstd::prelude::*;

fn main() {}

verus!{

// ==================== Definitions (from source) ====================

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

pub open spec fn tla_exists<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ==================== Axioms (from source) ====================

#[verifier::external_body]
pub proof fn temp_pred_equality<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        q.entails(p),
    ensures p == q,
{ unimplemented!() }

#[verifier::external_body]
proof fn a_to_temp_pred_equality<T, A>(p: spec_fn(A) -> TempPred<T>, q: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #[trigger] p(a).entails(q(a)) && q(a).entails(p(a)),
    ensures p == q,
{ unimplemented!() }

// ==================== BOUNDARY TESTS ====================

// Test 1: Call temp_pred_equality with always-true and always-false predicates.
// Neither p.entails(q) nor q.entails(p) when p=true, q=false:
//   p.entails(q) = valid(true ==> false) = forall ex, false = FALSE
// SHOULD FAIL
proof fn test_boundary_both_entailments_violated()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| true);
    let q = TempPred::<int>::new(|ex: Execution<int>| false);
    temp_pred_equality::<int>(p, q);
}

// Test 2: Call temp_pred_equality where only one direction holds.
// f.entails(t) = valid(false ==> true) = true  ✓
// t.entails(f) = valid(true ==> false) = false  ✗
// SHOULD FAIL
proof fn test_boundary_partial_entailment()
{
    let f = TempPred::<int>::new(|ex: Execution<int>| false);
    let t = TempPred::<int>::new(|ex: Execution<int>| true);
    temp_pred_equality::<int>(f, t);
}

// Test 3: Call a_to_temp_pred_equality with non-entailing function families.
// For each a: p(a)=always-true, q(a)=always-false.
// p(a).entails(q(a)) = valid(true ==> false) = FALSE
// SHOULD FAIL
proof fn test_boundary_a_to_equality_invalid()
{
    let p = |a: int| TempPred::<int>::new(|ex: Execution<int>| true);
    let q = |a: int| TempPred::<int>::new(|ex: Execution<int>| false);
    a_to_temp_pred_equality::<int, int>(p, q);
}

}
