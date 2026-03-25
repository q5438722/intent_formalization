# Test Execution Summary: `eventually_propagate_backwards`

## Target Specification

The specification defines temporal logic primitives (`Execution`, `TempPred`, `eventually`) and proves
that `eventually(p)` propagates backwards through execution suffixes: if `eventually(p)` holds at
`ex.suffix(i)`, then it holds at `ex`. Two external-body axioms support this: `eventually_unfold`
(unpacks the existential) and `execution_equality` (functional extensionality for executions).

## Results: All 15 tests FAILED verification ✅

The specification correctly rejected every adversarial query. No weaknesses detected.

---

### Boundary Tests (5/5 failed as expected)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `boundary_test_unfold_missing_precondition` | Call `eventually_unfold` without `eventually(p)` holding | ❌ precondition not satisfied |
| 2 | `boundary_test_propagate_missing_precondition` | Call `eventually_propagate_backwards` without precondition | ❌ precondition not satisfied |
| 3 | `boundary_test_equality_missing_precondition` | Call `execution_equality` without pointwise equality | ❌ precondition not satisfied |
| 4 | `boundary_test_wrong_predicate` | Precondition on `q` but conclusion about `p` | ❌ precondition not satisfied |
| 5 | `boundary_test_wrong_execution` | Precondition on `ex2` but conclusion about `ex1` | ❌ precondition not satisfied |

### Behavioral Mutation Tests (5/5 failed as expected)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `behavioral_test_drop_eventually` | Strengthen: `eventually(p)` → `p` directly | ❌ postcondition not satisfied |
| 2 | `behavioral_test_p_at_suffix` | Mutate: claim `p` holds at `suffix(i)` | ❌ postcondition not satisfied |
| 3 | `behavioral_test_eventually_means_now` | Mutate: `eventually(p)` means `p` at `suffix(0)` | ❌ postcondition not satisfied |
| 4 | `behavioral_test_bounded_witness` | Mutate: witness bounded to index 0 or 1 | ❌ postcondition not satisfied |
| 5 | `behavioral_test_propagate_p_not_eventually` | Mutate: propagate `p` backwards (not `eventually(p)`) | ❌ postcondition not satisfied |

### Logical Tests (5/5 failed as expected)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `logical_test_unique_witness` | Determinism: witness index is unique | ❌ postcondition not satisfied |
| 2 | `logical_test_eventually_implies_now` | Collapse: `eventually(p)` entails `p` | ❌ postcondition not satisfied |
| 3 | `logical_test_universal_eventually` | Universality: `eventually(p)` holds for all `ex, p` | ❌ postcondition not satisfied |
| 4 | `logical_test_forward_propagation` | Forward prop: `eventually(p)` at `ex` implies at `ex.suffix(i)` | ❌ postcondition not satisfied |
| 5 | `logical_test_false_equality_via_axiom` | Axiom exploit: prove `const(0) == const(1)` via `execution_equality` | ❌ precondition not satisfied |

## Conclusion

The specification is **consistent** across all three query dimensions:
- **Boundary**: All preconditions are necessary and enforced; invalid inputs are rejected.
- **Behavioral**: The specification rejects strengthened, mutated, and bounded variants of its guarantees.
- **Logical**: No unintended properties (determinism, universality, forward propagation, axiom exploits) are entailed.
