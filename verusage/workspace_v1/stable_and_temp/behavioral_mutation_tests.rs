use vstd::prelude::*;

fn main() {}

verus!{

// ===== Type definitions (from target) =====

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
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn stable<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| temp_pred.implies(always(temp_pred)).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn stable_unfold<T>(ex: Execution<T>, p: TempPred<T>)
    requires stable(p).satisfied_by(ex),
    ensures p.satisfied_by(ex) ==> forall |i| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
}

// ===== Behavioral Mutation Tests =====

// Mutation Test 1: Strengthen conclusion from stable to always
// stable(p.and(q)) means "if p∧q holds now, it holds forever."
// always(p.and(q)) means "p∧q holds at every point, unconditionally."
// These are fundamentally different: stability is conditional, always is absolute.
// SHOULD FAIL
proof fn test_mutation_stable_to_always<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        valid(stable(p)),
        valid(stable(q)),
    ensures valid(always(p.and(q))),
{
}

// Mutation Test 2: Mutate combinator from conjunction to implication
// stable(p) ∧ stable(q) does NOT imply stable(p ⟹ q).
// Counterexample: p doesn't hold at ex, q doesn't hold at ex.
// p⟹q is vacuously true. But at suffix(i), p may hold and q may not.
// SHOULD FAIL
proof fn test_mutation_and_to_implies<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        valid(stable(p)),
        valid(stable(q)),
    ensures valid(stable(p.implies(q))),
{
}

// Mutation Test 3: Drop the implication guard in stable_unfold's postcondition
// stable_unfold gives: p(ex) ==> ∀i. p(ex.suffix(i))
// This test mutates it to: ∀i. p(ex.suffix(i)) unconditionally.
// Without knowing p(ex) holds, we cannot derive always(p).
// SHOULD FAIL
proof fn test_mutation_unfold_unconditional<T>(ex: Execution<T>, p: TempPred<T>)
    requires stable(p).satisfied_by(ex),
    ensures forall |i| p.satisfied_by(#[trigger] ex.suffix(i)),
{
    stable_unfold::<T>(ex, p);
}

}
