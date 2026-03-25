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

// ===== BEHAVIORAL MUTATION TESTS =====
// These tests mutate the correct always_and_equality result.
// All should FAIL verification.

// SHOULD FAIL: always(p.and(q)) == always(p) — drops q from conjunction
proof fn mutation_test_1_drop_q<T>(p: TempPred<T>, q: TempPred<T>)
    ensures always(p.and(q)) == always(p),
{
    assert forall |ex| #[trigger] always(p.and(q)).satisfied_by(ex) implies always(p).satisfied_by(ex) by {
        assert forall |i| #[trigger] p.satisfied_by(ex.suffix(i)) by {
            always_unfold::<T>(ex, p.and(q));
        }
    };
    temp_pred_equality::<T>(always(p.and(q)), always(p));
}

// SHOULD FAIL: always(p.and(q)) == always(q) — drops p from conjunction
proof fn mutation_test_2_drop_p<T>(p: TempPred<T>, q: TempPred<T>)
    ensures always(p.and(q)) == always(q),
{
    assert forall |ex| #[trigger] always(p.and(q)).satisfied_by(ex) implies always(q).satisfied_by(ex) by {
        assert forall |i| #[trigger] q.satisfied_by(ex.suffix(i)) by {
            always_unfold::<T>(ex, p.and(q));
        }
    };
    temp_pred_equality::<T>(always(p.and(q)), always(q));
}

// SHOULD FAIL: always(p).and(always(q)) == always(p) — drops always(q) from conjunction
proof fn mutation_test_3_weaken_conjunction<T>(p: TempPred<T>, q: TempPred<T>)
    ensures always(p).and(always(q)) == always(p),
{
    temp_pred_equality::<T>(always(p).and(always(q)), always(p));
}

}
