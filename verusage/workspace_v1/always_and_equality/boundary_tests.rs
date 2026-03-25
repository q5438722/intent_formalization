use vstd::prelude::*;

fn main() {}

verus!{

// ===== Base definitions from target file =====

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

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
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
pub proof fn temp_pred_equality<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        q.entails(p),
    ensures p == q,
{
    unimplemented!()
}

// ===== BOUNDARY TESTS =====
// These tests violate preconditions of the axioms.
// All should FAIL verification.

// SHOULD FAIL: calling always_unfold without establishing always(p).satisfied_by(ex)
proof fn boundary_test_1_unfold_without_precondition<T>(ex: Execution<T>, p: TempPred<T>)
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    always_unfold::<T>(ex, p);
}

// SHOULD FAIL: calling temp_pred_equality with only one direction of entailment
proof fn boundary_test_2_equality_one_direction<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures p == q,
{
    temp_pred_equality::<T>(p, q);
}

// SHOULD FAIL: using always_unfold to conclude about a DIFFERENT predicate q
proof fn boundary_test_3_unfold_wrong_predicate<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| q.satisfied_by(#[trigger] ex.suffix(i)),
{
    always_unfold::<T>(ex, p);
}

}
