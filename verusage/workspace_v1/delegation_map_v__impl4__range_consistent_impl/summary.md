# Test Execution Summary: delegation_map_v__impl4__range_consistent_impl

## Overview
Generated 9 adversarial proof tests across 3 files targeting the `DelegationMap::range_consistent` specification. All tests correctly **FAIL verification**, confirming the specification rejects each invalid property.

## Results: All 9 tests FAIL as expected ✅

### Boundary Tests (`boundary_tests.rs`) — 3 errors, 3 verified
| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_range_consistent_nonempty_no_evidence` | Asserts range_consistent for non-empty range with no supporting evidence | ✅ FAIL |
| 2 | `test_between_end_as_lo` | Asserts `between(end, ki, hi)` is true when `end` is the maximum element | ✅ FAIL |
| 3 | `test_vacuous_range_consistent_derives_nothing` | Tries to derive `dm@[k] == dst@` from vacuously-true `range_consistent(end, hi, dst)` | ✅ FAIL |

### Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 3 errors, 3 verified
| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_range_consistent_wrong_dst` | Asserts consistency with `dst2` when only `dst1` is consistent and `dst1@ != dst2@` | ✅ FAIL |
| 2 | `test_range_consistent_implies_global` | Asserts `dm@[k] == dst@` for key `k` outside the consistent range | ✅ FAIL |
| 3 | `test_extend_with_gap` | Tries to stitch two sub-ranges with a gap (`mid1 < mid2`) into a full range | ✅ FAIL |

### Logical Tests (`logical_tests.rs`) — 3 errors, 3 verified
| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_hi_boundary_included` | Asserts `dm@[*hi.get()] == dst@` (hi is excluded from half-open interval `[lo, hi)`) | ✅ FAIL |
| 2 | `test_two_valid_maps_agree` | Asserts two independently valid DelegationMaps must map the same key identically | ✅ FAIL |
| 3 | `test_lt_cmp_inconsistency` | Asserts `KeyIterator::cmp_spec` and `KeyIterator::lt_spec` agree on `None` vs `Some` ordering | ✅ FAIL |

## Key Findings

1. **Specification is well-bounded for range_consistent**: The spec correctly rejects:
   - Unsubstantiated consistency claims (boundary test 1)
   - Wrong destination endpoints (behavioral test 1)
   - Out-of-range key conclusions (behavioral test 2)
   - Gap-based range extension (behavioral test 3)
   - Hi boundary inclusion (logical test 1)

2. **KeyIterator ordering inconsistency detected** (logical test 3):
   - `lt_spec`: `Some(x) < None` (end is maximum)
   - `cmp_spec`: `(Some(_), None) => Greater` (end is minimum)
   - These two definitions disagree, but `cmp_spec` on `KeyIterator` is unused in the verified code — `between` and all range logic use `lt_spec`. This is a latent spec defect that doesn't impact correctness but could cause issues if `cmp_spec` is relied upon in the future.

3. **No spec weaknesses found**: All 9 adversarial properties were correctly rejected. The specification does not allow unintended reasoning within the scope tested.
