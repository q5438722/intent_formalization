# Adversarial Test Results: `leads_to_stable.rs`

## Target
Temporal logic library with axioms (`external_body`) for `always`, `eventually`, `leads_to`, and the main theorem `leads_to_stable`.

## Results Summary

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary | 8 | 8 | 0 |
| Behavioral Mutation | 7 | 7 | 0 |
| Logical | 8 | 8 | 0 |
| **Total** | **23** | **23** | **0** |

**All 23 adversarial tests were correctly rejected by the specification.**

---

## Boundary Tests (8/8 FAILED ✓)

| Test | Failure Mode | Result |
|---|---|---|
| `test_implies_apply_missing_antecedent` | Call without `p.satisfied_by(ex)` | Precondition error ✓ |
| `test_implies_apply_missing_implication` | Call without `p.implies(q).satisfied_by(ex)` | Precondition error ✓ |
| `test_always_propagate_no_always` | Call without `always(p)` | Precondition error ✓ |
| `test_witness_not_satisfied` | Witness doesn't satisfy predicate | Precondition error ✓ |
| `test_induction_no_base_case` | No base case for induction | Precondition error ✓ |
| `test_leads_to_stable_missing_stability` | Missing stability preservation | Assertion error ✓ |
| `test_execution_equality_no_evidence` | No pointwise equality | Precondition error ✓ |
| `test_leads_to_unfold_no_precondition` | No leads_to precondition | Precondition error ✓ |

## Behavioral Mutation Tests (7/7 FAILED ✓)

| Test | Mutation | Result |
|---|---|---|
| `test_eventually_implies_always` | eventually(p) ⇒ always(p) | Postcondition error ✓ |
| `test_leads_to_not_symmetric` | p ~> q ⇒ q ~> p | Postcondition error ✓ |
| `test_implication_not_symmetric` | □(p⇒q) ⇒ □(q⇒p) | Postcondition error ✓ |
| `test_leads_to_stable_stronger_conclusion` | Strengthen to always(p∧q) | Assertion error ✓ |
| `test_leads_to_without_stability` | Remove stability condition | Assertion error ✓ |
| `test_always_pair_not_leads_to` | always(p) ⇒ p~>q | Postcondition error ✓ |
| `test_witness_off_by_one` | Off-by-one witness shift | Precondition error ✓ |

## Logical Tests (8/8 FAILED ✓)

| Test | Property Tested | Result |
|---|---|---|
| `test_derive_false` | Axiom soundness (derive ⊥) | Postcondition error ✓ |
| `test_arbitrary_predicates_equal` | All predicates equivalent | Postcondition error ✓ |
| `test_single_execution_not_valid` | Single execution ⇒ valid | Postcondition error ✓ |
| `test_execution_equality_wrong` | Partial equality ⇒ full equality | Precondition error ✓ |
| `test_suffix_collapse` | suffix(a).suffix(a) = suffix(a) | Postcondition error ✓ |
| `test_entails_not_symmetric` | Entailment symmetry | Postcondition error ✓ |
| `test_always_and_eventually_not` | □p ∧ ◇¬p consistency | Postcondition error ✓ |
| `test_no_forced_determinism` | Same start ⇒ same execution | Postcondition error ✓ |

---

## Conclusion

The specification for `leads_to_stable` is **consistent** across all three query dimensions:

1. **Boundary**: All axiom preconditions are enforced — invalid inputs are rejected.
2. **Behavioral**: Incorrect output relations (symmetry, strengthening, removing conditions) are rejected.
3. **Logical**: Unintended properties (unsoundness, determinism, predicate equivalence, temporal contradictions) cannot be derived.

No specification weaknesses were detected.
