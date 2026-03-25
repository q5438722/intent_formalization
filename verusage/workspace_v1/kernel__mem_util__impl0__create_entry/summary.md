# Test Summary: `kernel__mem_util__impl0__create_entry`

## Target Function: `Kernel::create_entry`

Creates page table entries (L4/L3/L2) for a given process and virtual address, allocating up to 3 pages from the free pool and deducting from the container's memory quota.

## Results

| Test File | Tests | All Failed (as expected) |
|-----------|-------|------------------------|
| `boundary_tests.rs` | 7 | ✅ Yes (7 errors) |
| `behavioral_mutation_tests.rs` | 7 | ✅ Yes (7 errors) |
| `logical_tests.rs` | 7 | ✅ Yes (7 errors) |

**Total: 21/21 tests correctly rejected by the specification.**

---

## Boundary Tests (precondition violations)

| # | Test | Property Violated | Result |
|---|------|-------------------|--------|
| 1 | `test_boundary_va_zero_not_valid_4k` | va=0 has L4 index 0 < KERNEL_MEM_END_L4INDEX | FAIL ✅ |
| 2 | `test_boundary_va_unaligned_not_valid` | va=1 not 4k-aligned | FAIL ✅ |
| 3 | `test_boundary_page_ptr_max_invalid` | usize::MAX not aligned, out of range | FAIL ✅ |
| 4 | `test_boundary_page_index_at_num_pages` | Off-by-one: index == NUM_PAGES | FAIL ✅ |
| 5 | `test_boundary_quota_insufficient` | Quota mem_4k=2 < 3 required | FAIL ✅ |
| 6 | `test_boundary_present_entry_not_empty` | present=true violates is_empty() | FAIL ✅ |
| 7 | `test_boundary_2m_not_implies_4k` | 2m-valid VA is also 4k-valid | FAIL ✅ |

## Behavioral Mutation Tests (incorrect outputs)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_subtract_mem2m_changed` | mem_2m wrongly changed in subtraction | FAIL ✅ |
| 2 | `test_mutation_subtract_wrong_amount` | Wrong subtraction result (80 ≠ 90) | FAIL ✅ |
| 3 | `test_mutation_subtract_pcid_changed` | pcid wrongly changed in subtraction | FAIL ✅ |
| 4 | `test_mutation_subtract_ioid_changed` | ioid wrongly changed in subtraction | FAIL ✅ |
| 5 | `test_mutation_page_entry_nonzero_addr_empty` | Nonzero addr cannot be empty | FAIL ✅ |
| 6 | `test_mutation_set_mem4k_changes_mem2m` | set_mem_4k should not touch mem_2m | FAIL ✅ |
| 7 | `test_mutation_subtract_mem1g_changed` | mem_1g wrongly changed in subtraction | FAIL ✅ |

## Logical Tests (unintended reasoning)

| # | Test | Property Tested | Result |
|---|------|----------------|--------|
| 1 | `test_logical_subtract_deterministic` | Subtraction is deterministic (q2 == q3) | FAIL ✅ |
| 2 | `test_logical_valid_ptr_not_unique` | Multiple valid page ptrs exist | FAIL ✅ |
| 3 | `test_logical_roundtrip_preserved` | index↔ptr roundtrip is consistent | FAIL ✅ |
| 4 | `test_logical_zero_subtract_preserves` | k=0 subtraction preserves mem_4k | FAIL ✅ |
| 5 | `test_logical_subtract_preserves_mem1g` | Subtraction preserves mem_1g | FAIL ✅ |
| 6 | `test_logical_set_mem4k_preserves_pcid` | set_mem_4k preserves pcid field | FAIL ✅ |
| 7 | `test_logical_distinct_indices_same_va` | Distinct index tuples → distinct VAs | FAIL ✅ |

## Conclusion

The specification for `create_entry` correctly rejects all 21 adversarial queries across boundary, behavioral, and logical dimensions. No specification weaknesses were detected — invalid inputs are rejected, incorrect behaviors are caught, and the spec does not admit unintended logical inferences within the tested properties.
