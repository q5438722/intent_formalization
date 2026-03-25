use vstd::prelude::*;

fn main() {}

verus! {

// === Type definitions from target file ===

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

proof fn tla_forall_not_equality<T, A>(a_to_p: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| not(a_to_p(a))) == not(tla_exists(a_to_p)),
{
    let a_to_not_p = |a: A| not(a_to_p(a));
    assert forall |ex| #[trigger] tla_forall(a_to_not_p).satisfied_by(ex)
    implies not(tla_exists(a_to_p)).satisfied_by(ex) by {
        assert forall |a| !#[trigger] a_to_p(a).satisfied_by(ex) by {
            tla_forall_unfold::<T, A>(ex, a_to_not_p);
            assert(a_to_not_p(a).satisfied_by(ex));
        };
    };

    temp_pred_equality::<T>(tla_forall(|a: A| not(a_to_p(a))), not(tla_exists(a_to_p)));
}

// === BEHAVIORAL MUTATION TESTS ===

// Test 1: Mutate by dropping inner negation.
// Correct: tla_forall(|a| not(p(a))) == not(tla_exists(p))
// Mutated: tla_forall(p) == not(tla_exists(p))  [wrong: forall != not-exists without negation]
// SHOULD FAIL
proof fn test_mutation_drop_inner_negation(a_to_p: spec_fn(int) -> TempPred<int>)
{
    tla_forall_not_equality::<int, int>(a_to_p);
    assert(tla_forall(a_to_p) == not(tla_exists(a_to_p)));
}

// Test 2: Mutate by dropping outer negation.
// Correct: tla_forall(|a| not(p(a))) == not(tla_exists(p))
// Mutated: tla_forall(|a| not(p(a))) == tla_exists(p)  [flipped: missing outer not]
// SHOULD FAIL
proof fn test_mutation_drop_outer_negation(a_to_p: spec_fn(int) -> TempPred<int>)
{
    tla_forall_not_equality::<int, int>(a_to_p);
    assert(tla_forall(|a: int| not(a_to_p(a))) == tla_exists(a_to_p));
}

// Test 3: Mutate by swapping forall and exists.
// Correct: tla_forall(|a| not(p(a))) == not(tla_exists(p))
// Mutated: tla_exists(|a| not(p(a))) == not(tla_forall(p))  [the other De Morgan — not proven]
// SHOULD FAIL
proof fn test_mutation_swap_quantifiers(a_to_p: spec_fn(int) -> TempPred<int>)
{
    tla_forall_not_equality::<int, int>(a_to_p);
    assert(tla_exists(|a: int| not(a_to_p(a))) == not(tla_forall(a_to_p)));
}

}
