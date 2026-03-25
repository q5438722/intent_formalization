# Adversarial Proof Test Results for `strengthen_next.rs`

## Target Specification

The file implements a temporal logic framework for the Anvil verification library. The main theorem `strengthen_next` combines:
- `spec ⊢ □(next)` (spec entails always-next)
- `spec ⊢ □(inv)` (spec entails always-invariant)
- `next_and_inv ⟺ next ∧ inv` (bidirectional equivalence)

to conclude `spec ⊢ □(next_and_inv)`.

Three axioms (`external_body`) support the proof: `temp_pred_equality`, `always_and_equality`, `entails_and_temp`.

---

## Results Summary

| Test File | Tests | Failed (Expected) | Passed (Unexpected) |
|-----------|-------|--------------------|---------------------|
| `boundary_tests.rs` | 6 | 6 | 0 |
| `behavioral_mutation_tests.rs` | 5 | 5 | 0 |
| `logical_tests.rs` | 5 | 5 | 0 |
| **Total** | **16** | **16** | **0** |

**All 16 tests failed verification as expected.** The specification correctly rejects all adversarial queries.

---

## Boundary Tests (6/6 FAILED ✓)

| # | Test | Failure Mode | Result |
|---|------|--------------|--------|
| 1 | `test_boundary_missing_always_next` | Missing `spec ⊢ □(next)` precondition | FAILED ✓ |
| 2 | `test_boundary_missing_always_inv` | Missing `spec ⊢ □(inv)` precondition | FAILED ✓ |
| 3 | `test_boundary_missing_forward_equiv` | Missing `next_and_inv ⊢ next ∧ inv` with weaker predicate | FAILED ✓ |
| 4 | `test_boundary_missing_backward_equiv` | Missing `next ∧ inv ⊢ next_and_inv` with stronger predicate | FAILED ✓ |
| 5 | `test_boundary_equality_one_direction` | `temp_pred_equality` with only one entailment direction | FAILED ✓ |
| 6 | `test_boundary_entails_and_missing_second` | `entails_and_temp` missing second entailment | FAILED ✓ |

**Note:** Initial tests 3, 4, 6 passed because the concrete predicates made missing preconditions trivially derivable. Fixed by using semantically distinct predicates (weaker/stronger `next_and_inv`, non-entailed `q`).

## Behavioral Mutation Tests (5/5 FAILED ✓)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_valid_instead_of_entails` | Conclusion strengthened from `spec.entails(…)` to `valid(…)` | FAILED ✓ |
| 2 | `test_mutation_negated_conclusion` | Conclusion negated: `spec ⊢ □(¬next_and_inv)` | FAILED ✓ |
| 3 | `test_mutation_wrong_predicate_in_conclusion` | Conclusion uses unrelated predicate `s2 == s1 * 2` | FAILED ✓ |
| 4 | `test_mutation_assert_non_equivalent_equal` | Assert `head ≥ 0 == head > 0` (differ at 0) | FAILED ✓ |
| 5 | `test_mutation_wrong_spec_in_conclusion` | Conclusion transferred to different spec | FAILED ✓ |

## Logical Tests (5/5 FAILED ✓)

| # | Test | Unintended Property Tested | Result |
|---|------|---------------------------|--------|
| 1 | `test_logical_entails_does_not_imply_valid` | `spec.entails(p) ⟹ valid(p)` — conflating relative and absolute truth | FAILED ✓ |
| 2 | `test_logical_entails_does_not_lift_to_always` | `spec.entails(p) ⟹ spec.entails(□p)` — temporal lifting without justification | FAILED ✓ |
| 3 | `test_logical_cross_spec_entailment` | `spec₁ ⊢ p ∧ spec₂ ⊢ q ⟹ spec₁ ⊢ q` — cross-spec transfer | FAILED ✓ |
| 4 | `test_logical_false_valid` | `valid(head > 0)` — asserting non-universal property | FAILED ✓ |
| 5 | `test_logical_entails_not_symmetric` | `p ⊢ q ⟹ q ⊢ p` — entailment symmetry | FAILED ✓ |

---

## Conclusion

The specification for `strengthen_next` is **well-bounded**: it correctly rejects all 16 adversarial queries across boundary violations, behavioral mutations, and unintended logical inferences. The preconditions are tight (each is individually necessary), the postconditions are precise (no stronger conclusions derivable), and the axioms do not enable unsound reasoning chains.
