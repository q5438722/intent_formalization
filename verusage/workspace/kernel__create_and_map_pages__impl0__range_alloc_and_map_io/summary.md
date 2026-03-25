# Adversarial Proof Test Summary

**Target**: `kernel__create_and_map_pages__impl0__range_alloc_and_map_io.rs`

**Functions tested**: `page_ptr2page_index`, `page_index2page_ptr`, `usize2page_entry_perm`, `usize2page_entry`, `usize2pa`, `va_4k_valid`, `VaRange4K::index`, `create_entry_and_alloc_and_map_io`, `range_alloc_and_map_io`

---

## Results Overview

| Test File | Total Tests | Failed (expected) | Passed (unexpected) |
|-----------|------------|-------------------|---------------------|
| boundary_tests.rs | 12 | 12 | 0 |
| behavioral_mutation_tests.rs | 12 | 12 | 0 |
| logical_tests.rs | 12 | 12 | 0 |
| **Total** | **36** | **36** | **0** |

All 36 adversarial tests were correctly **rejected** by Verus verification.

---

## Boundary Tests (12/12 FAIL ✅)

| # | Test | Violation | Result |
|---|------|-----------|--------|
| 1 | `test_boundary_page_ptr_not_aligned` | ptr=1 not 4k-aligned | FAIL ✅ |
| 2 | `test_boundary_page_ptr_just_below_boundary` | ptr=0xFFF not aligned | FAIL ✅ |
| 3 | `test_boundary_page_index_at_max` | i=NUM_PAGES out of range | FAIL ✅ |
| 4 | `test_boundary_page_index_overflow` | i=usize::MAX overflow | FAIL ✅ |
| 5 | `test_boundary_va_zero_not_valid` | va=0 fails L4 index check | FAIL ✅ |
| 6 | `test_boundary_va_not_aligned` | va=1 not 4k-aligned | FAIL ✅ |
| 7 | `test_boundary_insufficient_quota_for_create_entry` | mem_4k=3 < 4 | FAIL ✅ |
| 8 | `test_boundary_zero_free_pages` | free_pages=0 < 4 | FAIL ✅ |
| 9 | `test_boundary_va_already_in_io_space` | target_va already mapped | FAIL ✅ |
| 10 | `test_boundary_range_alloc_insufficient_quota` | quota=39 < 4*10=40 | FAIL ✅ |
| 11 | `test_boundary_range_alloc_overflow_guard` | len*4 overflows usize | FAIL ✅ |
| 12 | `test_boundary_varange_index_at_len` | i=len out of bounds | FAIL ✅ |

---

## Behavioral Mutation Tests (12/12 FAIL ✅)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_zero_entry_perm_present_true` | Assert present=true for v=0 | FAIL ✅ |
| 2 | `test_mutation_zero_entry_addr_nonzero` | Assert addr!=0 for v=0 | FAIL ✅ |
| 3 | `test_mutation_usize2pa_wrong_value` | Assert usize2pa(0x1000)==0 | FAIL ✅ |
| 4 | `test_mutation_page_ptr2index_wrong` | Assert index(0x2000)==3 | FAIL ✅ |
| 5 | `test_mutation_page_index2ptr_wrong` | Assert ptr(2)==0x1000 | FAIL ✅ |
| 6 | `test_mutation_write_bit_false` | Assert write=false when bit set | FAIL ✅ |
| 7 | `test_mutation_execute_disable_false` | Assert exec_disable=false when bit 63 set | FAIL ✅ |
| 8 | `test_mutation_create_entry_returns_more_than_4` | Assert ret.0=5 allowed | FAIL ✅ |
| 9 | `test_mutation_quota_subtract_changes_mem_2m` | Assert mem_2m changes on subtract | FAIL ✅ |
| 10 | `test_mutation_io_space_grows_wrong_va` | Assert insert(target_va) adds other_va | FAIL ✅ |
| 11 | `test_mutation_present_implies_ps` | Assert present implies ps | FAIL ✅ |
| 12 | `test_mutation_roundtrip_off_by_one` | Assert roundtrip gives ptr+0x1000 | FAIL ✅ |

---

## Logical Tests (12/12 FAIL ✅)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_usize2pa_injective` | usize2pa is injective | FAIL ✅ |
| 2 | `test_logical_distinct_ptrs_same_index` | Distinct ptrs map to same index | FAIL ✅ |
| 3 | `test_logical_create_entry_stronger_bound` | ret.0 <= 3 (tighter than <= 4) | FAIL ✅ |
| 4 | `test_logical_quota_subtract_symmetric` | Quota subtraction is symmetric | FAIL ✅ |
| 5 | `test_logical_all_aligned_va_valid` | All aligned VAs are valid | FAIL ✅ |
| 6 | `test_logical_roundtrip_non_aligned` | Roundtrip works for non-aligned ptrs | FAIL ✅ |
| 7 | `test_logical_range_alloc_preserves_proc_dom` | proc_dom preserved (commented-out spec) | FAIL ✅ |
| 8 | `test_logical_mem_valid_any_value` | MEM_valid holds for any value | FAIL ✅ |
| 9 | `test_logical_create_entry_always_allocates_exactly_4` | Always exactly 4 pages | FAIL ✅ |
| 10 | `test_logical_quota_double_subtract_wrong` | Double subtract with wrong k | FAIL ✅ |
| 11 | `test_logical_io_space_insert_leaks` | Insert leaks to other VAs | FAIL ✅ |
| 12 | `test_logical_page_index_zero_gives_nonzero` | page_index2ptr(0)==1 | FAIL ✅ |

---

## Key Observations

1. **Boundary tests**: The specifications correctly reject all invalid inputs including non-aligned pointers, out-of-range indices, insufficient quotas, and overflow conditions.

2. **Behavioral mutation tests**: All mutated behaviors are correctly rejected, confirming the spec is precise enough for the utility functions (`usize2pa`, `page_ptr2page_index`, etc.) and constraint properties (`Quota::spec_subtract_mem_4k`).

3. **Logical tests**: The spec correctly rejects false claims about injectivity, symmetry, totality, and stronger bounds.

4. **Weakness identified**: `range_alloc_and_map_io` has an extremely weak postcondition — only `self.wf()`. Many important properties (proc_dom preservation, thread_dom preservation, IO space domain updates, page allocation tracking) are **commented out** in the source. This means the spec provides very little assurance beyond well-formedness. Test 7 in the logical suite was designed to probe this weakness, but since we can only test the spec's own claims (not the full kernel invariants from within a standalone test), we validated the structural weakness by showing that domain non-preservation is consistent with the active spec.

5. **No unexpected passes**: All 36 tests failed as intended, indicating the active specifications are semantically sound within their stated boundaries. The primary concern remains the intentionally weak postcondition of `range_alloc_and_map_io`.
