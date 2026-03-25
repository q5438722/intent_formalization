# Adversarial Proof Test Results

**Target**: `util__page_ptr_util_u__page_index2page_ptr.rs`
**Spec**: `spec_page_index2page_ptr(i) = (i * 4096) as usize` with precondition `0 <= i < NUM_PAGES` (NUM_PAGES = 2097152)

---

## Summary

| Category | Tests | All Failed (as expected) |
|---|---|---|
| Boundary | 4 | ✅ Yes |
| Behavioral Mutation | 5 | ✅ Yes |
| Logical | 5 | ✅ Yes |

**Total: 14/14 tests correctly rejected by Verus.**

---

## Boundary Tests (`boundary_tests.rs`) — 4/4 FAILED ✅

| Test | Property Asserted | Why It Should Fail |
|---|---|---|
| `boundary_test_at_limit` | `page_index_valid(NUM_PAGES)` | NUM_PAGES is exclusive upper bound |
| `boundary_test_beyond_limit` | `page_index_valid(2097153)` | Beyond valid range |
| `boundary_test_large_value` | `page_index_valid(4194304)` | 2× beyond valid range |
| `boundary_test_zero_index_nonzero_result` | `spec_page_index2page_ptr(0) != 0` | 0 × 4096 = 0 |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 5/5 FAILED ✅

| Test | Property Asserted | Why It Should Fail |
|---|---|---|
| `mutation_wrong_multiplier` | `result(1) == 4095` | Correct is 4096 |
| `mutation_off_by_one_high` | `result(1) == 4097` | Correct is 4096 |
| `mutation_wrong_base_case` | `result(0) == 1` | Correct is 0 |
| `mutation_conflate_distinct_inputs` | `result(1) == result(2)` | 4096 ≠ 8192 |
| `mutation_negate_correct_result` | `result(1) != 4096` | Result IS 4096 |

## Logical Tests (`logical_tests.rs`) — 5/5 FAILED ✅

| Test | Property Asserted | Why It Should Fail |
|---|---|---|
| `logical_always_positive` | `∀i. valid(i) ⟹ result(i) > 0` | Counterexample: i=0 → 0 |
| `logical_minimum_4096` | `result(0) >= 4096` | 0 < 4096 |
| `logical_result_bounded_by_num_pages` | `result(512) < NUM_PAGES` | 512×4096 = 2097152 = NUM_PAGES |
| `logical_stronger_monotonicity` | `result(1) > result(0) + 4096` | 4096 > 0+4096 is false (equal, not greater) |
| `logical_result_always_odd` | `result(1) % 2 == 1` | 4096 is even |

---

## Conclusion

The specification correctly rejects all 14 adversarial queries:
- **Boundary**: Invalid inputs outside `[0, NUM_PAGES)` are properly rejected.
- **Behavioral**: Mutated outputs (wrong multiplier, off-by-one, negation) are rejected.
- **Logical**: Unintended properties (always positive, minimum bound, range bound, strict monotonicity, parity) are not entailed.

**No spec weakness detected.** The specification is consistent with respect to all tested semantic boundaries.
