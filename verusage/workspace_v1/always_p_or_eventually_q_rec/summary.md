# Test Results Summary: `always_p_or_eventually_q_rec`

## Target Specification

The function proves that predicate `p` holds at any suffix position `i` of an execution, given:
- **R1**: Step implication: `p ∧ next → p' ∨ q'` at each step
- **R2**: `next` holds at every suffix
- **R3**: `q` never holds at any suffix
- **R4**: `p` holds at the initial state
- **Ensures**: `p.satisfied_by(ex.suffix(i))`

## Results Overview

| Category | Tests | All Failed (as expected) |
|---|---|---|
| Boundary | 4 | ✅ Yes |
| Behavioral Mutation | 3 | ✅ Yes |
| Logical | 3 | ✅ Yes |
| **Total** | **10** | **✅ 10/10** |

## Boundary Tests (4/4 rejected)

| Test | Violated Precondition | Verus Error |
|---|---|---|
| `test_boundary_1_missing_initial_p` | R4: `p.satisfied_by(ex)` is false | precondition not satisfied |
| `test_boundary_2_q_sometimes_true` | R3: `q` holds at suffix(3) | precondition not satisfied |
| `test_boundary_3_next_always_false` | R2: `next` is always false | precondition not satisfied |
| `test_boundary_4_step_implication_violated` | R1: step fails at idx=0 | precondition not satisfied |

All four preconditions are independently enforced. Invalid inputs are correctly rejected.

## Behavioral Mutation Tests (3/3 rejected)

| Test | Mutation | Verus Error |
|---|---|---|
| `test_mutation_1_negated_postcondition` | Assert `!p` at suffix(i) | assertion failed |
| `test_mutation_2_q_instead_of_p` | Assert `q` at suffix(i) | assertion failed |
| `test_mutation_3_wrong_relationship` | Assert `next ⟹ q` | assertion failed |

Incorrect output relations are rejected. The spec correctly distinguishes `p` from `q` and does not conflate the roles of `next` and `q`.

## Logical Tests (3/3 rejected)

| Test | Unintended Property | Verus Error |
|---|---|---|
| `test_logical_1_p_on_different_execution` | `p` holds on an unrelated execution | assertion failed |
| `test_logical_2_p_equiv_next` | `p ⟺ next` (predicate equivalence) | assertion failed |
| `test_logical_3_p_universal` | `∀ e. p.satisfied_by(e)` (global) | assertion failed |

The spec does not leak guarantees to unrelated executions, does not conflate distinct predicates, and does not universalize the conclusion beyond the specific execution.

## Conclusion

The specification of `always_p_or_eventually_q_rec` is **consistent** across all three query dimensions:
1. **Boundary**: All four preconditions are independently necessary and enforced.
2. **Behavioral**: Incorrect output mutations are rejected; the postcondition is tight.
3. **Logical**: No unintended reasoning is admitted; the guarantee is scoped to the specific execution and predicates.

No specification weaknesses were detected.
