# Adversarial Test Summary: `range_alloc_and_map_io`

**Target**: `kernel__create_and_map_pages__impl0__range_alloc_and_map_io.rs`

## Results Overview

| Test File | Tests | Failed (expected) | Passed (unexpected) |
|-----------|-------|--------------------|---------------------|
| `boundary_tests.rs` | 8 | 8 | 0 |
| `behavioral_mutation_tests.rs` | 8 | 8 | 0 |
| `logical_tests.rs` | 8 | 8 | 0 |
| **Total** | **24** | **24** | **0** |

All 24 adversarial tests were correctly rejected by the specification.

---

## Boundary Tests (8/8 FAILED ✓)

| # | Test | Violation | Result |
|---|------|-----------|--------|
| 1 | `test_boundary_va_zero_not_4k_valid` | VA=0 fails L4 index check | FAIL ✓ |
| 2 | `test_boundary_unaligned_ptr_invalid` | ptr=1 not 4K-aligned | FAIL ✓ |
| 3 | `test_boundary_page_index_off_by_one` | index=NUM_PAGES (off-by-one) | FAIL ✓ |
| 4 | `test_boundary_va_max_invalid` | VA=usize::MAX fails validity | FAIL ✓ |
| 5 | `test_boundary_ptr_not_2m_aligned` | 4K-aligned ptr rejected as 2M | FAIL ✓ |
| 6 | `test_boundary_page_index_2m_unaligned` | index=1 not 512-aligned | FAIL ✓ |
| 7 | `test_boundary_ptr_not_1g_aligned` | 2M-aligned ptr rejected as 1G | FAIL ✓ |
| 8 | `test_boundary_page_index_1g_unaligned` | index=512 not (512×512)-aligned | FAIL ✓ |

**Conclusion**: Boundary validation specs (`page_ptr_valid`, `page_index_valid`, `spec_va_4k_valid`, `page_ptr_2m_valid`, `page_index_2m_valid`, `page_ptr_1g_valid`, `page_index_1g_valid`) correctly reject invalid inputs at all tested boundaries.

---

## Behavioral Mutation Tests (8/8 FAILED ✓)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_subtract_mem2m_changed` | Mutated mem_2m in subtract result | FAIL ✓ |
| 2 | `test_mutation_subtract_wrong_result` | Wrong mem_4k after subtraction | FAIL ✓ |
| 3 | `test_mutation_subtract_pcid_changed` | pcid changed during subtraction | FAIL ✓ |
| 4 | `test_mutation_subtract_ioid_changed` | ioid changed during subtraction | FAIL ✓ |
| 5 | `test_mutation_subtract_mem1g_changed` | mem_1g changed during subtraction | FAIL ✓ |
| 6 | `test_mutation_nonempty_page_entry` | Non-zero addr entry as empty | FAIL ✓ |
| 7 | `test_mutation_write_entry_not_empty` | write=true entry as empty | FAIL ✓ |
| 8 | `test_mutation_user_entry_not_empty` | user=true entry as empty | FAIL ✓ |

**Conclusion**: `spec_subtract_mem_4k` correctly rejects mutations to any field (mem_4k, mem_2m, mem_1g, pcid, ioid). `PageEntry::is_empty()` correctly rejects entries with any non-default field.

---

## Logical Tests (8/8 FAILED ✓)

| # | Test | Property Tested | Result |
|---|------|-----------------|--------|
| 1 | `test_logical_subtract_deterministic` | Determinism of subtraction | FAIL ✓ |
| 2 | `test_logical_valid_ptr_not_unique` | Uniqueness of valid ptrs | FAIL ✓ |
| 3 | `test_logical_roundtrip_should_hold` | ptr↔index roundtrip integrity | FAIL ✓ |
| 4 | `test_logical_zero_subtract_preserves` | k=0 preserves value | FAIL ✓ |
| 5 | `test_logical_subtract_preserves_mem1g` | Subtraction field isolation | FAIL ✓ |
| 6 | `test_logical_range_alloc_proc_dom_weaker` | wf() alone constrains domains | FAIL ✓ |
| 7 | `test_logical_distinct_ptrs_distinct_index` | Index injectivity | FAIL ✓ |
| 8 | `test_logical_va_range_allows_zero_len` | VaRange allows len=0 | FAIL ✓ |

**Conclusion**: The specification correctly maintains logical consistency. Key observations:
- `spec_subtract_mem_4k` is deterministic (test 1)
- `page_ptr2page_index` / `page_index2page_ptr` form a valid roundtrip (test 3)
- `VaRange4K::wf()` permits zero-length ranges (test 8)

---

## Notable Spec Observation

**`range_alloc_and_map_io` has very weak postconditions**: The function's `ensures` clause only guarantees `self.wf()`. Most of the detailed postconditions (domain preservation, IO space updates, page tracking, quota accounting) are **commented out** in the source. This means:

- The spec allows `range_alloc_and_map_io` to arbitrarily modify process/thread/endpoint domains
- IO space updates are not guaranteed at the function level (only at the per-iteration loop invariant level via `create_entry_and_alloc_and_map_io`)
- Free page count changes are unspecified
- Quota accounting is unspecified in the postcondition

This is a significant spec incompleteness that could not be detected by these adversarial tests alone (since we test what the spec *does* entail, not what it *omits*), but it is documented here for reference.
