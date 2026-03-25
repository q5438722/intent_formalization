# Test Summary: `remove_mapping_4k_helper1`

## Target
`allocator__page_allocator_spec_impl__impl2__remove_mapping_4k_helper1.rs`

## Results Overview

| Category | Tests | All Failed (as expected) |
|---|---|---|
| Boundary Tests | 5 | ✅ Yes |
| Behavioral Mutation Tests | 5 | ✅ Yes |
| Logical Tests | 5 | ✅ Yes |
| **Total** | **15** | **15/15 rejected** |

All 15 adversarial tests were **correctly rejected** by Verus, indicating the specification is consistent with respect to the queried properties.

---

## Boundary Tests (`boundary_tests.rs`)

| # | Test | Violated Precondition | Result |
|---|---|---|---|
| 1 | `test_boundary_remove_absent_mapping` | `page_mappings(target).contains((pcid,va))` — mapping absent | ✅ FAIL |
| 2 | `test_boundary_ref_count_zero_with_mapping` | `ref_count == 1` — ref_count is 0 with non-empty mappings | ✅ FAIL |
| 3 | `test_boundary_unaligned_ptr_valid` | `page_ptr_valid(target_ptr)` — ptr=7 not 4k-aligned | ✅ FAIL |
| 4 | `test_boundary_ptr_overflow` | `page_ptr_valid(target_ptr)` — ptr exceeds address space | ✅ FAIL |
| 5 | `test_boundary_remove_from_empty` | `page_mappings` non-empty — removing from empty set | ✅ FAIL |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

| # | Test | Mutated Postcondition | Result |
|---|---|---|---|
| 1 | `test_mutation_mapping_still_present` | Mapping survives removal | ✅ FAIL |
| 2 | `test_mutation_other_mapping_lost` | Other mapping lost after removal | ✅ FAIL |
| 3 | `test_mutation_io_mappings_gained` | io_mappings gained a phantom element | ✅ FAIL |
| 4 | `test_mutation_allocated_pages_gained` | allocated_pages_4k gained a page | ✅ FAIL |
| 5 | `test_mutation_container_map_unchanged` | target_ptr still in container set | ✅ FAIL |

## Logical Tests (`logical_tests.rs`)

| # | Test | Unintended Property | Result |
|---|---|---|---|
| 1 | `test_logical_page_still_mapped_after_last_remove` | ref_count > 0 after removing last mapping (ref_count==1) | ✅ FAIL |
| 2 | `test_logical_distinct_ptrs_same_index` | Two distinct valid ptrs share same page index (injectivity violation) | ✅ FAIL |
| 3 | `test_logical_singleton_remove_nonempty` | Removing sole element from singleton yields non-empty set | ✅ FAIL |
| 4 | `test_logical_4k_implies_2m` | page_ptr_valid implies page_ptr_2m_valid (stronger alignment claim) | ✅ FAIL |
| 5 | `test_logical_remove_mapping_creates_io_mapping` | Removing a mapping causes io_mappings to grow from empty | ✅ FAIL |

---

## Conclusion

The specification for `remove_mapping_4k_helper1` correctly rejects all 15 adversarial queries:
- **Boundary**: Invalid inputs (unaligned pointers, wrong ref_count, absent mappings) are properly rejected.
- **Behavioral**: Incorrect post-states (mapping survival, side effects on io_mappings/allocated_pages) are properly rejected.
- **Logical**: Unwarranted inferences (stronger alignment, non-injectivity, impossible ref_count states) are properly rejected.

No spec weaknesses were detected by this test suite.
