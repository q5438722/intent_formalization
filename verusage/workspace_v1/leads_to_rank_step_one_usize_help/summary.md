# Test Execution Summary: `leads_to_rank_step_one_usize_help`

## Target Specification

The function `leads_to_rank_step_one_usize_help` proves by induction on `n: usize` that if each step leads to the previous one (`∀ n > 0: p(n) ~> p(n-1)`), then `p(n) ~> p(0)`.

- **Requires**: `∀ n: usize. n > 0 ⟹ spec.entails(p(n).leads_to(p(n-1)))`
- **Ensures**: `spec.entails(p(n).leads_to(p(0)))`

## Results Overview

| Test File | Tests | All Failed (as expected) |
|---|---|---|
| `boundary_tests.rs` | 3 | ✅ Yes |
| `behavioral_mutation_tests.rs` | 3 | ✅ Yes |
| `logical_tests.rs` | 3 | ✅ Yes |
| **Total** | **9** | **9/9 correctly rejected** |

## Boundary Tests (Precondition Violations)

| # | Test | Failure Mode | Result |
|---|---|---|---|
| 1 | `test_boundary_no_precondition` | Call without any precondition assumed | ✅ FAILED (precondition not satisfied) |
| 2 | `test_boundary_reversed_precondition` | Provide `p(n-1) ~> p(n)` instead of `p(n) ~> p(n-1)` | ✅ FAILED (precondition not satisfied) |
| 3 | `test_boundary_partial_precondition` | Only 2 specific steps (not universal ∀) | ✅ FAILED (precondition not satisfied) |

## Behavioral Mutation Tests (Incorrect Output Relations)

| # | Test | Failure Mode | Result |
|---|---|---|---|
| 1 | `test_mutation_upward_leads_to` | Assert `p(n) ~> p(n+1)` instead of `p(n) ~> p(0)` | ✅ FAILED (postcondition not satisfied) |
| 2 | `test_mutation_reversed_result` | Assert `p(0) ~> p(n)` instead of `p(n) ~> p(0)` | ✅ FAILED (postcondition not satisfied) |
| 3 | `test_mutation_valid_not_entailed` | Assert `valid(...)` instead of `spec.entails(...)` | ✅ FAILED (postcondition not satisfied) |

## Logical Tests (Unintended Reasoning)

| # | Test | Failure Mode | Result |
|---|---|---|---|
| 1 | `test_logical_different_spec` | Transfer result from `spec1` to unrelated `spec2` | ✅ FAILED (postcondition not satisfied) |
| 2 | `test_logical_leads_to_unrelated` | Derive `p(n) ~> q` for arbitrary unrelated `q` | ✅ FAILED (postcondition not satisfied) |
| 3 | `test_logical_cross_function` | Apply result for `p` to different function `q` | ✅ FAILED (postcondition not satisfied) |

## Conclusion

The specification of `leads_to_rank_step_one_usize_help` is **consistent** across all three categories of adversarial tests:

- **Boundary**: The universal quantifier precondition cannot be bypassed by omission, reversal, or partial instantiation.
- **Behavioral**: The postcondition precisely captures `p(n) ~> p(0)` and rejects mutations (wrong direction, wrong target, stronger claim).
- **Logical**: Results are properly scoped to the specific `spec`, specific predicate function `p`, and `entails` (not `valid`); no cross-spec, cross-function, or over-generalization leaks detected.

No specification weaknesses were found.
