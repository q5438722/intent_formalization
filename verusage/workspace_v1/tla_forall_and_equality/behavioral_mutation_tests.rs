use vstd::prelude::*;

fn main() {}

verus! {

// === Type definitions (from target file) ===

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

pub open spec fn tla_forall<T, A>(a_to_temp_pred: spec_fn(A) -> TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |a: A| #[trigger] a_to_temp_pred(a).satisfied_by(ex))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// === Axioms (external_body from target file) ===

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

// === BEHAVIORAL MUTATION TESTS ===

// Test 1: Mutated postcondition - drops q from the conjunction.
// Correct: tla_forall(|a| p(a).and(q)) == tla_forall(p).and(q)
// Mutation: tla_forall(p) entails tla_forall(|a| p(a).and(q)) — claims q is unnecessary.
// With p=always_true and q=always_false:
//   tla_forall(p) = always_true, tla_forall(|a| p(a).and(q)) = always_false.
//   always_true entails always_false is false.
// SHOULD FAIL
proof fn test_mutation_drops_q()
{
    let always_true = TempPred::<int>::new(|ex: Execution<int>| true);
    let always_false = TempPred::<int>::new(|ex: Execution<int>| false);
    let a_to_p = |a: int| always_true;
    let q = always_false;
    assert(tla_forall(a_to_p).entails(tla_forall(|a: int| a_to_p(a).and(q))));
}

// Test 2: Mutated postcondition - drops tla_forall, claims q alone suffices.
// Correct: tla_forall(|a| p(a).and(q)) == tla_forall(p).and(q)
// Mutation: q entails tla_forall(|a| p(a).and(q)) — claims the forall part is unnecessary.
// With p(a)=always_false: tla_forall(|a| false.and(q)) = always_false.
// Since q is arbitrary, q.entails(always_false) cannot be proven.
// SHOULD FAIL
proof fn test_mutation_drops_forall(q: TempPred<int>)
{
    let a_to_p = |a: int| TempPred::<int>::new(|ex: Execution<int>| false);
    assert(q.entails(tla_forall(|a: int| a_to_p(a).and(q))));
}

// Test 3: Wrong connective — claims implies distributes through tla_forall like and.
// Correct: tla_forall(|a| p(a).and(q)) == tla_forall(p).and(q)
// Mutation: tla_forall(p).implies(q) entails tla_forall(|a| p(a).implies(q))
// With p(a) = (a == 0), q = always_false:
//   tla_forall(p).implies(q) = (forall a: a==0).implies(false) = false.implies(false) = true (always)
//   tla_forall(|a| p(a).implies(q)) = forall a: (a==0)==>false = forall a: a!=0 (false when a=0)
//   So always_true entails always_false — false.
// SHOULD FAIL
proof fn test_mutation_wrong_connective()
{
    let a_to_p = |a: int| TempPred::<int>::new(|ex: Execution<int>| a == 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| false);
    assert(
        tla_forall(a_to_p).implies(q).entails(
            tla_forall(|a: int| a_to_p(a).implies(q))
        )
    );
}

}
