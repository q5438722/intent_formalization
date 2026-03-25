# Adversarial Test Summary: `always_propagate_forwards`

**Target**: `source-projects/anvil-library/verified/temporal_logic/always_propagate_forwards.rs`

## Specification Under Test

- **`always_propagate_forwards<T>`**: If `always(p)` holds for execution `ex`, then `always(p)` also holds for `ex.suffix(i)`.
- **`always_unfold<T>`** (external_body): Unfolds `always(p).satisfied_by(ex)` into a universal quantifier over suffixes.
- **`execution_equality<T>`** (external_body): Two executions with pointwise-equal state functions are equal.

## Results

**All 12 adversarial tests FAILED verification as expected.** The specification correctly rejects all invalid properties.

### Boundary Tests (4/4 failed ✓)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_propagate_no_precondition` | Call `always_propagate_forwards` without `always(p).satisfied_by(ex)` | ✅ Rejected (precondition not satisfied) |
| `test_boundary_unfold_no_precondition` | Call `always_unfold` without `always(p).satisfied_by(ex)` | ✅ Rejected (precondition not satisfied) |
| `test_boundary_exec_equality_no_precondition` | Call `execution_equality` without pointwise equality | ✅ Rejected (precondition not satisfied) |
| `test_boundary_propagate_negated_precondition` | Call `always_propagate_forwards` when `always(p)` is explicitly false | ✅ Rejected (precondition not satisfied) |

### Behavioral Mutation Tests (4/4 failed ✓)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_wrong_execution` | Swap target execution: conclude `always(p)` on unrelated `ex2` | ✅ Rejected (postcondition not satisfied) |
| `test_mutation_wrong_predicate` | Swap predicate: conclude `always(q)` instead of `always(p)` | ✅ Rejected (postcondition not satisfied) |
| `test_mutation_negated_conclusion` | Negate conclusion: assert `!always(p).satisfied_by(ex.suffix(i))` | ✅ Rejected (postcondition not satisfied) |
| `test_mutation_wrong_suffix_index` | Wrong suffix: `always(p)` at `ex.suffix(i)` does NOT imply `always(p)` at `ex.suffix(j)` | ✅ Rejected (postcondition not satisfied) |

### Logical Tests (4/4 failed ✓)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_logical_converse_backward_propagation` | `always(p)` at a suffix does NOT imply `always(p)` at the original | ✅ Rejected (postcondition not satisfied) |
| `test_logical_single_point_to_always` | `p(ex)` does NOT imply `always(p)(ex)` | ✅ Rejected (postcondition not satisfied) |
| `test_logical_determinism` | Two executions satisfying `always(p)` need NOT be equal | ✅ Rejected (postcondition not satisfied) |
| `test_logical_predicate_conflation` | Two predicates always-satisfied by the same execution need NOT be equal | ✅ Rejected (postcondition not satisfied) |

## Conclusion

The specification for `always_propagate_forwards` is **consistent** with respect to all tested adversarial queries:

1. **Preconditions are enforced**: Invalid inputs (missing `always(p)`, missing pointwise equality) are properly rejected.
2. **Behavioral correctness is tight**: Mutations to the execution, predicate, conclusion sign, or suffix index are all rejected.
3. **No unintended logical entailments**: The spec does not admit backward propagation, single-point-to-always inflation, determinism, or predicate conflation.
