use vstd::prelude::*;

fn main() {}

verus! {

// === Definitions (copied from target) ===

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

// === Axioms (external_body functions from target) ===

#[verifier::external_body]
pub proof fn tla_forall_always_equality_variant<T, A>(a_to_always: spec_fn(A) -> TempPred<T>, a_to_p: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #![trigger a_to_always(a)] a_to_always(a).entails((|a: A| always(a_to_p(a)))(a)) && ((|a: A| always(a_to_p(a)))(a)).entails(a_to_always(a)),
    ensures tla_forall(a_to_always) == always(tla_forall(a_to_p)),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn tla_forall_implies_equality2<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| p.implies(a_to_q(a))) == p.implies(tla_forall(a_to_q)),
{
    unimplemented!()
}

pub proof fn tla_forall_always_implies_equality2<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| always(p.implies(a_to_q(a)))) == always(p.implies(tla_forall(a_to_q))),
{
    tla_forall_always_equality_variant::<T, A>(|a: A| always(p.implies(a_to_q(a))), |a: A| p.implies(a_to_q(a)));
    tla_forall_implies_equality2::<T, A>(p, a_to_q);
}

// === Behavioral Mutation Tests ===

// Test M1: Assert inequality where equality is stated (negation of postcondition)
// The main function ensures LHS == RHS; asserting LHS != RHS directly contradicts it
// SHOULD FAIL
proof fn test_mutation_assert_inequality<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| always(p.implies(a_to_q(a)))) != always(p.implies(tla_forall(a_to_q))),
{
    tla_forall_always_implies_equality2::<T, A>(p, a_to_q);
}

// Test M2: Drop always from LHS of the equality
// Original: tla_forall(|a| always(p→q(a))) == always(p→∀a.q(a))
// Mutated:  tla_forall(|a| p→q(a)) == always(p→∀a.q(a))
// LHS without always is strictly weaker; equality should not hold
// SHOULD FAIL
proof fn test_mutation_drop_always_lhs<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| p.implies(a_to_q(a))) == always(p.implies(tla_forall(a_to_q))),
{
    tla_forall_always_implies_equality2::<T, A>(p, a_to_q);
    tla_forall_implies_equality2::<T, A>(p, a_to_q);
}

// Test M3: Swap implication direction on LHS
// Original: tla_forall(|a| always(p→q(a))) == always(p→∀a.q(a))
// Mutated:  tla_forall(|a| always(q(a)→p)) == always(p→∀a.q(a))
// Reversing the implication changes semantics
// SHOULD FAIL
proof fn test_mutation_swap_implies_direction<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| always(a_to_q(a).implies(p))) == always(p.implies(tla_forall(a_to_q))),
{
    tla_forall_always_implies_equality2::<T, A>(p, a_to_q);
}

// Test M4: Replace tla_forall(a_to_q) with a_to_q(specific_a) on RHS
// Original: ... == always(p→∀a.q(a))
// Mutated:  ... == always(p→q(specific_a))
// Replacing universal quantification with a single witness weakens the RHS
// SHOULD FAIL
proof fn test_mutation_specific_instead_of_forall<T>(p: TempPred<T>, a_to_q: spec_fn(int) -> TempPred<T>, specific_a: int)
    ensures tla_forall(|a: int| always(p.implies(a_to_q(a)))) == always(p.implies(a_to_q(specific_a))),
{
    tla_forall_always_implies_equality2::<T, int>(p, a_to_q);
}

}
