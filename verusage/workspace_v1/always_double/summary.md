# Adversarial Test Summary: `always_double.rs`

## Target Specification

The target defines temporal logic primitives (`Execution`, `TempPred`, `always`) and three proof functions:
- `always_unfold`: `always(p) ⟹ ∀i. p(ex.suffix(i))`
- `always_propagate_forwards`: `always(p) at ex ⟹ always(p) at ex.suffix(i)`
- `always_double`: `always(p) ⟹ always(always(p))`

## Results: All 9 tests FAIL verification ✅

| Test ID | Category | Property Queried | Result |
|---------|----------|-----------------|--------|
| B1 | Boundary | `always_unfold` without precondition | FAIL ✅ |
| B2 | Boundary | `always_propagate_forwards` without precondition | FAIL ✅ |
| B3 | Boundary | `always_double` without precondition | FAIL ✅ |
| M1 | Behavioral Mutation | `p(ex) ⟹ always(p)(ex)` (reverse implication) | FAIL ✅ |
| M2 | Behavioral Mutation | `always(p)(ex) ⟹ always(p)(ex2)` (cross-execution transfer) | FAIL ✅ |
| M3 | Behavioral Mutation | `always(p)(ex) ⟹ ¬p(ex.suffix(0))` (negated output) | FAIL ✅ |
| L1 | Logical | 3 finite satisfactions ⟹ always (induction fallacy) | FAIL ✅ |
| L2 | Logical | `always(p)(ex.suffix(j)) ⟹ always(p)(ex)` (backward propagation) | FAIL ✅ |
| L3 | Logical | `always(p)(ex) ⟹ always(q)(ex)` (unrelated predicate) | FAIL ✅ |

## Interpretation

The specification is **well-constrained** across all three query dimensions:

1. **Boundary**: Preconditions are enforced — invalid inputs (missing `always(p)` assumption) are correctly rejected.
2. **Behavioral**: Incorrect behavioral relations (reverse implication, cross-execution transfer, negated output) are correctly rejected.
3. **Logical**: Unintended logical properties (finite-to-infinite generalization, backward time propagation, cross-predicate entailment) are correctly rejected.

**Conclusion**: No specification weaknesses detected. The `always` operator and its associated lemmas correctly constrain the semantic space — they entail only the intended properties and reject all tested adversarial queries.
