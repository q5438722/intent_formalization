# Adversarial Proof Test Results: `next_preserves_inv_rec`

## Target Specification

The function `next_preserves_inv_rec<T>` proves temporal induction: if an invariant `inv` holds on execution `ex`, the step predicate `next` holds at every suffix, and `inv` is preserved by `next`, then `inv` holds at any suffix position `i`.

## Results Summary

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary | 4 | 4 ✅ | 0 |
| Behavioral Mutation | 4 | 4 ✅ | 0 |
| Logical | 4 | 4 ✅ | 0 |
| **Total** | **12** | **12 ✅** | **0** |

**Verdict: The specification is consistent.** All adversarial queries were correctly rejected.

---

## Boundary Tests (4/4 FAILED as expected)

All tests attempted to call `next_preserves_inv_rec` with missing preconditions.

| Test | Missing Precondition | Verus Error |
|---|---|---|
| `test_boundary_missing_initial_inv` | `inv.satisfied_by(ex)` | precondition not satisfied |
| `test_boundary_missing_next` | `∀idx. next.satisfied_by(ex.suffix(idx))` | precondition not satisfied |
| `test_boundary_missing_preservation` | Inductive step (inv ∧ next → inv at idx+1) | precondition not satisfied |
| `test_boundary_no_preconditions` | All three preconditions | precondition not satisfied |

**Analysis:** Each precondition is independently necessary. The spec correctly requires all three for the induction to proceed: base case, universal next-step, and preservation.

---

## Behavioral Mutation Tests (4/4 FAILED as expected)

All tests used valid preconditions but asserted mutated conclusions.

| Test | Mutation | Verus Error |
|---|---|---|
| `test_mutation_negated_conclusion` | `ensures !inv.satisfied_by(ex.suffix(i))` | postcondition not satisfied |
| `test_mutation_wrong_execution` | `ensures inv.satisfied_by(ex2.suffix(i))` (different execution) | postcondition not satisfied |
| `test_mutation_contradictory_conjunction` | `ensures inv(...) && !next(...)` | postcondition not satisfied |
| `test_mutation_inverted_precondition` | Negated inv chain → positive inv conclusion | postcondition not satisfied |

**Analysis:** The spec correctly binds the conclusion to the specific execution `ex` and invariant `inv`. It does not allow negation of results, transfer to unrelated executions, or contradictory conjunctions.

---

## Logical Tests (4/4 FAILED as expected)

All tests queried properties not explicitly guaranteed by the specification.

| Test | Unintended Property | Verus Error |
|---|---|---|
| `test_logical_converse` | Backward induction: inv at suffix(i) → inv at ex | postcondition not satisfied |
| `test_logical_arbitrary_predicate` | Arbitrary predicate `p` holds at suffix(i) | postcondition not satisfied |
| `test_logical_determinism` | Two executions with same inv/next have equal states | postcondition not satisfied |
| `test_logical_soundness` | `ensures false` from consistent assumptions | postcondition not satisfied |

**Analysis:**
- **Converse:** The induction is strictly forward (0→1→2→...). Knowing `inv` at position `i` does not allow reasoning backward to position 0.
- **Arbitrary predicate:** The spec only speaks about `inv`, not unrelated predicates.
- **Determinism:** The spec tracks predicate satisfaction, not state identity. Multiple distinct executions can satisfy the same invariant.
- **Soundness:** The preconditions are satisfiable and do not collapse to `false`, confirming the spec is sound.

---

## Conclusion

The specification for `next_preserves_inv_rec` is **well-formed and consistent**:

1. **Precondition completeness:** All three preconditions (base case, universal next, inductive preservation) are individually necessary — removing any one prevents verification.
2. **Output precision:** The postcondition is tightly scoped to the given execution `ex` and invariant `inv` at position `i`. No mutations of these relationships are admitted.
3. **Logical tightness:** The spec does not entail backward induction, cross-execution generalization, state determinism, or arbitrary predicate satisfaction. It also does not entail `false` (soundness).

No weaknesses were found in the specification.
