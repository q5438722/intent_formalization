use vstd::prelude::*;

fn main() {}

verus! {

// === Type definitions from target file ===

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

pub open spec fn tla_exists<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn tla_forall_unfold<T, A>(ex: Execution<T>, a_to_p: spec_fn(A) -> TempPred<T>)
    requires tla_forall(a_to_p).satisfied_by(ex),
    ensures forall |a| #[trigger] a_to_p(a).satisfied_by(ex),
{
    unimplemented!()
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

// === BOUNDARY TESTS ===

// Test 1: Call tla_forall_unfold on an execution where tla_forall is NOT satisfied.
// The precondition requires tla_forall(a_to_p).satisfied_by(ex), but here
// we use a predicate that is false for some 'a', so the forall doesn't hold.
// SHOULD FAIL
proof fn test_boundary_unfold_without_forall_satisfied(ex: Execution<int>)
    requires
        !tla_forall(|a: int| TempPred::<int>::new(|ex: Execution<int>| a > 0)).satisfied_by(ex),
{
    let a_to_p = |a: int| TempPred::<int>::new(|ex: Execution<int>| a > 0);
    tla_forall_unfold::<int, int>(ex, a_to_p);
}

// Test 2: Call temp_pred_equality with only one direction of entailment.
// Requires both p.entails(q) AND q.entails(p), but only p.entails(q) is given.
// SHOULD FAIL
proof fn test_boundary_equality_one_direction()
{
    let p = TempPred::<int>::new(|ex: Execution<int>| true);
    let q = TempPred::<int>::new(|ex: Execution<int>| false);
    // true ==> false is false, so p.entails(q) doesn't hold either,
    // but the point is we're calling without both preconditions.
    temp_pred_equality::<int>(p, q);
}

// Test 3: Call tla_forall_unfold with an exists-based predicate pretending it's a forall.
// The predicate is only existentially satisfied, not universally.
// SHOULD FAIL
proof fn test_boundary_unfold_with_exists_not_forall(ex: Execution<int>)
    requires
        tla_exists(|a: int| TempPred::<int>::new(|ex: Execution<int>| a == 42)).satisfied_by(ex),
        !tla_forall(|a: int| TempPred::<int>::new(|ex: Execution<int>| a == 42)).satisfied_by(ex),
{
    let a_to_p = |a: int| TempPred::<int>::new(|ex: Execution<int>| a == 42);
    tla_forall_unfold::<int, int>(ex, a_to_p);
}

}
