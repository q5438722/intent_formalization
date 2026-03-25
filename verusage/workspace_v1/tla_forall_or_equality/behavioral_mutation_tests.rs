use vstd::prelude::*;

fn main() {}

verus!{

// === Shared Definitions (from target file) ===

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

#[verifier(reject_recursive_types(T))]
pub struct TempPred<T> {
    pub pred: spec_fn(Execution<T>) -> bool,
}

impl<T> TempPred<T> {

    pub open spec fn new(pred: spec_fn(Execution<T>) -> bool) -> Self {
        TempPred {
            pred: pred,
        }
    }

    pub open spec fn satisfied_by(self, execution: Execution<T>) -> bool {
        (self.pred)(execution)
    }

    pub open spec fn or(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) || other.satisfied_by(ex))
    }

    pub open spec fn implies(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) ==> other.satisfied_by(ex))
    }

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }

}


pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
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

proof fn tla_forall_or_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    ensures tla_forall(|a: A| a_to_p(a).or(q)) == tla_forall(a_to_p).or(q),
{
    let a_to_p_or_q = |a: A| a_to_p(a).or(q);
    assert forall |ex| #[trigger] tla_forall(a_to_p_or_q).satisfied_by(ex)
    implies (tla_forall(a_to_p).or(q)).satisfied_by(ex) by {
        tla_forall_unfold::<T, A>(ex, a_to_p_or_q);
        if !q.satisfied_by(ex) {
            assert forall |a| #[trigger] a_to_p(a).satisfied_by(ex) by {
                assert(a_to_p_or_q(a).satisfied_by(ex));
            };
        }
    };

    temp_pred_equality::<T>(tla_forall(|a: A| a_to_p(a).or(q)), tla_forall(a_to_p).or(q));
}

// === Behavioral Mutation Tests ===

// Test 4: Mutate the theorem's postcondition by dropping .or(q) from the RHS.
// Correct: tla_forall(|a| P(a).or(Q)) == tla_forall(P).or(Q)
// Mutated: tla_forall(|a| P(a).or(Q)) == tla_forall(P)
// SHOULD FAIL
proof fn test_drop_or_from_rhs(a_to_p: spec_fn(int) -> TempPred<int>, q: TempPred<int>)
{
    tla_forall_or_equality::<int, int>(a_to_p, q);
    assert(tla_forall(|a: int| a_to_p(a).or(q)) == tla_forall(a_to_p));
}

// Test 5: Mutate the operator: replace .or with .implies in the RHS.
// Correct: tla_forall(|a| P(a).or(Q)) entails tla_forall(P).or(Q)
// Mutated: tla_forall(|a| P(a).or(Q)) entails tla_forall(P).implies(Q)
// Counterexample: all P(a) true, Q false => LHS true, RHS false.
// SHOULD FAIL
proof fn test_swap_or_to_implies(a_to_p: spec_fn(int) -> TempPred<int>, q: TempPred<int>)
{
    assert(tla_forall(|a: int| a_to_p(a).or(q)).entails(tla_forall(a_to_p).implies(q)));
}

// Test 6: Mutate by strengthening: assert the disjunction entails one disjunct.
// Correct: (forall P(a)) or Q  is a disjunction
// Mutated: (forall P(a)) or Q  entails  (forall P(a))  alone
// Counterexample: all P(a) false, Q true => LHS true, RHS false.
// SHOULD FAIL
proof fn test_or_entails_left_disjunct(a_to_p: spec_fn(int) -> TempPred<int>, q: TempPred<int>)
{
    assert(tla_forall(a_to_p).or(q).entails(tla_forall(a_to_p)));
}

}
