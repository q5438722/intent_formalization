# Adversarial Test Summary: `seq_is_unique__endpoints_contain`

## Target Specification

**Function**: `endpoints_contain(endpoints, endpoint) -> present`
**Postcondition**: `present == abstractify_end_points(*endpoints).contains(endpoint@)`

**Helper**: `do_end_points_match(e1, e2) -> eq`
**Postcondition**: `eq == (e1@ == e2@)`

---

## Results Overview

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary | 3 | 3 ✅ | 0 |
| Behavioral Mutation | 3 | 3 ✅ | 0 |
| Logical | 3 | 3 ✅ | 0 |
| **Total** | **9** | **9** | **0** |

**All 9 adversarial tests were correctly rejected by the specification.**

---

## Boundary Tests (`boundary_tests.rs`)

| # | Test | Property Queried | Result |
|---|---|---|---|
| 1 | `test_boundary_empty_list_claims_present` | Empty list contains an endpoint | ❌ FAILED (correct) |
| 2 | `test_boundary_single_no_match_claims_present` | Non-matching single-element list contains target | ❌ FAILED (correct) |
| 3 | `test_boundary_multi_no_match_claims_present` | Multi-element list with no matches contains target | ❌ FAILED (correct) |

**Analysis**: The spec correctly rejects membership claims when the target is not in the list across all edge cases (empty, single-element, multi-element).

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

| # | Test | Property Queried | Result |
|---|---|---|---|
| 1 | `test_mutation_present_claims_absent` | Element in list → claim not found (flip result) | ❌ FAILED (correct) |
| 2 | `test_mutation_last_position_claims_absent` | Element at last index → claim not found | ❌ FAILED (correct) |
| 3 | `test_mutation_distinct_claim_match` | Distinct endpoints → claim they match | ❌ FAILED (correct) |

**Analysis**: The spec correctly rejects mutated outputs. Both `endpoints_contain` and `do_end_points_match` postconditions are strong enough to distinguish matching from non-matching endpoints at any position.

---

## Logical Tests (`logical_tests.rs`)

| # | Test | Property Queried | Result |
|---|---|---|---|
| 1 | `test_logical_must_be_at_index_zero` | Contains → element is at index 0 (too-strong positional claim) | ❌ FAILED (correct) |
| 2 | `test_logical_contains_implies_list_equality` | Same element in two lists → lists are equal (too-strong structural claim) | ❌ FAILED (correct) |
| 3 | `test_logical_contains_implies_all_equal` | Contains → all elements equal target (too-strong universality) | ❌ FAILED (correct) |

**Analysis**: The spec correctly refuses to entail over-strong logical properties. Membership (`contains`) does not leak positional information, does not create injective relationships between containers, and does not imply universality.

---

## Conclusion

The specification for `endpoints_contain` is **consistent** with respect to all tested semantic queries:

- **Boundary correctness**: Invalid inputs (empty lists, non-matching elements) are properly rejected.
- **Behavioral correctness**: Mutated outputs (flipped results, false matches) are properly rejected.
- **Logical soundness**: Unintended inferences (positional claims, structural equivalence, universal membership) are properly rejected.

No specification weaknesses were detected.
