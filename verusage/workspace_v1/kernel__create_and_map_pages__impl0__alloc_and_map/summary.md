# Adversarial Proof Test Summary

**Target**: `kernel__create_and_map_pages__impl0__alloc_and_map.rs`
**Function under test**: `Kernel::alloc_and_map` — allocates a single 4K page and maps it into a process's address space.

## Results: All 15 tests FAILED verification ✅

All adversarial tests were correctly rejected by the specification, meaning the spec properly excludes the tested invalid inputs, incorrect behaviors, and unintended logical inferences.

---

### Boundary Tests (5/5 failed as expected)

| # | Test | Property Probed | Result |
|---|------|----------------|--------|
| 1 | `test_boundary_va_zero_not_4k_valid` | `spec_va_4k_valid(0)` — VA=0 has L4 index 0 < KERNEL_MEM_END_L4INDEX | FAIL ✅ |
| 2 | `test_boundary_unaligned_page_ptr_invalid` | `page_ptr_valid(1)` — ptr=1 is not 4096-aligned | FAIL ✅ |
| 3 | `test_boundary_page_index_at_num_pages` | `page_index_valid(NUM_PAGES)` — off-by-one at upper bound | FAIL ✅ |
| 4 | `test_boundary_va_max_usize_invalid` | `spec_va_4k_valid(usize::MAX)` — max value fails alignment | FAIL ✅ |
| 5 | `test_boundary_present_entry_not_empty` | `PageEntry{present:true}.is_empty()` — present entries aren't empty | FAIL ✅ |

### Behavioral Mutation Tests (5/5 failed as expected)

| # | Test | Property Probed | Result |
|---|------|----------------|--------|
| 1 | `test_mutation_quota_wrong_subtraction_amount` | `spec_subtract_mem_4k` rejects 100→98 with k=1 (should be 99) | FAIL ✅ |
| 2 | `test_mutation_quota_mem2m_changed` | `spec_subtract_mem_4k` rejects mem_2m changing from 50→49 | FAIL ✅ |
| 3 | `test_mutation_quota_mem1g_changed` | `spec_subtract_mem_4k` rejects mem_1g changing from 25→24 | FAIL ✅ |
| 4 | `test_mutation_quota_pcid_changed` | `spec_subtract_mem_4k` rejects pcid changing from 10→9 | FAIL ✅ |
| 5 | `test_mutation_page_entry_nonzero_addr_not_empty` | `PageEntry{addr:4096}.is_empty()` rejects non-zero addr | FAIL ✅ |

### Logical Tests (5/5 failed as expected)

| # | Test | Property Probed | Result |
|---|------|----------------|--------|
| 1 | `test_logical_subtract_deterministic` | `spec_subtract_mem_4k` is deterministic — two results must agree | FAIL ✅ |
| 2 | `test_logical_valid_ptr_not_unique` | `page_ptr_valid(ptr)` does not imply `ptr == 0` — multiple valid ptrs exist | FAIL ✅ |
| 3 | `test_logical_roundtrip_consistent` | `page_index2page_ptr(page_ptr2page_index(4096)) == 4096` — roundtrip holds | FAIL ✅ |
| 4 | `test_logical_zero_subtract_preserves` | `spec_subtract_mem_4k(q2, 0)` preserves mem_4k value | FAIL ✅ |
| 5 | `test_logical_subtract_preserves_mem1g` | `spec_subtract_mem_4k` does not alter mem_1g field | FAIL ✅ |

## Conclusion

The specification for `alloc_and_map` correctly rejects all 15 adversarial queries across the three consistency dimensions:
- **Boundary**: Invalid inputs (zero VA, unaligned pointers, out-of-range indices) are properly excluded by precondition specs.
- **Behavioral**: Mutated outputs (wrong subtraction amounts, changed unrelated quota fields) are properly rejected by the `spec_subtract_mem_4k` relation and `is_empty()` spec.
- **Logical**: Unintended inferences (non-determinism, uniqueness of valid pointers, broken roundtrips) are not derivable from the spec.

No spec weaknesses were detected by these tests.
