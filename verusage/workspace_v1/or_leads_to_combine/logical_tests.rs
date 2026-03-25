use vstd::prelude::*;

fn main() {}

verus!{

// ===== Definitions from target file =====

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

    pub open spec fn or(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) || other.satisfied_by(ex))
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

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall |ex: Execution<T>| temp_pred.satisfied_by(ex)
}

#[verifier::external_body]
#[verifier::spinoff_prover]
proof fn leads_to_unfold<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires p.leads_to(q).satisfied_by(ex),
    ensures forall |i: nat| p.implies(eventually(q)).satisfied_by(#[trigger] ex.suffix(i)),
{
    unimplemented!()
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

pub proof fn or_leads_to_combine<T>(spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>)
    requires
        spec.entails(p.leads_to(r)),
        spec.entails(q.leads_to(r)),
    ensures spec.entails(p.or(q).leads_to(r)),
{
    assert forall |ex| #[trigger] spec.satisfied_by(ex) implies p.or(q).leads_to(r).satisfied_by(ex) by {
        assert forall |i| #[trigger] p.or(q).satisfied_by(ex.suffix(i)) implies eventually(r).satisfied_by(ex.suffix(i)) by {
            implies_apply::<T>(ex, spec, p.leads_to(r));
            implies_apply::<T>(ex, spec, q.leads_to(r));
            leads_to_unfold::<T>(ex, p, r);
            leads_to_unfold::<T>(ex, q, r);
        };
    };
}

// ===== Logical Tests =====

// Test 1: Transitivity misuse — derive p leads_to q from p leads_to r and q leads_to r
// This is NOT a valid temporal logic inference.
// SHOULD FAIL
proof fn test_logical_transitivity_misuse<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>,
)
    requires
        spec.entails(p.leads_to(r)),
        spec.entails(q.leads_to(r)),
    ensures
        spec.entails(p.leads_to(q)),  // NOT entailed: sharing a target doesn't connect p to q
{
}

// Test 2: Spec independence — derive valid(p.or(q).leads_to(r)) without spec
// The theorem only gives spec.entails(...), not valid(...).
// SHOULD FAIL
proof fn test_logical_spec_independence<T>(
    spec: TempPred<T>, p: TempPred<T>, q: TempPred<T>, r: TempPred<T>,
)
    requires
        spec.entails(p.leads_to(r)),
        spec.entails(q.leads_to(r)),
    ensures
        valid(p.or(q).leads_to(r)),  // NOT entailed: holds only under spec, not universally
{
    or_leads_to_combine(spec, p, q, r);
}

// Test 3: Commutativity of leads_to — derive r leads_to p from p leads_to r
// leads_to is NOT symmetric in temporal logic.
// SHOULD FAIL
proof fn test_logical_leads_to_not_symmetric<T>(
    spec: TempPred<T>, p: TempPred<T>, r: TempPred<T>,
)
    requires
        spec.entails(p.leads_to(r)),
    ensures
        spec.entails(r.leads_to(p)),  // NOT entailed: leads_to is not symmetric
{
}

}
