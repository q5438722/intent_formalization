use vstd::prelude::*;

fn main() {}

verus!{

// === Definitions (copied from target) ===

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

pub open spec fn tla_exists<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// Axioms (from target)
#[verifier::external_body]
pub proof fn temp_pred_equality<T>(p: TempPred<T>, q: TempPred<T>)
    requires
        p.entails(q),
        q.entails(p),
    ensures p == q,
{ unimplemented!() }

#[verifier::external_body]
proof fn a_to_temp_pred_equality<T, A>(p: spec_fn(A) -> TempPred<T>, q: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #[trigger] p(a).entails(q(a)) && q(a).entails(p(a)),
    ensures p == q,
{ unimplemented!() }

#[verifier::external_body]
proof fn tla_forall_not_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| not(a_to_p(a))) == not(tla_exists(a_to_p)),
{ unimplemented!() }

#[verifier::external_body]
proof fn tla_forall_or_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    ensures tla_forall(|a: A| a_to_p(a).or(q)) == tla_forall(a_to_p).or(q),
{ unimplemented!() }


// === BEHAVIORAL MUTATION TESTS ===

// BMT1: Mutate the main theorem result: replace tla_exists with tla_forall on RHS.
// Original: tla_forall(|a| p(a).implies(q)) == tla_exists(p).implies(q)
// Mutated: tla_forall(|a| p(a).implies(q)) == tla_forall(p).implies(q)   <-- WRONG
// SHOULD FAIL
proof fn bmt1_forall_instead_of_exists_rhs()
{
    let a_to_p = |a: int| TempPred::<int>::new(
        |ex: Execution<int>| (ex.nat_to_state)(0) == a
    );
    let q = TempPred::<int>::new(|ex: Execution<int>| true);

    let lhs = tla_forall(|a: int| a_to_p(a).implies(q));
    let rhs = tla_forall(a_to_p).implies(q);  // WRONG: should be tla_exists

    assert(lhs == rhs); // SHOULD FAIL: forall(p).implies(q) != exists(p).implies(q) in general
}

// BMT2: Swap implies direction in the body.
// Original: tla_forall(|a| p(a).implies(q))
// Mutated:  tla_forall(|a| q.implies(p(a)))
// SHOULD FAIL
proof fn bmt2_swap_implies_direction()
{
    let a_to_p = |a: int| TempPred::<int>::new(
        |ex: Execution<int>| (ex.nat_to_state)(0) == a
    );
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);

    let original_lhs = tla_forall(|a: int| a_to_p(a).implies(q));
    let mutated_lhs  = tla_forall(|a: int| q.implies(a_to_p(a)));

    assert(original_lhs == mutated_lhs); // SHOULD FAIL: p=>q != q=>p in general
}

// BMT3: Replace tla_forall with tla_exists on the LHS.
// Original: tla_forall(|a| p(a).implies(q)) == tla_exists(p).implies(q)
// Mutated:  tla_exists(|a| p(a).implies(q)) == tla_exists(p).implies(q)   <-- WRONG
// SHOULD FAIL
proof fn bmt3_exists_instead_of_forall_lhs()
{
    let a_to_p = |a: int| TempPred::<int>::new(
        |ex: Execution<int>| (ex.nat_to_state)(0) == a
    );
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);

    let mutated_lhs = tla_exists(|a: int| a_to_p(a).implies(q));
    let rhs = tla_exists(a_to_p).implies(q);

    assert(mutated_lhs == rhs); // SHOULD FAIL
}

// BMT4: Mutate tla_forall_or_equality: replace forall with exists on RHS.
// Original: tla_forall(|a| p(a).or(q)) == tla_forall(p).or(q)
// Mutated:  tla_forall(|a| p(a).or(q)) == tla_exists(p).or(q)   <-- WRONG
// SHOULD FAIL
proof fn bmt4_mutate_or_equality_exists()
{
    let a_to_p = |a: int| TempPred::<int>::new(
        |ex: Execution<int>| (ex.nat_to_state)(0) == a
    );
    let q = TempPred::<int>::new(|ex: Execution<int>| false);

    let lhs = tla_forall(|a: int| a_to_p(a).or(q));
    let mutated_rhs = tla_exists(a_to_p).or(q);

    assert(lhs == mutated_rhs); // SHOULD FAIL: forall(p) != exists(p)
}

// BMT5: Mutate tla_forall_not_equality: swap forall/exists.
// Original: tla_forall(|a| not(p(a))) == not(tla_exists(p))
// Mutated:  tla_exists(|a| not(p(a))) == not(tla_exists(p))   <-- WRONG
// SHOULD FAIL
proof fn bmt5_mutate_demorgan()
{
    let a_to_p = |a: int| TempPred::<int>::new(
        |ex: Execution<int>| (ex.nat_to_state)(0) == a
    );

    let mutated_lhs = tla_exists(|a: int| not(a_to_p(a)));
    let rhs = not(tla_exists(a_to_p));

    assert(mutated_lhs == rhs); // SHOULD FAIL: ∃a.¬p(a) ≠ ¬∃a.p(a)
}

}
