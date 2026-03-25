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

// === Logical Tests ===

// Test L1: Try to derive that p is universally valid from the equality
// The spec gives an equality between temporal predicates but says nothing about p's validity
// SHOULD FAIL
proof fn test_logical_derive_valid_p<T, A>(p: TempPred<T>, a_to_q: spec_fn(A) -> TempPred<T>)
    ensures valid(p),
{
    tla_forall_always_implies_equality2::<T, A>(p, a_to_q);
}

// Test L2: Try to derive that p entails always(p)
// A property holding at one state should NOT imply it holds at all future states
// SHOULD FAIL
proof fn test_logical_p_entails_always_p<T>(p: TempPred<T>)
    ensures p.entails(always(p)),
{
    tla_forall_always_implies_equality2::<T, int>(p, |_a: int| p);
}

// Test L3: Try to derive □(p→q) ⊨ (p→□q)
// always(p implies q) should NOT entail (p implies always(q)) in general
// □(p→q) means at every step, if p then q; but (p→□q) means if p now, then q forever
// SHOULD FAIL
proof fn test_logical_always_implies_does_not_distribute<T>(p: TempPred<T>, q: TempPred<T>)
    ensures always(p.implies(q)).entails(p.implies(always(q))),
{
    tla_forall_always_implies_equality2::<T, int>(p, |_a: int| q);
}

// Test L4: Try to derive tla_forall(a_to_q) ⊨ always(tla_forall(a_to_q))
// Universal quantification at a single state should NOT persist through all future states
// SHOULD FAIL
proof fn test_logical_forall_entails_always_forall<T, A>(a_to_q: spec_fn(A) -> TempPred<T>)
    ensures tla_forall(a_to_q).entails(always(tla_forall(a_to_q))),
{
    let p = TempPred::<T>::new(|ex: Execution<T>| true);
    tla_forall_always_implies_equality2::<T, A>(p, a_to_q);
}

}
