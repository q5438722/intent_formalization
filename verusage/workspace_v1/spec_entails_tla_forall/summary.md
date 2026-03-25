# Adversarial Test Results: `spec_entails_tla_forall`

**Target**: `source-projects/anvil-library/verified/temporal_logic/spec_entails_tla_forall.rs`
**Date**: 2026-03-22

## Specification Under Test

The file defines temporal logic primitives (`Execution`, `TempPred`, `valid`, `tla_forall`) and two proof functions:

- **`implies_apply`** (external_body axiom): Modus ponens — given `(p ⟹ q)(ex)` and `p(ex)`, conclude `q(ex)`.
- **`spec_entails_tla_forall`**: If `spec` entails each `a_to_p(a)` individually, then `spec` entails `tla_forall(a_to_p)`.

## Results Summary

| # | File | Test | Type | Verdict | Expected |
|---|------|------|------|---------|----------|
| 1 | boundary_tests.rs | `test_boundary_missing_forall_precondition` | Precondition violation | ❌ FAILED | ✅ SHOULD FAIL |
| 2 | boundary_tests.rs | `test_boundary_implies_apply_missing_antecedent` | Precondition violation | ❌ FAILED | ✅ SHOULD FAIL |
| 3 | boundary_tests.rs | `test_boundary_implies_apply_missing_implication` | Precondition violation | ❌ FAILED | ✅ SHOULD FAIL |
| 4 | boundary_tests.rs | `test_boundary_partial_entailment` | Precondition violation | ❌ FAILED | ✅ SHOULD FAIL |
| 5 | behavioral_mutation_tests.rs | `test_mutation_conclude_valid_instead_of_entails` | Output mutation | ❌ FAILED | ✅ SHOULD FAIL |
| 6 | behavioral_mutation_tests.rs | `test_mutation_reversed_entailment` | Output mutation | ❌ FAILED | ✅ SHOULD FAIL |
| 7 | behavioral_mutation_tests.rs | `test_mutation_entails_arbitrary` | Output mutation | ❌ FAILED | ✅ SHOULD FAIL |
| 8 | behavioral_mutation_tests.rs | `test_mutation_implies_apply_wrong_conclusion` | Output mutation | ❌ FAILED | ✅ SHOULD FAIL |
| 9 | logical_tests.rs | `test_logical_entailment_implies_spec_valid` | Unintended inference | ❌ FAILED | ✅ SHOULD FAIL |
| 10 | logical_tests.rs | `test_logical_implies_apply_derive_false` | Soundness | ❌ FAILED | ✅ SHOULD FAIL |
| 11 | logical_tests.rs | `test_logical_individual_converse` | Unintended inference | ❌ FAILED | ✅ SHOULD FAIL |
| 12 | logical_tests.rs | `test_logical_entailment_not_validity` | Unintended inference | ❌ FAILED | ✅ SHOULD FAIL |

**Total: 12/12 tests correctly rejected.**

## Analysis

### Boundary Tests (4/4 rejected)
The specification correctly guards its functions with preconditions:
- `spec_entails_tla_forall` properly requires the **universal** quantifier (`forall |a|`) — partial entailment (e.g., only for `true` but not `false`) is rejected.
- `implies_apply` requires **both** the implication and the antecedent — omitting either is rejected.

### Behavioral Mutation Tests (4/4 rejected)
The specification rejects all mutated conclusions:
- **`valid(X)` vs `entails(X)`**: The spec correctly distinguishes conditional entailment from unconditional validity.
- **Reversed entailment**: `spec ⊨ tla_forall(a_to_p)` does NOT imply `tla_forall(a_to_p) ⊨ spec`.
- **Arbitrary entailment**: Entailing one predicate does not entail unrelated predicates.
- **Reversed modus ponens**: Pointwise `q(ex)` does not globally entail `q ⊨ p`.

### Logical Tests (4/4 rejected)
The specification does not allow unintended logical inferences:
- **Entailment ≠ validity**: `spec.entails(X)` does not imply `valid(spec)`.
- **Soundness**: The `implies_apply` axiom (modus ponens) does not introduce inconsistency — `false` is not derivable.
- **No converse**: Individual `spec ⊨ a_to_p(a0)` does not imply `a_to_p(a0) ⊨ spec`.
- **No validity lifting**: `spec.entails(tla_forall(a_to_p))` does not imply `valid(tla_forall(a_to_p))`.

## Conclusion

The specification for `spec_entails_tla_forall` is **tight and consistent**. All 12 adversarial queries were correctly rejected, indicating:
1. Preconditions are necessary and sufficient — invalid inputs are rejected.
2. Postconditions are precise — incorrect behaviors are rejected.
3. No unintended logical consequences are derivable — the `external_body` axiom for `implies_apply` does not introduce unsoundness.
