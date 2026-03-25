# Adversarial Test Summary: `always_p_is_stable`

## Target Specification

The specification defines temporal logic primitives (`Execution`, `TempPred`, `always`, `stable`, `valid`) and proves that `always(p)` is a stable predicate: `valid(stable(always(p)))`.

Key axiom: `always_propagate_forwards` — if `always(p)` holds at an execution, it holds at any suffix.

---

## Results Overview

| # | File | Test | Type | Expected | Actual | Status |
|---|------|------|------|----------|--------|--------|
| 1 | boundary_tests.rs | `test_boundary_violate_precondition` | Precondition violation | FAIL | FAIL (precondition not satisfied) | ✅ |
| 2 | boundary_tests.rs | `test_boundary_always_from_single_point` | Edge case (finite info) | FAIL | FAIL (postcondition not satisfied) | ✅ |
| 3 | boundary_tests.rs | `test_boundary_stable_does_not_imply_p` | Reverse implication | FAIL | FAIL (postcondition not satisfied) | ✅ |
| 4 | behavioral_mutation_tests.rs | `test_mutation_always_does_not_imply_valid` | Output mutation | FAIL | FAIL (postcondition not satisfied) | ✅ |
| 5 | behavioral_mutation_tests.rs | `test_mutation_arbitrary_p_not_stable` | Overgeneralization | FAIL | FAIL (postcondition not satisfied) | ✅ |
| 6 | behavioral_mutation_tests.rs | `test_mutation_always_p_does_not_imply_always_q` | Predicate swap | FAIL | FAIL (postcondition not satisfied) | ✅ |
| 7 | logical_tests.rs | `test_logical_determinism` | Uniqueness/determinism | FAIL | FAIL (postcondition not satisfied) | ✅ |
| 8 | logical_tests.rs | `test_logical_implies_not_symmetric` | Symmetry of implies | FAIL | FAIL (postcondition not satisfied) | ✅ |
| 9 | logical_tests.rs | `test_logical_modus_ponens_incomplete` | Missing antecedent | FAIL | FAIL (postcondition not satisfied) | ✅ |

**Score: 9/9 tests correctly rejected.**

---

## Analysis

### Boundary Tests
All three boundary tests were correctly rejected:
- The spec properly enforces `always_propagate_forwards`'s precondition — cannot be called without proof that `always(p)` holds.
- `always(p)` cannot be concluded from `p` holding at a single point — the universal quantifier over suffixes is enforced.
- `stable(p)` (i.e., `p ⟹ always(p)`) does not imply `p` — the vacuous truth of the implication when `p` is false is correctly handled.

### Behavioral Mutation Tests
All three mutations were correctly rejected:
- `always(p)` on a single execution does not generalize to `valid(p)` (all executions) — the existential/universal distinction is sound.
- The theorem is specific to `always(p)` being stable; arbitrary predicates are not proven stable.
- Unrelated predicates cannot be substituted — `always(p)` gives no information about `always(q)`.

### Logical Tests
All three logical properties were correctly rejected:
- The spec does not imply determinism — multiple distinct executions can satisfy `always(p)`.
- Implication is correctly asymmetric — `p ⟹ q` does not entail `q ⟹ p`.
- Modus ponens requires both `always(p ⟹ q)` AND `always(p)` to conclude `always(q)` — omitting the antecedent is rejected.

---

## Conclusion

The specification for `always_p_is_stable` is **well-constrained**. It correctly rejects:
- **Invalid inputs** (boundary violations)
- **Incorrect behaviors** (mutated outputs and overgeneralizations)
- **Unintended reasoning** (determinism, symmetry, incomplete inference rules)

No specification weaknesses were found through these 9 adversarial tests. The `external_body` axiom `always_propagate_forwards` and the proven theorem `always_p_is_stable` together define a tight semantic boundary that does not admit unintended entailments.
