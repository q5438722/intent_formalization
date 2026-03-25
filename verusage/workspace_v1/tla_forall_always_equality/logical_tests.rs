use vstd::prelude::*;

fn main() {}

verus!{

// === Definitions (from target file) ===

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

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
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

// === Logical Tests ===
// These test properties NOT explicitly guaranteed by the specification.
// Each SHOULD FAIL verification.

// SHOULD FAIL: Universal validity from a single execution.
// always(p) holding for one execution does NOT imply valid(always(p)) (all executions).
proof fn test_logical_valid_from_single_execution(p: TempPred<int>, ex: Execution<int>)
    requires always(p).satisfied_by(ex),
    ensures valid(always(p)),
{
}

// SHOULD FAIL: always does NOT distribute over disjunction.
// always(p ∨ q) does NOT entail always(p) ∨ always(q).
// Counterexample: p true at even steps, q true at odd steps.
proof fn test_logical_always_distributes_over_or(p: TempPred<int>, q: TempPred<int>, ex: Execution<int>)
    requires always(TempPred::<int>::new(|e: Execution<int>| p.satisfied_by(e) || q.satisfied_by(e))).satisfied_by(ex),
    ensures always(p).satisfied_by(ex) || always(q).satisfied_by(ex),
{
}

// SHOULD FAIL: tla_forall does NOT promote to always.
// Knowing for all a, p(a) holds at the current execution does NOT mean
// for all a, p(a) holds at all future times.
proof fn test_logical_tla_forall_promotes_to_always(a_to_p: spec_fn(int) -> TempPred<int>, ex: Execution<int>)
    requires tla_forall(a_to_p).satisfied_by(ex),
    ensures tla_forall(|a: int| always(a_to_p(a))).satisfied_by(ex),
{
}

}
