use vstd::prelude::*;

fn main() {}

verus! {

// ========== Definitions (from target) ==========

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
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
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

proof fn always_double_equality<T>(p: TempPred<T>)
    ensures always(always(p)) == always(p),
{
    assert forall |ex| #[trigger] always(p).satisfied_by(ex) implies always(always(p)).satisfied_by(ex) by {
        assert forall |i| #[trigger] always(p).satisfied_by(ex.suffix(i)) by {
            assert forall |j| #[trigger] p.satisfied_by(ex.suffix(i).suffix(j)) by {
                execution_equality(ex.suffix(i).suffix(j), ex.suffix(i + j));
                assert(p.satisfied_by(ex.suffix(i + j)));
            }
        }
    }
    assert forall |ex| #[trigger] always(always(p)).satisfied_by(ex) implies always(p).satisfied_by(ex) by {
        execution_equality(ex.suffix(0), ex);
        assert(always(p).satisfied_by(ex.suffix(0)));
    }
    temp_pred_equality::<T>(always(always(p)), always(p));
}

// ========== Behavioral Mutation Tests ==========

// SHOULD FAIL
// Test 1: Mutate the theorem to over-collapse: always(always(p)) == p.
// The correct result is always(p), not p. always(p) is strictly stronger than p.
proof fn test_always_over_collapse<T>(p: TempPred<T>)
    ensures always(always(p)) == p,
{
    always_double_equality(p);
    // We now know always(always(p)) == always(p), but NOT always(p) == p.
}

// SHOULD FAIL
// Test 2: Mutate the always operator to be the identity: always(p) == p.
// always(p) requires p to hold at ALL suffixes, not just the current state.
proof fn test_always_is_identity<T>(p: TempPred<T>)
    ensures always(p) == p,
{
}

// SHOULD FAIL
// Test 3: Mutate to claim always ignores its argument: always(p) == always(q).
// Two different predicates under always should remain distinguishable.
proof fn test_always_ignores_argument<T>(p: TempPred<T>, q: TempPred<T>)
    ensures always(p) == always(q),
{
}

}
