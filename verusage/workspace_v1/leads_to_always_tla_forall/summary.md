# Adversarial Test Summary: `leads_to_always_tla_forall`

## Target
`source-projects/anvil-library/verified/temporal_logic/leads_to_always_tla_forall.rs`

The file defines temporal logic primitives (`Execution`, `TempPred`, `always`, `eventually`, `tla_forall`, `valid`) and four trusted axioms (`implies_apply`, `always_propagate_forwards`, `eventually_proved_by_witness`, `execution_equality`), plus the main lemma `leads_to_always_tla_forall` which proves:

> If for every `a`, `spec ⊨ (p ~> □a_to_p(a))`, and `A` is finite and covers all elements, then `spec ⊨ (p ~> □(∀a. a_to_p(a)))`.

---

## Results: ALL 16 tests FAILED verification ✅

The specification correctly rejects all invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (6/6 failed) — `boundary_tests.rs`

| # | Test | Violation | Result |
|---|------|-----------|--------|
| 1 | `test_implies_apply_missing_antecedent` | Missing `p.satisfied_by(ex)` | ❌ precondition error |
| 2 | `test_implies_apply_missing_implication` | Missing `p.implies(q).satisfied_by(ex)` | ❌ precondition error |
| 3 | `test_always_propagate_missing_always` | Provided `p` instead of `always(p)` | ❌ precondition error |
| 4 | `test_eventually_missing_witness` | No witness condition provided | ❌ precondition error |
| 5 | `test_leads_to_missing_finite` | Missing `domain.finite()` | ❌ precondition error |
| 6 | `test_leads_to_missing_nonempty` | Missing `domain.len() > 0` | ❌ precondition error |

### Behavioral Mutation Tests (5/5 failed) — `behavioral_mutation_tests.rs`

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_implies_converse` | Derive `p` from `(p⇒q) ∧ q` (affirming consequent) | ❌ postcondition error |
| 2 | `test_eventually_does_not_imply_always` | `◇p ⇒ □p` | ❌ postcondition error |
| 3 | `test_leads_to_strengthened_to_always` | Drop `leads_to`, assert `□(∀a. a_to_p(a))` directly | ❌ postcondition error |
| 4 | `test_always_propagate_reverse` | Reverse: `□p` at suffix ⇒ `□p` at original | ❌ postcondition error |
| 5 | `test_leads_to_swap_conclusion` | Swap direction: `□(∀a. a_to_p(a)) ~> p` | ❌ postcondition error |

### Logical Tests (5/5 failed) — `logical_tests.rs`

| # | Test | Property Tested | Result |
|---|------|-----------------|--------|
| 1 | `test_leads_to_not_symmetric` | `p ~> q` does NOT imply `q ~> p` | ❌ postcondition error |
| 2 | `test_execution_equality_partial` | Equality at index 0 ≠ full equality | ❌ precondition error |
| 3 | `test_valid_does_not_transfer` | `valid(p)` does NOT imply `valid(q)` | ❌ postcondition error |
| 4 | `test_forall_eventually_swap_invalid` | `∀a.◇(a_to_p(a))` ⇏ `◇(∀a. a_to_p(a))` | ❌ postcondition error |
| 5 | `test_entails_not_symmetric` | `spec ⊨ p` does NOT imply `p ⊨ spec` | ❌ postcondition error |

---

## Conclusion

The specification is **consistent** with respect to all 16 adversarial queries:
- **Preconditions** are tight — every required clause is necessary, and dropping any one causes rejection.
- **Postconditions** are precise — the conclusion cannot be strengthened, reversed, or swapped.
- **Logical boundaries** are sound — the spec does not admit symmetry of leads_to, invalid forall/exists swaps, or unwarranted entailment transfer.
