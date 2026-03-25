use vstd::prelude::*;

fn main() {}

verus! {

// ==================== Definitions (from target) ====================

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

pub proof fn entails_trans<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures p.entails(r),
{
    assert forall |ex| p.satisfied_by(ex) implies r.satisfied_by(ex) by {
        implies_apply::<T>(ex, p, q);
        implies_apply::<T>(ex, q, r);
    };
}

// ==================== Boundary Tests ====================

// Test 1: Call entails_trans missing first precondition p.entails(q)
// SHOULD FAIL
proof fn boundary_missing_first_precondition<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        q.entails(r),
    ensures p.entails(r),
{
    entails_trans(p, q, r);
}

// Test 2: Call entails_trans missing second precondition q.entails(r)
// SHOULD FAIL
proof fn boundary_missing_second_precondition<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
    ensures p.entails(r),
{
    entails_trans(p, q, r);
}

// Test 3: Call entails_trans with no preconditions at all
// SHOULD FAIL
proof fn boundary_no_preconditions<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    ensures p.entails(r),
{
    entails_trans(p, q, r);
}

// Test 4: Call implies_apply without the implication precondition
// SHOULD FAIL
proof fn boundary_implies_apply_missing_implication<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

// Test 5: Call implies_apply without the satisfaction precondition
// SHOULD FAIL
proof fn boundary_implies_apply_missing_satisfaction<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

}
