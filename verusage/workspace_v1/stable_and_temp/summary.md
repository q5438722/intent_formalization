# Adversarial Test Results: `stable_and_temp.rs`

## Target Specification

The file defines temporal logic primitives (`Execution`, `TempPred`, `always`, `stable`, `valid`) and proves `stable_and_temp`: if `p` and `q` are both stable predicates, then `p ∧ q` is also stable. An axiom `stable_unfold` (external_body) unfolds the definition of stability.

## Test Results Summary

| # | Category | Test Name | Expected | Actual | Status |
|---|----------|-----------|----------|--------|--------|
| 1 | Boundary | `test_boundary_missing_stable_q` | FAIL | postcondition not satisfied | ✅ |
| 2 | Boundary | `test_boundary_no_preconditions` | FAIL | postcondition not satisfied | ✅ |
| 3 | Boundary | `test_boundary_unfold_without_stable` | FAIL | precondition not satisfied | ✅ |
| 4 | Behavioral | `test_mutation_stable_to_always` | FAIL | postcondition not satisfied | ✅ |
| 5 | Behavioral | `test_mutation_and_to_implies` | FAIL | postcondition not satisfied | ✅ |
| 6 | Behavioral | `test_mutation_unfold_unconditional` | FAIL | postcondition not satisfied | ✅ |
| 7 | Logical | `test_logical_stable_does_not_imply_valid` | FAIL | postcondition not satisfied | ✅ |
| 8 | Logical | `test_logical_any_pred_is_stable` | FAIL | postcondition not satisfied | ✅ |
| 9 | Logical | `test_logical_stable_conjunction_decomposition` | FAIL | postcondition not satisfied | ✅ |

**Result: 9/9 tests correctly rejected.**

## Analysis by Category

### Boundary Tests (3/3 rejected ✅)
- **Missing one precondition**: The spec correctly requires *both* `valid(stable(p))` and `valid(stable(q))` — dropping one breaks the proof.
- **Missing both preconditions**: Cannot derive stability of a conjunction from nothing.
- **Axiom precondition enforcement**: `stable_unfold` correctly demands `stable(p).satisfied_by(ex)` and rejects calls without it.

### Behavioral Mutation Tests (3/3 rejected ✅)
- **stable → always mutation**: The spec correctly distinguishes conditional persistence (stable) from universal truth (always). `valid(stable(p)) ∧ valid(stable(q))` does NOT imply `valid(always(p ∧ q))`.
- **∧ → ⟹ combinator mutation**: Stability of `p` and `q` individually does NOT imply stability of `p ⟹ q`. Counterexample: when neither holds, `p⟹q` is vacuously true, but at a later point `p` may hold without `q`.
- **Removing implication guard**: `stable(p).satisfied_by(ex)` gives `p(ex) ⟹ ∀i. p(ex.suffix(i))`, NOT `∀i. p(ex.suffix(i))` unconditionally.

### Logical Tests (3/3 rejected ✅)
- **Stable ≠ valid**: A predicate that never holds is trivially stable; stability does NOT imply the predicate holds everywhere.
- **Not every predicate is stable**: Arbitrary predicates can hold transiently; the spec does not admit universal stability.
- **Conjunction decomposition**: `stable(p ∧ q)` does NOT decompose into `stable(p)`. Stability of the conjunction only guarantees persistence when *both* hold simultaneously.

## Conclusion

The specification for `stable_and_temp` is **consistent** with respect to all 9 adversarial queries. It correctly:
1. **Rejects invalid inputs** — preconditions are necessary and enforced.
2. **Rejects incorrect behaviors** — mutated conclusions are not derivable.
3. **Rejects unintended reasoning** — properties outside the semantic boundary are not entailed.

No specification weaknesses were detected.
