# Test Summary: `merged_4k_to_2m` Specification Consistency

**Target**: `allocator__page_allocator_spec_impl__impl2__merged_4k_to_2m.rs`
**Function**: `PageAllocator::merged_4k_to_2m` — merges 512 contiguous Free4k pages into a single Free2m page.

---

## Results Overview

| Test File | Tests | All Failed (as expected) |
|-----------|-------|--------------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (5/5 rejected) |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5/5 rejected) |
| `logical_tests.rs` | 5 | ✅ Yes (5/5 rejected) |

**Total: 15/15 adversarial tests correctly rejected by the specification.**

---

## Boundary Tests

All boundary tests correctly failed, confirming the spec rejects invalid inputs:

| # | Test | Property Tested | Result |
|---|------|----------------|--------|
| 1 | `test_boundary_4k_not_2m_aligned` | `page_ptr_2m_valid` rejects 4k-only-aligned ptrs | ✅ FAIL |
| 2 | `test_boundary_index_not_512_aligned` | `page_index_2m_valid` rejects non-512-aligned index | ✅ FAIL |
| 3 | `test_boundary_index_at_num_pages` | `page_index_valid` rejects off-by-one at NUM_PAGES | ✅ FAIL |
| 4 | `test_boundary_merge_index_at_upper_bound` | `spec_page_index_merge_2m_vaild` rejects j == i+512 | ✅ FAIL |
| 5 | `test_boundary_unaligned_page_ptr` | `page_ptr_valid` rejects unaligned pointer | ✅ FAIL |

## Behavioral Mutation Tests

All mutation tests correctly failed, confirming the spec rejects incorrect postcondition mutations:

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_free4k_decrease_by_511` | 4k count decreases by 511 instead of 512 | ✅ FAIL |
| 2 | `test_mutation_free2m_increase_by_2` | 2m count increases by 2 instead of 1 | ✅ FAIL |
| 3 | `test_mutation_free1g_changed` | 1g count changes (should be unchanged) | ✅ FAIL |
| 4 | `test_mutation_allocated4k_gained_page` | Allocated 4k set gains a page (should be preserved) | ✅ FAIL |
| 5 | `test_mutation_allocated2m_lost_page` | Allocated 2m set loses a page (should be preserved) | ✅ FAIL |

## Logical Tests

All logical tests correctly failed, confirming the spec does not entail unintended properties:

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_4k_valid_not_implies_2m_valid` | 4k-valid → 2m-valid (false cross-level claim) | ✅ FAIL |
| 2 | `test_logical_truncate_not_injective` | truncate_2m is injective (it's not: truncate(0)==truncate(1)) | ✅ FAIL |
| 3 | `test_logical_2m_valid_not_implies_1g_valid` | 2m-valid → 1g-valid (false hierarchy claim) | ✅ FAIL |
| 4 | `test_logical_2m_ptr_not_unique` | Valid 2m pointer is unique/deterministic | ✅ FAIL |
| 5 | `test_logical_merge_does_not_empty_free4k` | Merge empties the free 4k list (only guaranteed: decreases by 512) | ✅ FAIL |

---

## Conclusion

The specification for `merged_4k_to_2m` correctly rejects all 15 adversarial queries across boundary violations, behavioral mutations, and unintended logical inferences. No specification weakness was detected — the spec appropriately constrains its semantic space for the tested properties.
