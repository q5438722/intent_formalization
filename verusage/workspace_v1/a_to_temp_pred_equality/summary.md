# Adversarial Test Results: `a_to_temp_pred_equality.rs`

## Target Specification

- **`temp_pred_equality<T>`** (axiom): mutual entailment (`p.entails(q) ∧ q.entails(p)`) → `p == q`
- **`a_to_temp_pred_equality<T, A>`**: lifts above to parametric families — `∀a. p(a) ⇔ q(a)` → `p == q` as functions

## Results Summary

**All 9 adversarial tests FAILED verification as expected.** The spec correctly rejects all invalid inputs, incorrect behaviors, and unintended reasoning.

| # | File | Test | Type | Result | Error |
|---|------|------|------|--------|-------|
| 1 | boundary_tests.rs | `test_boundary_contradictory_predicates` | Precondition violation (both) | ✅ FAIL | precondition `p.entails(q)` not satisfied |
| 2 | boundary_tests.rs | `test_boundary_one_direction_only` | Precondition violation (one side) | ✅ FAIL | precondition `q.entails(p)` not satisfied |
| 3 | boundary_tests.rs | `test_boundary_a_to_partial_condition` | Missing universal quantifier | ✅ FAIL | forall precondition not satisfied |
| 4 | behavioral_mutation_tests.rs | `test_mutation_strengthened_to_valid` | Strengthened conclusion | ✅ FAIL | `valid(p)` not entailed by `p == q` |
| 5 | behavioral_mutation_tests.rs | `test_mutation_equality_with_unrelated` | Unrelated equality | ✅ FAIL | `p == r` not entailed |
| 6 | behavioral_mutation_tests.rs | `test_mutation_entails_arbitrary` | Unwarranted entailment | ✅ FAIL | `p.entails(r)` not entailed |
| 7 | logical_tests.rs | `test_logical_one_direction_implies_equality` | One-directional → equality | ✅ FAIL | `p == q` not derivable from `p.entails(q)` alone |
| 8 | logical_tests.rs | `test_logical_soundness` | Axiom soundness | ✅ FAIL | `false` not derivable (axiom is sound) |
| 9 | logical_tests.rs | `test_logical_equality_without_lemma` | Equality without axiom | ✅ FAIL | `p == q` not derivable without calling lemma |

## Analysis

### Boundary (Tests 1–3)
The specification correctly requires **both** directions of entailment for `temp_pred_equality` and the **universal** quantifier for `a_to_temp_pred_equality`. Partial or missing preconditions are properly rejected.

### Behavioral Mutation (Tests 4–6)
The postcondition `p == q` is **tight** — it does not leak additional guarantees like `valid(p)`, equality with unrelated predicates, or entailment of arbitrary predicates.

### Logical (Tests 7–9)
- **One-directional entailment** does not imply equality (antisymmetry correctly requires both directions).
- The **axiom is sound**: `temp_pred_equality` does not introduce inconsistency (`false` is not derivable).
- **Extensional equality requires the axiom**: Verus cannot derive `p == q` from mutual entailment without the explicit `temp_pred_equality` lemma.

## Conclusion

The specification is **consistent** — it entails no unintended properties across all three adversarial dimensions tested. No weaknesses were found.
