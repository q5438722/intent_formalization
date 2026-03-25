use vstd::prelude::*;

fn main() {}

verus!{

// ========== Definitions from target file ==========

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

pub open spec fn later<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| temp_pred.satisfied_by(ex.suffix(1)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn always_propagate_forwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires always(p).satisfied_by(ex),
    ensures always(p).satisfied_by(ex.suffix(i)),
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

pub proof fn always_to_always_later<T>(spec: TempPred<T>, p: TempPred<T>)
    requires spec.entails(always(p)),
    ensures spec.entails(always(later(p))),
{
    assert forall |ex| #[trigger] always(p).satisfied_by(ex) implies always(later(p)).satisfied_by(ex) by {
        always_propagate_forwards(ex, p, 1);
        assert forall |i| #[trigger] later(p).satisfied_by(ex.suffix(i)) by {
            execution_equality(ex.suffix(i).suffix(1), ex.suffix(1).suffix(i));
        }
    }
    entails_trans(spec, always(p), always(later(p)));
}

#[verifier::external_body]
pub proof fn entails_trans<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        p.entails(q),
        q.entails(r),
    ensures p.entails(r),
{
    unimplemented!()
}

// ========== Boundary Tests ==========

// Test 1: Call always_to_always_later without establishing spec.entails(always(p))
// SHOULD FAIL
proof fn test_missing_entails_precondition<T>(spec: TempPred<T>, p: TempPred<T>)
    ensures spec.entails(always(later(p))),
{
    always_to_always_later(spec, p);
}

// Test 2: Call always_propagate_forwards without always(p).satisfied_by(ex)
// SHOULD FAIL
proof fn test_propagate_without_always<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    ensures always(p).satisfied_by(ex.suffix(i)),
{
    always_propagate_forwards(ex, p, i);
}

// Test 3: Call entails_trans with only one of two required preconditions
// SHOULD FAIL
proof fn test_entails_trans_partial_precondition<T>(p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires p.entails(q),
    ensures p.entails(r),
{
    entails_trans(p, q, r);
}

// Test 4: Call execution_equality without establishing pointwise equality
// SHOULD FAIL
proof fn test_execution_equality_no_evidence<T>(ex1: Execution<T>, ex2: Execution<T>)
    ensures ex1 == ex2,
{
    execution_equality(ex1, ex2);
}

}
