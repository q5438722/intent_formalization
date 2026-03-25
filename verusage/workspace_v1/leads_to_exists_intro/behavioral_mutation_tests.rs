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

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Mutate conclusion from tla_exists to tla_forall
// The real postcondition uses tla_exists(a_to_p). Mutating it to tla_forall(a_to_p)
// makes the conclusion strictly stronger and should NOT be provable from the same premises.
// SHOULD FAIL
proof fn test_mutation_exists_to_forall()
{
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 0);
    let a_to_p = |a: bool| TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == if a { 1int } else { 2int });

    // Assume the precondition holds
    assume(forall |a: bool| #[trigger] spec.entails(a_to_p(a).leads_to(q)));

    // The correct conclusion would be: spec.entails(tla_exists(a_to_p).leads_to(q))
    // Mutated: assert tla_forall instead of tla_exists
    leads_to_exists_intro::<int, bool>(spec, a_to_p, q);
    assert(spec.entails(tla_forall(a_to_p).leads_to(q)));
}

// Test 2: Mutate leads_to direction (reverse p and q)
// The real conclusion is: tla_exists(a_to_p) ~> q
// Mutated to: q ~> tla_exists(a_to_p)
// SHOULD FAIL
proof fn test_mutation_reverse_leads_to()
{
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 0);
    let a_to_p = |a: bool| TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == if a { 1int } else { 2int });

    assume(forall |a: bool| #[trigger] spec.entails(a_to_p(a).leads_to(q)));

    // Mutated: reverse direction
    leads_to_exists_intro::<int, bool>(spec, a_to_p, q);
    assert(spec.entails(q.leads_to(tla_exists(a_to_p))));
}

// Test 3: Mutate leads_to to entails (drop temporal structure)
// Instead of concluding spec.entails(tla_exists(a_to_p).leads_to(q)),
// try to conclude spec.entails(q) directly—dropping the temporal implication.
// SHOULD FAIL
proof fn test_mutation_leads_to_to_plain_entails()
{
    let spec = TempPred::<int>::new(|ex: Execution<int>| true);
    let q = TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == 0);
    let a_to_p = |a: bool| TempPred::<int>::new(|ex: Execution<int>| (ex.nat_to_state)(0) == if a { 1int } else { 2int });

    assume(forall |a: bool| #[trigger] spec.entails(a_to_p(a).leads_to(q)));

    leads_to_exists_intro::<int, bool>(spec, a_to_p, q);
    // Mutated: try to derive q holds unconditionally under spec
    assert(spec.entails(q));
}

}
