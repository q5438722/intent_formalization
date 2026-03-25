# Adversarial Test Results: `tla_forall_or_equality`

## Target Specification

The file defines a TLA-style temporal logic framework with:
- `tla_forall_unfold` (axiom): unfolds universal quantification given satisfaction evidence
- `temp_pred_equality` (axiom): derives structural equality from mutual entailment
- `tla_forall_or_equality` (theorem): `∀a. (P(a) ∨ Q) ≡ (∀a. P(a)) ∨ Q`

## Summary

| # | File | Test Name | Type | Result | Expected |
|---|------|-----------|------|--------|----------|
| 1 | boundary_tests.rs | `test_unfold_missing_precondition` | Boundary | ❌ FAIL (precondition) | ❌ FAIL |
| 2 | boundary_tests.rs | `test_equality_missing_both_preconditions` | Boundary | ❌ FAIL (precondition) | ❌ FAIL |
| 3 | boundary_tests.rs | `test_equality_missing_one_precondition` | Boundary | ❌ FAIL (precondition) | ❌ FAIL |
| 4 | behavioral_mutation_tests.rs | `test_drop_or_from_rhs` | Mutation | ❌ FAIL (assertion) | ❌ FAIL |
| 5 | behavioral_mutation_tests.rs | `test_swap_or_to_implies` | Mutation | ❌ FAIL (assertion) | ❌ FAIL |
| 6 | behavioral_mutation_tests.rs | `test_or_entails_left_disjunct` | Mutation | ❌ FAIL (assertion) | ❌ FAIL |
| 7 | logical_tests.rs | `test_valid_disjunction_implies_valid_left` | Logical | ❌ FAIL (assertion) | ❌ FAIL |
| 8 | logical_tests.rs | `test_extensional_equality_without_axiom` | Logical | ❌ FAIL (assertion) | ❌ FAIL |
| 9 | logical_tests.rs | `test_component_equality_from_forall` | Logical | ❌ FAIL (assertion) | ❌ FAIL |

**Result: 9/9 tests failed as expected.**

## Analysis

### Boundary Tests (Precondition Enforcement)
All three boundary tests confirm that the axioms' preconditions are properly enforced:
- **Test 1**: `tla_forall_unfold` cannot be invoked without proving the universal predicate is satisfied on the given execution.
- **Test 2**: `temp_pred_equality` cannot be invoked without establishing either direction of entailment.
- **Test 3**: `temp_pred_equality` cannot be invoked with only one direction — both `p ⊨ q` and `q ⊨ p` are required.

### Behavioral Mutation Tests (Output Correctness)
All three mutations of the theorem's postcondition are correctly rejected:
- **Test 4**: Dropping `.or(q)` from the RHS falsifies the equality — the forall-or cannot collapse to just forall.
- **Test 5**: Replacing `or` with `implies` in the RHS is rejected — `∀a.(P(a)∨Q)` does not entail `(∀a.P(a))⟹Q`.
- **Test 6**: The disjunction `(∀a.P(a))∨Q` does not entail the left disjunct `∀a.P(a)` alone.

### Logical Tests (Unintended Reasoning)
All three unintended logical properties are correctly rejected:
- **Test 7**: `valid(p.or(q))` does NOT imply `valid(p)` — the spec does not allow strengthening disjunctions.
- **Test 8**: Mutual entailment alone (without calling `temp_pred_equality`) does NOT yield structural equality `p == q`. The extensionality axiom must be explicitly invoked.
- **Test 9**: The forall-level equality theorem does NOT propagate to component-level equality (`P(a).or(Q) == P(a)` is not derivable for specific `a`).

## Conclusion

The specification is **consistent**: it correctly rejects all tested classes of invalid inputs, incorrect behaviors, and unintended logical inferences. The preconditions are tight, the postconditions resist mutation, and no unintended stronger properties are derivable from the axioms.
