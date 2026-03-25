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

// ========== Behavioral Mutation Tests ==========

// Test 1: Valid precondition, but reverse the entailment direction in conclusion
// SHOULD FAIL
proof fn test_reversed_entailment<T>(spec: TempPred<T>, p: TempPred<T>)
    requires spec.entails(always(p)),
    ensures always(later(p)).entails(spec),
{
    always_to_always_later(spec, p);
}

// Test 2: Valid precondition, but conclude about unrelated predicate q
// SHOULD FAIL
proof fn test_wrong_predicate<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires spec.entails(always(p)),
    ensures spec.entails(always(later(q))),
{
    always_to_always_later(spec, p);
}

// Test 3: Claim later(p) entails always(p) — next step does NOT imply all steps
// SHOULD FAIL
proof fn test_later_implies_always<T>(p: TempPred<T>)
    ensures later(p).entails(always(p)),
{
}

// Test 4: Claim p entails always(p) — holding now does NOT mean holding forever
// SHOULD FAIL
proof fn test_p_implies_always_p<T>(p: TempPred<T>)
    ensures p.entails(always(p)),
{
}

}
