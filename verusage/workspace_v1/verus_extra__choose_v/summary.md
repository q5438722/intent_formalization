# Adversarial Test Results: `choose_smallest`

**Target**: `verus_extra__choose_v.rs` — `choose_smallest(low, high, p) -> res`

**Spec**:
- **requires**: ∃ i ∈ [low, high) such that p(i)
- **ensures**: low ≤ res < high, p(res), ∀ i ∈ [low, res): ¬p(i)

---

## Results Summary

| Category | Tests | All Failed? | Spec Weakness Found? |
|----------|-------|-------------|---------------------|
| Boundary | 4/4 FAIL | ✅ Yes | No |
| Behavioral Mutation | 4/4 FAIL | ✅ Yes | No |
| Logical | 4/4 FAIL | ✅ Yes | No |

**Overall: 12/12 adversarial tests correctly rejected. The specification is consistent.**

---

## Boundary Tests (4/4 FAIL ✅)

All precondition violations are correctly rejected:

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_empty_range` | low == high (empty range) | FAIL ✅ |
| 2 | `test_no_satisfying_element` | p always false | FAIL ✅ |
| 3 | `test_reversed_range` | low > high | FAIL ✅ |
| 4 | `test_satisfying_outside_range` | p(i) true only for i=20, range [0,10) | FAIL ✅ |

**Conclusion**: The `requires` clause correctly rejects all invalid input configurations.

## Behavioral Mutation Tests (4/4 FAIL ✅)

All mutated output assertions are correctly rejected:

| # | Test | Mutation | Result |
|---|------|---------|--------|
| 1 | `test_result_is_not_smallest` | Assert res ≠ 3 when smallest is 3 | FAIL ✅ |
| 2 | `test_result_does_not_satisfy_predicate` | Assert ¬p(res) | FAIL ✅ |
| 3 | `test_result_is_largest_not_smallest` | Assert res == 7 (largest, not smallest) | FAIL ✅ |
| 4 | `test_result_out_of_range` | Assert res ≥ 10 (out of range) | FAIL ✅ |

**Conclusion**: The `ensures` clause is precise enough to reject all tested behavioral mutations.

## Logical Tests (4/4 FAIL ✅)

All non-guaranteed properties are correctly rejected:

| # | Test | Unintended Property | Result |
|---|------|-------------------|--------|
| 1 | `test_result_always_equals_low` | res == low always | FAIL ✅ |
| 2 | `test_result_strictly_greater_than_low` | res > low always | FAIL ✅ |
| 3 | `test_result_is_unique_satisfier` | res is the only satisfier of p | FAIL ✅ |
| 4 | `test_all_above_result_satisfy_p` | ∀ i > res: p(i) | FAIL ✅ |

**Conclusion**: The specification does not over-constrain — it does not entail structural properties beyond what is explicitly stated (smallest satisfier in range).

---

## Overall Assessment

The specification for `choose_smallest` is **well-formed and consistent**:

1. **Preconditions are tight** — invalid inputs (empty ranges, unsatisfiable predicates, reversed bounds, out-of-range witnesses) are all rejected.
2. **Postconditions are precise** — the result is pinned to the exact smallest satisfier; mutations to value, predicate satisfaction, ordering, and range are all caught.
3. **No unintended entailments** — the spec does not accidentally imply uniqueness of satisfiers, monotonicity claims about p above the result, or fixed relationships between res and low.
