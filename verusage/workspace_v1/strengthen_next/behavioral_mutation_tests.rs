use vstd::prelude::*;

fn main() {}

verus!{

// ===== Definitions (from source) =====

pub type StatePred<T> = spec_fn(T) -> bool;
pub type ActionPred<T> = spec_fn(T, T) -> bool;

pub struct Execution<T> {
    pub nat_to_state: spec_fn(nat) -> T,
}

impl<T> Execution<T> {
    pub open spec fn head(self) -> T {
        (self.nat_to_state)(0)
    }
    pub open spec fn head_next(self) -> T {
        (self.nat_to_state)(1)
    }
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
    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn lift_state<T>(state_pred: StatePred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| state_pred(ex.head()))
}

pub open spec fn lift_action<T>(action_pred: ActionPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| action_pred(ex.head(), ex.head_next()))
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
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
{ unimplemented!() }

#[verifier::external_body]
pub proof fn always_and_equality<T>(p: TempPred<T>, q: TempPred<T>)
    ensures always(p.and(q)) == always(p).and(always(q)),
{ unimplemented!() }

#[verifier::external_body]
pub proof fn entails_and_temp<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        spec.entails(p),
        spec.entails(q),
    ensures spec.entails(p.and(q)),
{ unimplemented!() }

pub proof fn strengthen_next<T>(spec: TempPred<T>, next: ActionPred<T>, inv: StatePred<T>, next_and_inv: ActionPred<T>)
    requires
        spec.entails(always(lift_action(next))),
        spec.entails(always(lift_state(inv))),
        lift_action(next_and_inv).entails(lift_action(next).and(lift_state(inv))),
        lift_action(next).and(lift_state(inv)).entails(lift_action(next_and_inv)),
    ensures spec.entails(always(lift_action(next_and_inv))),
{
    entails_and_temp::<T>(spec, always(lift_action(next)), always(lift_state(inv)));
    always_and_equality::<T>(lift_action(next), lift_state(inv));
    temp_pred_equality::<T>(lift_action(next_and_inv), lift_action(next).and(lift_state(inv)));
}

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Mutated conclusion — assert valid() instead of spec.entails()
// After correctly calling strengthen_next, the conclusion is spec.entails(always(...))
// Mutation: upgrade to valid(always(...)), which is strictly stronger
// SHOULD FAIL
proof fn test_mutation_valid_instead_of_entails()
{
    let spec: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let next: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1;
    let inv: StatePred<int> = |s: int| s >= 0;
    let next_and_inv: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1 && s1 >= 0;

    assume(spec.entails(always(lift_action(next))));
    assume(spec.entails(always(lift_state(inv))));
    assume(lift_action(next_and_inv).entails(lift_action(next).and(lift_state(inv))));
    assume(lift_action(next).and(lift_state(inv)).entails(lift_action(next_and_inv)));

    strengthen_next(spec, next, inv, next_and_inv);

    // Mutated: valid is stronger than entails — NOT guaranteed
    assert(valid(always(lift_action(next_and_inv))));
}

// Test 2: Mutated conclusion — assert spec entails the NEGATION of next_and_inv
// After strengthen_next proves spec ⊢ □(next_and_inv), assert spec ⊢ □(¬next_and_inv)
// SHOULD FAIL
proof fn test_mutation_negated_conclusion()
{
    let spec: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let next: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1;
    let inv: StatePred<int> = |s: int| s >= 0;
    let next_and_inv: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1 && s1 >= 0;
    let not_next_and_inv: ActionPred<int> = |s1: int, s2: int| !(s2 == s1 + 1 && s1 >= 0);

    assume(spec.entails(always(lift_action(next))));
    assume(spec.entails(always(lift_state(inv))));
    assume(lift_action(next_and_inv).entails(lift_action(next).and(lift_state(inv))));
    assume(lift_action(next).and(lift_state(inv)).entails(lift_action(next_and_inv)));

    strengthen_next(spec, next, inv, next_and_inv);

    // Mutated: assert the negation of the proven conclusion
    assert(spec.entails(always(lift_action(not_next_and_inv))));
}

// Test 3: Mutated conclusion — assert spec entails always of a DIFFERENT predicate
// After correct setup, claim spec ⊢ □(wrong_pred) for an unrelated predicate
// SHOULD FAIL
proof fn test_mutation_wrong_predicate_in_conclusion()
{
    let spec: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let next: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1;
    let inv: StatePred<int> = |s: int| s >= 0;
    let next_and_inv: ActionPred<int> = |s1: int, s2: int| s2 == s1 + 1 && s1 >= 0;
    let wrong_pred: ActionPred<int> = |s1: int, s2: int| s2 == s1 * 2;

    assume(spec.entails(always(lift_action(next))));
    assume(spec.entails(always(lift_state(inv))));
    assume(lift_action(next_and_inv).entails(lift_action(next).and(lift_state(inv))));
    assume(lift_action(next).and(lift_state(inv)).entails(lift_action(next_and_inv)));

    strengthen_next(spec, next, inv, next_and_inv);

    // Mutated: wrong predicate — not entailed by spec
    assert(spec.entails(always(lift_action(wrong_pred))));
}

// Test 4: Assert non-equivalent temporal predicates are equal
// Two TempPreds with different semantics should NOT be equal
// SHOULD FAIL
proof fn test_mutation_assert_non_equivalent_equal()
{
    let p: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() >= 0);
    let q: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() > 0);

    // head >= 0 is NOT the same as head > 0 (differ at head == 0)
    assert(p == q);
}

// Test 5: Mutated spec — conclude entailment for a DIFFERENT spec
// Use entails_and_temp with spec1, then assert conclusion for spec2
// SHOULD FAIL
proof fn test_mutation_wrong_spec_in_conclusion()
{
    let spec1: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);
    let spec2: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 1);
    let p: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() >= 0);
    let q: TempPred<int> = TempPred::new(|ex: Execution<int>| ex.head() == 0);

    assume(spec1.entails(p));
    assume(spec1.entails(q));

    entails_and_temp(spec1, p, q);

    // Mutated: assert conclusion for spec2, not spec1
    // spec2 is head==1, q is head==0, so spec2.entails(q) means 1==0, which is false
    assert(spec2.entails(p.and(q)));
}

}
