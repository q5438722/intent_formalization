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

// === Behavioral Mutation Tests ===
// These tests mutate expected outputs or relations from the main theorem.
// Each SHOULD FAIL verification.

// SHOULD FAIL: Mutated theorem — dropped inner "always" from tla_forall argument.
// The real theorem says tla_forall(|a| always(a_to_p(a))) == always(tla_forall(a_to_p)).
// This mutant claims tla_forall(a_to_p) == always(tla_forall(a_to_p)), which is wrong:
// tla_forall without inner always is strictly weaker than always(tla_forall).
proof fn test_mutation_dropped_inner_always(a_to_p: spec_fn(int) -> TempPred<int>)
    ensures tla_forall(a_to_p) == always(tla_forall(a_to_p)),
{
}

// SHOULD FAIL: Wrong entailment direction — claims p entails always(p).
// Knowing that p holds at a given execution does NOT mean p holds at all future suffixes.
proof fn test_mutation_p_entails_always_p(p: TempPred<int>)
    ensures p.entails(always(p)),
{
}

// SHOULD FAIL: Negation of the main theorem — claims inequality instead of equality.
// The original theorem proves these are equal; claiming they differ should be unprovable.
proof fn test_mutation_negated_equality(a_to_p: spec_fn(int) -> TempPred<int>)
    ensures tla_forall(|a: int| always(a_to_p(a))) != always(tla_forall(a_to_p)),
{
}

}
