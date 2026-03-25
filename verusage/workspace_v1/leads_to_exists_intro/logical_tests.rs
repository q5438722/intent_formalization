use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions (copied from target) =====

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

    pub open spec fn leads_to(self, other: Self) -> Self {
        always(self.implies(eventually(other)))
    }

    pub open spec fn entails(self, other: Self) -> bool {
        valid(self.implies(other))
    }
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
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

// ===== Axioms (copied from target) =====

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
proof fn tla_forall_leads_to_equality1<T, A>(a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    ensures tla_forall(|a: A| a_to_p(a).leads_to(q)) == tla_exists(a_to_p).leads_to(q),
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

pub proof fn leads_to_exists_intro<T, A>(spec: TempPred<T>, a_to_p: spec_fn(A) -> TempPred<T>, q: TempPred<T>)
    requires forall |a: A| #[trigger] spec.entails(a_to_p(a).leads_to(q)),
    ensures spec.entails(tla_exists(a_to_p).leads_to(q)),
{
    let a_to_p_leads_to_q = |a: A| a_to_p(a).leads_to(q);
    spec_entails_tla_forall::<T, A>(spec, a_to_p_leads_to_q);
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies tla_exists(a_to_p).leads_to(q).satisfied_by(ex) by {
        implies_apply::<T>(ex, spec, tla_forall(a_to_p_leads_to_q));
        tla_forall_leads_to_equality1::<T, A>(a_to_p, q);
    };
}

// ===== LOGICAL TESTS =====

// Test 1: leads_to does NOT imply always(q)
// From spec |= (p ~> q), one should NOT be able to derive spec |= always(q),
// because leads_to only guarantees q eventually follows p, not that q holds forever.
// SHOULD FAIL
proof fn test_logical_leads_to_does_not_imply_always()
{
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 0);

    assume(spec.entails(p.leads_to(q)));

    // This is too strong: leads_to does NOT give us always(q)
    assert(spec.entails(always(q)));
}

// Test 2: leads_to q does NOT imply leads_to always(q)
// From spec |= (p ~> q), one should NOT derive spec |= (p ~> always(q)).
// leads_to only guarantees q holds at some future point, not that it holds forever after.
// SHOULD FAIL
proof fn test_logical_leads_to_does_not_strengthen_to_always()
{
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 0);

    assume(spec.entails(p.leads_to(q)));

    // Strengthened: leads_to should NOT imply leads_to always
    assert(spec.entails(p.leads_to(always(q))));
}

// Test 3: Premises of leads_to_exists_intro do NOT imply the witness exists
// From forall a, spec |= (a_to_p(a) ~> q), we should NOT derive spec |= exists a, a_to_p(a).
// The leads-to premises say "if a_to_p(a) ever holds, q follows", but they
// don't guarantee that any a_to_p(a) actually holds.
// SHOULD FAIL
proof fn test_logical_no_existence_from_leads_to()
{
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 0);
    let a_to_p = |a: bool| TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == if a { 1int } else { 2int });

    assume(forall |a: bool| #[trigger] spec.entails(a_to_p(a).leads_to(q)));

    // This is NOT provable: leads_to doesn't imply the antecedent exists
    assert(spec.entails(tla_exists(a_to_p)));
}

// Test 4: entails is NOT symmetric
// spec.entails(p) does NOT imply p.entails(spec)
// Using spec that is strictly stronger than p, so reverse entailment should fail.
// SHOULD FAIL
proof fn test_logical_entails_not_symmetric()
{
    let spec = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 5);
    let p = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) > 0);

    assume(spec.entails(p));

    // Entails is NOT symmetric: (>5) entails (>0) but NOT vice versa
    assert(p.entails(spec));
}

}
