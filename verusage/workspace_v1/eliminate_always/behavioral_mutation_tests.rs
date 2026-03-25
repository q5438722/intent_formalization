use vstd::prelude::*;

fn main() {}

verus!{

// ===== Definitions from target file =====

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

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{
    unimplemented!()
}

pub proof fn eliminate_always<T>(spec: TempPred<T>, p: TempPred<T>)
    requires spec.entails(always(p)),
    ensures spec.entails(p),
{
    assert forall |ex| spec.satisfied_by(ex) implies #[trigger] p.satisfied_by(ex) by {
        implies_apply(ex, spec, always(p));
        execution_equality(ex, ex.suffix(0));
    }
}

// ===== Behavioral Mutation Tests =====
// These tests start from valid inputs but assert mutated/incorrect outputs.
// All should FAIL verification.

// SHOULD FAIL: Reverse direction — entailing p does NOT imply entailing always(p).
proof fn test_mutation_reverse_eliminate_always<T>(spec: TempPred<T>, p: TempPred<T>)
    requires spec.entails(p),
    ensures spec.entails(always(p)),
{
}

// SHOULD FAIL: Strengthened output — entailing always(p) does NOT imply p is universally valid.
proof fn test_mutation_entails_always_to_valid<T>(spec: TempPred<T>, p: TempPred<T>)
    requires spec.entails(always(p)),
    ensures valid(p),
{
    eliminate_always(spec, p);
}

// SHOULD FAIL: Wrong predicate — entailing always(p) does NOT imply entailing arbitrary q.
proof fn test_mutation_wrong_predicate<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p)),
    ensures spec.entails(q),
{
    eliminate_always(spec, p);
}

}
