# Test Results Summary: `remove_mapping_4k_helper3`

## Overview

15 adversarial proof tests were generated across 3 files to probe the semantic boundaries of the `remove_mapping_4k_helper3` specification. **All 15 tests failed verification as expected**, indicating the specification correctly rejects invalid inputs, incorrect behaviors, and unintended logical reasoning.

## Target Function

`remove_mapping_4k_helper3(&mut self, target_ptr: PagePtr, pcid: Pcid, va: VAddr)`

**Key preconditions:** `wf()`, `mapped_pages_4k().contains(target_ptr)`, `page_mappings(target_ptr).contains((pcid, va))`, `ref_count != 1`

**Key postconditions:** `wf()` preserved, allocated pages unchanged, mapping `(pcid, va)` removed from target, io_mappings unchanged, container maps unchanged, other pages' mappings preserved.

---

## Boundary Tests (5/5 FAILED ✅)

| # | Test | Violation | Result |
|---|------|-----------|--------|
| 1 | `test_boundary_remove_absent_mapping` | Mapping absent from set | FAIL ✅ |
| 2 | `test_boundary_ref_count_is_one` | ref_count == 1 (violates != 1) | FAIL ✅ |
| 3 | `test_boundary_unaligned_ptr_valid` | Non-4k-aligned pointer | FAIL ✅ |
| 4 | `test_boundary_ptr_overflow` | Pointer exceeds NUM_PAGES | FAIL ✅ |
| 5 | `test_boundary_remove_from_empty` | Empty mapping set | FAIL ✅ |

## Behavioral Mutation Tests (5/5 FAILED ✅)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_mapping_still_present` | Removed mapping survives | FAIL ✅ |
| 2 | `test_mutation_other_mapping_lost` | Other mapping also removed | FAIL ✅ |
| 3 | `test_mutation_io_mappings_gained` | io_mappings gained element | FAIL ✅ |
| 4 | `test_mutation_allocated_pages_gained` | allocated_pages grew | FAIL ✅ |
| 5 | `test_mutation_container_map_changed` | container_map lost target | FAIL ✅ |

## Logical Tests (5/5 FAILED ✅)

| # | Test | Unwarranted Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_ref_count_becomes_zero` | ref_count always becomes 0 | FAIL ✅ |
| 2 | `test_logical_distinct_ptrs_same_index` | Non-injective page indexing | FAIL ✅ |
| 3 | `test_logical_multi_element_remove_yields_empty` | Multi-element set becomes empty | FAIL ✅ |
| 4 | `test_logical_4k_implies_2m` | 4k-valid implies 2m-valid | FAIL ✅ |
| 5 | `test_logical_remove_increases_size` | Set grows after remove | FAIL ✅ |

## Conclusion

The specification for `remove_mapping_4k_helper3` is **consistent** with respect to all 15 tested properties. The spec correctly:
- Rejects invalid inputs (boundary violations)
- Rejects incorrect output mutations (behavioral mutations)
- Does not entail unwarranted logical properties (logical tests)

No specification weaknesses were detected.
