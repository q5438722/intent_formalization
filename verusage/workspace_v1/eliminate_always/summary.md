# Adversarial Test Summary: `eliminate_always.rs`

## Target Specification

The file defines a temporal logic framework with `Execution<T>`, `TempPred<T>`, `always`, `valid`, and three proof functions:
- **`implies_apply`**: Modus ponens for temporal predicates on a specific execution
- **`execution_equality`**: Extensional equality for executions
- **`eliminate_always`**: If `spec ⊨ □p` then `spec ⊨ p`

## Test Results

**All 9 adversarial tests FAILED verification as expected** — the specification correctly rejects all invalid properties.

| # | Category | Test Name | Failure Mode | Result |
|---|----------|-----------|-------------|--------|
| 1 | Boundary | `test_boundary_eliminate_always_no_precondition` | Missing `spec.entails(always(p))` precondition | ✅ REJECTED (precondition not satisfied) |
| 2 | Boundary | `test_boundary_implies_apply_missing_antecedent` | Missing `p.satisfied_by(ex)` precondition | ✅ REJECTED (precondition not satisfied) |
| 3 | Boundary | `test_boundary_execution_equality_no_precondition` | Missing extensional equality precondition | ✅ REJECTED (precondition not satisfied) |
| 4 | Behavioral | `test_mutation_reverse_eliminate_always` | Reverse direction: `spec⊨p ⇒ spec⊨□p` | ✅ REJECTED (postcondition not satisfied) |
| 5 | Behavioral | `test_mutation_entails_always_to_valid` | Strengthened output: `spec⊨□p ⇒ valid(p)` | ✅ REJECTED (postcondition not satisfied) |
| 6 | Behavioral | `test_mutation_wrong_predicate` | Wrong predicate: `spec⊨□p ⇒ spec⊨q` | ✅ REJECTED (postcondition not satisfied) |
| 7 | Logical | `test_logical_entails_symmetry` | Entails symmetry: `spec⊨p ⇒ p⊨spec` | ✅ REJECTED (postcondition not satisfied) |
| 8 | Logical | `test_logical_satisfaction_transfer` | Satisfaction transfer across executions | ✅ REJECTED (postcondition not satisfied) |
| 9 | Logical | `test_logical_determinism` | Determinism: same spec ⇒ same execution | ✅ REJECTED (postcondition not satisfied) |

## Conclusion

The specification for `eliminate_always` is **consistent** with respect to all tested adversarial queries:

1. **Boundary correctness**: All three preconditions (`eliminate_always`, `implies_apply`, `execution_equality`) are properly enforced — invalid inputs are rejected.
2. **Behavioral correctness**: The specification rejects mutated outputs including the unsound reverse direction, over-strengthened conclusions, and substitution of unrelated predicates.
3. **Logical correctness**: The specification does not admit unintended reasoning — it correctly prevents symmetry of entailment, satisfaction transfer between executions, and determinism assumptions.

No specification weaknesses were detected.
