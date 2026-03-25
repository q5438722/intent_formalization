use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions from target file =====

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
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

pub open spec fn not<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| !temp_pred.satisfied_by(ex))
}

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
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

#[verifier::external_body]
proof fn a_to_temp_pred_equality<T, A>(p: spec_fn(A) -> TempPred<T>, q: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #[trigger] p(a).entails(q(a)) && q(a).entails(p(a)),
    ensures p == q,
{
    unimplemented!()
}

#[verifier::external_body]
proof fn tla_forall_or_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    ensures tla_forall(|a: A| a_to_p(a).or(q)) == tla_forall(a_to_p).or(q),
{
    unimplemented!()
}

proof fn tla_forall_implies_equality2<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| p.implies(a_to_q(a))) == p.implies(tla_forall(a_to_q)),
{
    a_to_temp_pred_equality::<T, A>(|a: A| p.implies(a_to_q(a)), |a: A| a_to_q(a).or(not(p)));
    temp_pred_equality::<T>(tla_forall(|a: A| p.implies(a_to_q(a))), tla_forall(|a: A| a_to_q(a).or(not(p))));
    tla_forall_or_equality::<T, A>(a_to_q, not(p));
    temp_pred_equality::<T>(tla_forall(a_to_q).or(not(p)), p.implies(tla_forall(a_to_q)));
}

// ===== Behavioral Mutation Tests =====

// SHOULD FAIL: Mutation — swap implies direction on the LHS.
// Correct: tla_forall(|a| p.implies(q(a))) == p.implies(tla_forall(q))
// Mutated: tla_forall(|a| q(a).implies(p)) == p.implies(tla_forall(q))
// ∀a.(Q(a)→P) ≠ P→(∀a.Q(a)) in general.
proof fn mutation_swap_implies_direction<A>(p: TempPred<int>, a_to_q: spec_fn(A) -> TempPred<int>)
    ensures tla_forall(|a: A| a_to_q(a).implies(p)) == p.implies(tla_forall(a_to_q)),
{
}

// SHOULD FAIL: Mutation — replace tla_forall on RHS with a single instantiation.
// Correct: ... == p.implies(tla_forall(a_to_q))
// Mutated: ... == p.implies(a_to_q(0))
// ∀a.(P→Q(a)) ≠ P→Q(0) in general.
proof fn mutation_forall_to_instance(p: TempPred<int>, a_to_q: spec_fn(int) -> TempPred<int>)
    ensures tla_forall(|a: int| p.implies(a_to_q(a))) == p.implies(a_to_q(0int)),
{
}

// SHOULD FAIL: Mutation — negate p on the RHS.
// Correct: ... == p.implies(tla_forall(a_to_q))
// Mutated: ... == not(p).implies(tla_forall(a_to_q))
// Replacing P with ¬P changes the semantics entirely.
proof fn mutation_negate_antecedent<A>(p: TempPred<int>, a_to_q: spec_fn(A) -> TempPred<int>)
    ensures tla_forall(|a: A| p.implies(a_to_q(a))) == not(p).implies(tla_forall(a_to_q)),
{
}

// SHOULD FAIL: Mutation — replace implies with or on the LHS.
// Correct: tla_forall(|a| p.implies(q(a))) == p.implies(tla_forall(q))
// Mutated: tla_forall(|a| p.or(q(a))) == p.implies(tla_forall(q))
// P∨Q(a) ≠ P→Q(a) in general, so ∀a.(P∨Q(a)) ≠ P→(∀a.Q(a))
proof fn mutation_implies_to_or<A>(p: TempPred<int>, a_to_q: spec_fn(A) -> TempPred<int>)
    ensures tla_forall(|a: A| p.or(a_to_q(a))) == p.implies(tla_forall(a_to_q)),
{
}

}
