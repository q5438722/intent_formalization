use vstd::prelude::*;

fn main() {}

verus!{

// === Definitions ===

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

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// === Axioms ===

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{ unimplemented!() }

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{ unimplemented!() }

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{ unimplemented!() }

// === Boundary Tests ===

// Test 1: No precondition at all
// The conclusion should not hold for arbitrary spec, p, q without any constraint.
// SHOULD FAIL
proof fn test_no_precondition<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    ensures spec.entails(always(always(p).implies(always(q)))),
{
}

// Test 2: Missing 'always' wrapper in precondition
// Only knowing p => q at the top level (not at every suffix) is insufficient.
// SHOULD FAIL
proof fn test_missing_always_in_precondition<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(p.implies(q)),
    ensures spec.entails(always(always(p).implies(always(q)))),
{
}

// Test 3: Wrong predicate - only always(p), not always(p => q)
// Knowing p is always true says nothing about q.
// SHOULD FAIL
proof fn test_only_always_p<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p)),
    ensures spec.entails(always(always(p).implies(always(q)))),
{
}

// Test 4: Converse implication in precondition - always(q => p) instead of always(p => q)
// The converse direction does not establish the original conclusion.
// Counterexample: p = true, q = sometimes false. q => p is always true,
// but always(p) does not imply always(q).
// SHOULD FAIL
proof fn test_converse_precondition<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(q.implies(p))),
    ensures spec.entails(always(always(p).implies(always(q)))),
{
}

}
