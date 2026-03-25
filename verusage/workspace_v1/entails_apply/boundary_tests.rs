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

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    unimplemented!()
}

proof fn entails_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply::<T>(ex, p, q);
}

// ===== Boundary Tests =====

// SHOULD FAIL: Call entails_apply without p.entails(q) (missing first precondition)
proof fn boundary_test_missing_entails<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.satisfied_by(ex),
        // NOTE: p.entails(q) is NOT required
    ensures q.satisfied_by(ex),
{
    entails_apply::<T>(ex, p, q);
}

// SHOULD FAIL: Call entails_apply without p.satisfied_by(ex) (missing second precondition)
proof fn boundary_test_missing_satisfaction<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        // NOTE: p.satisfied_by(ex) is NOT required
    ensures q.satisfied_by(ex),
{
    entails_apply::<T>(ex, p, q);
}

// SHOULD FAIL: Call implies_apply without p.implies(q).satisfied_by(ex)
proof fn boundary_test_implies_missing_implication<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.satisfied_by(ex),
        // NOTE: p.implies(q).satisfied_by(ex) is NOT required
    ensures q.satisfied_by(ex),
{
    implies_apply::<T>(ex, p, q);
}

// SHOULD FAIL: Call implies_apply without p.satisfied_by(ex)
proof fn boundary_test_implies_missing_satisfaction<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        // NOTE: p.satisfied_by(ex) is NOT required
    ensures q.satisfied_by(ex),
{
    implies_apply::<T>(ex, p, q);
}

// SHOULD FAIL: Use a predicate that is never satisfied (always false) and try to derive q
proof fn boundary_test_unsatisfiable_predicate<T>(ex: Execution<T>, q: TempPred<T>)
    requires
        // p is always false, so p.entails(q) is vacuously true, but p.satisfied_by(ex) is false
        !TempPred::new(|e: Execution<T>| false).satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    let p = TempPred::<T>::new(|e: Execution<T>| false);
    entails_apply::<T>(ex, p, q);
}

}
