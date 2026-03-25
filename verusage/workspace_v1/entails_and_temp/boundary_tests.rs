use vstd::prelude::*;

fn main() {}

verus!{

// === Type Definitions (from target) ===

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
    pub open spec fn and(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) && other.satisfied_by(ex))
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

pub proof fn entails_and_temp<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(p),
        spec.entails(q),
    ensures spec.entails(p.and(q)),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies p.and(q).satisfied_by(ex) by {
        implies_apply::<T>(ex, spec, p);
        implies_apply::<T>(ex, spec, q);
    };
}

// ============================================================
// BOUNDARY TESTS — Violate preconditions of entails_and_temp
// ============================================================

// Test 1: Missing first precondition spec.entails(p)
// Only spec.entails(q) is provided; spec.entails(p) is absent.
// SHOULD FAIL
proof fn boundary_missing_first_precondition(
    s: TempPred<int>, p: TempPred<int>, q: TempPred<int>
)
    requires
        s.entails(q),
{
    entails_and_temp::<int>(s, p, q);
}

// Test 2: Missing second precondition spec.entails(q)
// Only spec.entails(p) is provided; spec.entails(q) is absent.
// SHOULD FAIL
proof fn boundary_missing_second_precondition(
    s: TempPred<int>, p: TempPred<int>, q: TempPred<int>
)
    requires
        s.entails(p),
{
    entails_and_temp::<int>(s, p, q);
}

// Test 3: No preconditions at all — completely unconstrained call
// SHOULD FAIL
proof fn boundary_no_preconditions(
    s: TempPred<int>, p: TempPred<int>, q: TempPred<int>
)
{
    entails_and_temp::<int>(s, p, q);
}

// Test 4: implies_apply called without the implication precondition
// p.satisfied_by(ex) holds but p.implies(q).satisfied_by(ex) is missing.
// SHOULD FAIL
proof fn boundary_implies_apply_missing_implication(
    ex: Execution<int>, p: TempPred<int>, q: TempPred<int>
)
    requires
        p.satisfied_by(ex),
{
    implies_apply::<int>(ex, p, q);
}

}
