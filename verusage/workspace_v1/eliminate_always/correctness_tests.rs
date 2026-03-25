use vstd::prelude::*;

fn main() {}

verus!{

// ===== Definitions from target file: eliminate_always.rs =====

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

// ============================================================
// BOUNDARY TESTS — violate preconditions
// ============================================================

// SHOULD FAIL: eliminate_always called without its precondition.
proof fn test_boundary_eliminate_always_no_precondition<T>(spec: TempPred<T>, p: TempPred<T>)
    ensures spec.entails(p),
{
    eliminate_always(spec, p);
}

// SHOULD FAIL: implies_apply called without p.satisfied_by(ex).
proof fn test_boundary_implies_apply_missing_antecedent<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.implies(q).satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    implies_apply(ex, p, q);
}

// SHOULD FAIL: execution_equality called without extensional equality.
proof fn test_boundary_execution_equality_no_precondition<T>(ex1: Execution<T>, ex2: Execution<T>)
    ensures ex1 == ex2,
{
    execution_equality(ex1, ex2);
}

// ============================================================
// BEHAVIORAL MUTATION TESTS — mutated outputs from valid inputs
// ============================================================

// SHOULD FAIL: Reverse direction — spec.entails(p) does NOT imply spec.entails(always(p)).
proof fn test_mutation_reverse_eliminate_always<T>(spec: TempPred<T>, p: TempPred<T>)
    requires spec.entails(p),
    ensures spec.entails(always(p)),
{
}

// SHOULD FAIL: Strengthened output — spec.entails(always(p)) does NOT imply valid(p).
proof fn test_mutation_entails_always_to_valid<T>(spec: TempPred<T>, p: TempPred<T>)
    requires spec.entails(always(p)),
    ensures valid(p),
{
    eliminate_always(spec, p);
}

// SHOULD FAIL: Wrong predicate — spec.entails(always(p)) does NOT imply spec.entails(q).
proof fn test_mutation_wrong_predicate<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p)),
    ensures spec.entails(q),
{
    eliminate_always(spec, p);
}

// ============================================================
// LOGICAL TESTS — unintended reasoning / structural assumptions
// ============================================================

// SHOULD FAIL: Entails is NOT symmetric — spec.entails(p) does NOT imply p.entails(spec).
proof fn test_logical_entails_symmetry<T>(spec: TempPred<T>, p: TempPred<T>)
    requires spec.entails(p),
    ensures p.entails(spec),
{
}

// SHOULD FAIL: Satisfaction does NOT transfer between arbitrary executions.
proof fn test_logical_satisfaction_transfer<T>(ex1: Execution<T>, ex2: Execution<T>, p: TempPred<T>)
    requires p.satisfied_by(ex1),
    ensures p.satisfied_by(ex2),
{
}

// SHOULD FAIL: Two executions satisfying the same spec are NOT necessarily equal (no determinism).
proof fn test_logical_determinism<T>(spec: TempPred<T>, ex1: Execution<T>, ex2: Execution<T>)
    requires
        spec.satisfied_by(ex1),
        spec.satisfied_by(ex2),
    ensures ex1 == ex2,
{
}

}
