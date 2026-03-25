# Adversarial Test Summary: `always_to_current.rs`

## Target Specification

The spec proves `always(p) → p` for temporal predicates over executions,
using an external-body `execution_equality` axiom (pointwise ⟹ structural equality).

## Results: All 9 tests FAILED verification ✅

All adversarial queries were correctly rejected — the specification does not entail
any of the unintended properties we tested.

---

### Boundary Tests (3/3 rejected)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_boundary_no_precondition` | `p.satisfied_by(ex)` from nothing | ❌ FAILED (postcondition) |
| 2 | `test_boundary_single_suffix` | `p(ex.suffix(1)) → p(ex)` | ❌ FAILED (postcondition) |
| 3 | `test_boundary_equality_missing_precondition` | `execution_equality` without pointwise eq | ❌ FAILED (precondition) |

**Interpretation**: The spec properly guards its preconditions. No invalid inputs are accepted.
The `always` quantifier cannot be weakened to a single suffix, and `execution_equality`
cannot be invoked without proving pointwise agreement.

---

### Behavioral Mutation Tests (3/3 rejected)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 4 | `test_mutation_converse` | `p(ex) → always(p)(ex)` (converse) | ❌ FAILED (postcondition) |
| 5 | `test_mutation_negated_output` | `always(p)(ex) → ¬p(ex)` (negated) | ❌ FAILED (postcondition) |
| 6 | `test_mutation_cross_execution` | `always(p)(ex1) → p(ex2)` (wrong execution) | ❌ FAILED (postcondition) |

**Interpretation**: The spec correctly distinguishes direction (current ≠ always),
rejects contradictory outputs (always ∧ ¬current), and does not confuse distinct
executions. The behavioral semantics are tight.

---

### Logical Tests (3/3 rejected)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 7 | `test_logical_execution_uniqueness` | `always(p)(ex1) ∧ always(p)(ex2) → ex1 = ex2` | ❌ FAILED (postcondition) |
| 8 | `test_logical_suffix_not_identity` | `ex.suffix(1) = ex` | ❌ FAILED (postcondition) |
| 9 | `test_logical_predicate_uniqueness` | `always(p)(ex) ∧ always(q)(ex) → p = q` | ❌ FAILED (postcondition) |

**Interpretation**: The spec does not support unintended logical inferences.
It correctly avoids: execution determinism under `always`, collapsing suffix
to identity, and equating distinct predicates that happen to hold on the same execution.

---

## Conclusion

The specification for `always_to_current` is **consistent** with respect to all
9 adversarial queries across boundary, behavioral, and logical dimensions.
No unintended entailments were discovered. The spec is neither too weak
(it rejects invalid inputs and incorrect behaviors) nor logically overpowered
(it does not support spurious reasoning).
