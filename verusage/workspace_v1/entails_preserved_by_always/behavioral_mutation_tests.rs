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

// ========== Axioms (from target) ==========

#[verifier::external_body]
proof fn always_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires always(p).satisfied_by(ex),
    ensures forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
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

// ========== Behavioral Mutation Tests ==========

// SHOULD FAIL: Reversed conclusion — always(q) entails always(p) instead of always(p) entails always(q)
// Mutates the direction of the conclusion. p.entails(q) does NOT imply q entails p.
proof fn test_reversed_conclusion<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures always(q).entails(always(p)),
{
    assert forall |ex| always(q).satisfied_by(ex) implies always(p).satisfied_by(ex) by {
        assert forall |i: nat| p.satisfied_by(#[trigger] ex.suffix(i)) by {
            always_unfold::<T>(ex, q);
            implies_apply::<T>(ex.suffix(i), q, p);
        };
    };
}

// SHOULD FAIL: Strengthened conclusion — p entails always(q)
// p holding at one execution does NOT mean q holds at all suffixes.
proof fn test_strengthened_to_always<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures p.entails(always(q)),
{
}

// SHOULD FAIL: Mutated output — valid(always(q)) from p.entails(q)
// Entailment is conditional; it does NOT imply q is universally valid.
proof fn test_valid_from_entails<T>(p: TempPred<T>, q: TempPred<T>)
    requires p.entails(q),
    ensures valid(always(q)),
{
}

}
