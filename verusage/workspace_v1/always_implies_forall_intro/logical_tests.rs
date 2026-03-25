use vstd::prelude::*;

fn main() {}

verus! {

// ============================================================
// Definitions (from target file)
// ============================================================

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
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures q.satisfied_by(ex),
{
    unimplemented!()
}

#[verifier::external_body]
proof fn tla_forall_always_implies_equality2<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(|a: A| always(p.implies(a_to_q(a)))) == always(p.implies(tla_forall(a_to_q))),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn spec_entails_tla_forall<T, A>(spec: TempPred<T>, a_to_p: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| spec.entails(#[trigger] a_to_p(a)),
    ensures spec.entails(tla_forall(a_to_p)),
{
    unimplemented!()
}

pub proof fn always_implies_forall_intro<T, A>(spec: TempPred<T>, p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    requires forall |a: A| #[trigger] spec.entails(always(p.implies(a_to_q(a)))),
    ensures spec.entails(always(p.implies(tla_forall(a_to_q)))),
{
    let a_to_always_p_implies_q = |a: A| always(p.implies(a_to_q(a)));
    spec_entails_tla_forall::<T, A>(spec, a_to_always_p_implies_q);
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies always(p.implies(tla_forall(a_to_q))).satisfied_by(ex) by {
        implies_apply::<T>(ex, spec, tla_forall(a_to_always_p_implies_q));
        tla_forall_always_implies_equality2::<T, A>(p, a_to_q)
    };
}

// ============================================================
// Logical Tests: Properties NOT explicitly guaranteed
// ============================================================

// SHOULD FAIL
// Test 1: Assert universal validity instead of spec-entailment.
// The lemma gives spec ⊨ □(p → ∀a.q(a)), which means for all executions
// satisfying spec, the property holds. It does NOT mean valid(□(p → ∀a.q(a)))
// which requires it to hold on ALL executions unconditionally.
proof fn logical_valid_instead_of_entails()
{
    let spec = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 42);
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    let a_to_q = |a: int| TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > a);

    assume(forall |a: int| #[trigger] spec.entails(always(p.implies(a_to_q(a)))));
    always_implies_forall_intro::<int, int>(spec, p, a_to_q);

    // SHOULD FAIL: valid() is strictly stronger than spec.entails()
    assert(valid(always(p.implies(tla_forall(a_to_q)))));
}

// SHOULD FAIL
// Test 2: Conclude that spec entails the antecedent p unconditionally.
// Knowing spec ⊨ □(p → ∀a.q(a)) does NOT tell us spec ⊨ □(p).
proof fn logical_entails_antecedent()
{
    let spec = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 42);
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    let a_to_q = |a: int| TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > a);

    assume(forall |a: int| #[trigger] spec.entails(always(p.implies(a_to_q(a)))));
    always_implies_forall_intro::<int, int>(spec, p, a_to_q);

    // SHOULD FAIL: the lemma says nothing about p being true;
    // it only says IF p THEN ∀a.q(a)
    assert(spec.entails(always(p)));
}

// SHOULD FAIL
// Test 3: Derive a single instance from the conclusion without using tla_forall.
// From spec ⊨ □(p → ∀a.q(a)) on a concrete execution, try to extract
// q(specific_a).satisfied_by(ex) without going through tla_forall properly.
// This tests whether the spec allows short-circuiting the universal quantification.
proof fn logical_single_instance_without_forall()
{
    let spec = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 42);
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    let a_to_q = |a: int| TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > a);

    // Assume only for a single value, NOT for all a
    assume(spec.entails(always(p.implies(a_to_q(0int)))));

    // SHOULD FAIL: can't conclude universal from single instance
    assert(spec.entails(always(p.implies(tla_forall(a_to_q)))));
}

}
