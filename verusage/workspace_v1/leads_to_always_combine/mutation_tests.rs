use vstd::prelude::*;

fn main() {}

verus! {

// ========== Definitions from source ==========

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

    pub open spec fn and(self, other: Self) -> Self {
        TempPred::new(|ex: Execution<T>| self.satisfied_by(ex) && other.satisfied_by(ex))
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
    TempPred::new(|ex: Execution<T>| forall|i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn eventually<T>(temp_pred: TempPred<T>) -> TempPred<T> {
    TempPred::new(|ex: Execution<T>| exists|i: nat| #[trigger] temp_pred.satisfied_by(ex.suffix(i)))
}

pub open spec fn valid<T>(temp_pred: TempPred<T>) -> bool {
    forall|ex: Execution<T>| temp_pred.satisfied_by(ex)
}

// ========== Helper lemmas (axioms) ==========

#[verifier::external_body]
proof fn implies_apply<T>(ex: Execution<T>, p: TempPred<T>, q: TempPred<T>)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
    ensures
        q.satisfied_by(ex),
{
    unimplemented!()
}

#[verifier::external_body]
proof fn always_propagate_forwards<T>(ex: Execution<T>, p: TempPred<T>, i: nat)
    requires
        always(p).satisfied_by(ex),
    ensures
        always(p).satisfied_by(ex.suffix(i)),
{
    unimplemented!()
}

#[verifier::external_body]
proof fn eventually_proved_by_witness<T>(ex: Execution<T>, p: TempPred<T>, witness_idx: nat)
    requires
        p.satisfied_by(ex.suffix(witness_idx)),
    ensures
        eventually(p).satisfied_by(ex),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn execution_equality<T>(ex1: Execution<T>, ex2: Execution<T>)
    requires
        forall|i: nat| #[trigger] (ex1.nat_to_state)(i) == (ex2.nat_to_state)(i),
    ensures
        ex1 == ex2,
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn always_and_equality<T>(p: TempPred<T>, q: TempPred<T>)
    ensures
        always(p.and(q)) == always(p).and(always(q)),
{
    unimplemented!()
}

pub proof fn leads_to_always_combine<T>(
    spec: TempPred<T>,
    p: TempPred<T>,
    q: TempPred<T>,
    r: TempPred<T>,
)
    requires
        spec.entails(p.leads_to(always(q))),
        spec.entails(p.leads_to(always(r))),
    ensures
        spec.entails(p.leads_to(always(q.and(r)))),
        spec.entails(p.leads_to(always(q).and(always(r)))),
{
    assert forall|ex| #[trigger] spec.satisfied_by(ex) implies p.leads_to(always(q.and(r))).satisfied_by(ex) by {
        assert forall|i| #[trigger] p.satisfied_by(ex.suffix(i)) implies eventually(always(q.and(r))).satisfied_by(ex.suffix(i)) by {
            implies_apply::<T>(ex, spec, p.leads_to(always(q)));
            implies_apply::<T>(ex, spec, p.leads_to(always(r)));
            implies_apply::<T>(ex.suffix(i), p, eventually(always(q)));
            implies_apply::<T>(ex.suffix(i), p, eventually(always(r)));
            let witness_q_idx = choose|j: nat| #[trigger] always(q).satisfied_by(ex.suffix(i).suffix(j));
            let witness_r_idx = choose|j: nat| #[trigger] always(r).satisfied_by(ex.suffix(i).suffix(j));
            if witness_q_idx < witness_r_idx {
                always_propagate_forwards::<T>(
                    ex.suffix(i).suffix(witness_q_idx),
                    q,
                    (witness_r_idx - witness_q_idx) as nat,
                );
                execution_equality::<T>(
                    ex.suffix(i).suffix(witness_r_idx),
                    ex.suffix(i).suffix(witness_q_idx).suffix(
                        (witness_r_idx - witness_q_idx) as nat,
                    ),
                );
                eventually_proved_by_witness(ex.suffix(i), always(q.and(r)), witness_r_idx);
            } else {
                always_propagate_forwards::<T>(
                    ex.suffix(i).suffix(witness_r_idx),
                    r,
                    (witness_q_idx - witness_r_idx) as nat,
                );
                execution_equality::<T>(
                    ex.suffix(i).suffix(witness_q_idx),
                    ex.suffix(i).suffix(witness_r_idx).suffix(
                        (witness_q_idx - witness_r_idx) as nat,
                    ),
                );
                eventually_proved_by_witness(ex.suffix(i), always(q.and(r)), witness_q_idx);
            }
        };
    };
    always_and_equality(q, r);
}

// ========== BEHAVIORAL MUTATION TESTS ==========
// Each test uses valid inputs but asserts a MUTATED (wrong) conclusion. SHOULD FAIL.

// SHOULD FAIL: implies_apply ensures q at ex; mutation asserts q at ex.suffix(1) instead
proof fn test_mutation_1_implies_wrong_position(
    ex: Execution<int>,
    p: TempPred<int>,
    q: TempPred<int>,
)
    requires
        p.implies(q).satisfied_by(ex),
        p.satisfied_by(ex),
{
    implies_apply::<int>(ex, p, q);
    // Correct: q.satisfied_by(ex)
    // Mutated: q at a shifted position — not guaranteed
    assert(q.satisfied_by(ex.suffix(1)));
}

// SHOULD FAIL: always_propagate_forwards ensures always(p) at suffix(i); mutation strengthens to always(p.and(q))
proof fn test_mutation_2_always_propagate_strengthened(
    ex: Execution<int>,
    p: TempPred<int>,
    q: TempPred<int>,
    i: nat,
)
    requires
        always(p).satisfied_by(ex),
{
    always_propagate_forwards::<int>(ex, p, i);
    // Correct: always(p).satisfied_by(ex.suffix(i))
    // Mutated: strengthened to always(p.and(q)) — q is unconstrained
    assert(always(p.and(q)).satisfied_by(ex.suffix(i)));
}

// SHOULD FAIL: eventually_proved_by_witness ensures eventually(p); mutation strengthens to always(p)
proof fn test_mutation_3_eventually_to_always(
    ex: Execution<int>,
    p: TempPred<int>,
    witness_idx: nat,
)
    requires
        p.satisfied_by(ex.suffix(witness_idx)),
{
    eventually_proved_by_witness::<int>(ex, p, witness_idx);
    // Correct: eventually(p).satisfied_by(ex)
    // Mutated: always(p) — far too strong
    assert(always(p).satisfied_by(ex));
}

// SHOULD FAIL: always_and_equality holds for and; mutation claims same for implies (wrong in temporal logic)
proof fn test_mutation_4_always_implies_not_distributes(
    p: TempPred<int>,
    q: TempPred<int>,
) {
    always_and_equality::<int>(p, q);
    // Correct: always(p.and(q)) == always(p).and(always(q))
    // Mutated: claim the same for implies — this is NOT valid
    assert(always(p.implies(q)) == always(p).implies(always(q)));
}

// SHOULD FAIL: leads_to_always_combine gives p.leads_to(always(q.and(r))); mutation drops leads_to
proof fn test_mutation_5_combine_drop_leads_to(
    spec: TempPred<int>,
    p: TempPred<int>,
    q: TempPred<int>,
    r: TempPred<int>,
)
    requires
        spec.entails(p.leads_to(always(q))),
        spec.entails(p.leads_to(always(r))),
{
    leads_to_always_combine::<int>(spec, p, q, r);
    // Correct: spec.entails(p.leads_to(always(q.and(r))))
    // Mutated: drop the leads_to — claim always(q.and(r)) directly
    assert(spec.entails(always(q.and(r))));
}

// SHOULD FAIL: leads_to_always_combine gives p~>always(q∧r); mutation adds extra conjunct s
proof fn test_mutation_6_combine_extra_conjunct(
    spec: TempPred<int>,
    p: TempPred<int>,
    q: TempPred<int>,
    r: TempPred<int>,
    s: TempPred<int>,
)
    requires
        spec.entails(p.leads_to(always(q))),
        spec.entails(p.leads_to(always(r))),
{
    leads_to_always_combine::<int>(spec, p, q, r);
    // Correct: spec.entails(p.leads_to(always(q.and(r))))
    // Mutated: sneak in extra predicate s — not justified
    assert(spec.entails(p.leads_to(always(q.and(r).and(s)))));
}

}
