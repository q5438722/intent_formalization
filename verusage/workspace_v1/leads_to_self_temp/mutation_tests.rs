use vstd::prelude::*;

fn main() {}

verus! {

// ===== Definitions from source =====

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
}

pub open spec fn always<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| forall |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists |i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
proof fn eventually_proved_by_witness<T>(ex: Execution<T>, p: TempPred<T>, witness_idx: nat)
    requires p.satisfied_by(ex.suffix(witness_idx)),
    ensures eventually(p).satisfied_by(ex),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires forall |i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures ex1 == ex2,
{
    unimplemented!()
}

// ===== Behavioral Mutation Tests =====

// Test 1: Mutate leads_to_self — p leads to a DIFFERENT arbitrary predicate q.
// leads_to_self only proves p ~> p, not p ~> q for arbitrary q.
// SHOULD FAIL
proof fn test_leads_to_different_predicate<T>(p: TempPred<T>, q: TempPred<T>)
    ensures valid(p.leads_to(q)),
{
    // Mutated: changed p.leads_to(p) to p.leads_to(q)
}

// Test 2: Strengthen the postcondition — p leads to always(p) instead of just p.
// leads_to_self proves p ~> p, but not p ~> always(p).
// SHOULD FAIL
proof fn test_leads_to_always_self<T>(p: TempPred<T>)
    ensures valid(p.leads_to(always(p))),
{
    // Mutated: strengthened eventually(p) to always(p) in the leads_to target
}

// Test 3: Mutate the temporal operator — replace eventually with always in the definition.
// p => always(p) is strictly stronger than p => eventually(p).
// SHOULD FAIL
proof fn test_implies_always_instead_of_eventually<T>(p: TempPred<T>)
    ensures valid(always(p.implies(always(p)))),
{
    // Mutated: swapped eventually(p) for always(p) in the leads_to definition
}

}
