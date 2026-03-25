# Test Summary: `alloc_page_4k_for_new_container`

## Target
`allocator__page_allocator_spec_impl__impl2__alloc_page_4k_for_new_container.rs`

## Overview
Generated **20 adversarial proof tests** across 3 categories. **All 20 tests correctly FAIL verification**, confirming the specification rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (6 tests) — ✅ All FAIL as expected

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_unaligned_ptr_not_valid` | Odd value (7) violates 4k alignment | FAIL ✅ |
| 2 | `test_boundary_out_of_range_ptr` | Pointer at NUM_PAGES*4096 exceeds valid range | FAIL ✅ |
| 3 | `test_boundary_remove_from_empty_set` | Empty set removal cannot produce non-empty result | FAIL ✅ |
| 4 | `test_boundary_off_by_one_alignment` | 0x1001 is not 4k-aligned | FAIL ✅ |
| 5 | `test_boundary_map_insert_wrong_key` | Inserting key A does not make unrelated key B appear | FAIL ✅ |
| 6 | `test_boundary_zero_ptr_not_valid` | 0 is actually page_ptr_valid (lower boundary) | FAIL ✅ |

## Behavioral Mutation Tests (7 tests) — ✅ All FAIL as expected

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_page_still_free` | Claim ret.0 remains in free set after removal | FAIL ✅ |
| 2 | `test_mutation_page_not_allocated` | Claim ret.0 is NOT in allocated set after insertion | FAIL ✅ |
| 3 | `test_mutation_new_container_pages_nonempty` | Claim new container has non-empty owned pages | FAIL ✅ |
| 4 | `test_mutation_existing_container_pages_changed` | Claim existing container lost its pages | FAIL ✅ |
| 5 | `test_mutation_free_list_length_unchanged` | Claim free list length didn't decrease | FAIL ✅ |
| 6 | `test_mutation_container_map_missing_new_entry` | Claim ret.0 not in container_map domain | FAIL ✅ |
| 7 | `test_mutation_ret_page_now_mapped` | Claim allocation also mapped the page | FAIL ✅ |

## Logical Tests (7 tests) — ✅ All FAIL as expected

| # | Test | Property Tested | Result |
|---|------|----------------|--------|
| 1 | `test_logical_determinism` | Two free pages must be equal (determinism) | FAIL ✅ |
| 2 | `test_logical_4k_implies_2m` | 4k-valid implies 2m-valid (stronger alignment) | FAIL ✅ |
| 3 | `test_logical_distinct_ptrs_same_index` | Distinct valid pointers have same index (injectivity violation) | FAIL ✅ |
| 4 | `test_logical_remove_empties_larger_set` | Removing 1 from size-2 set empties it | FAIL ✅ |
| 5 | `test_logical_alloc_4k_page_in_2m_free` | Valid 4k pointer must be in free_2m set | FAIL ✅ |
| 6 | `test_logical_insert_clears_all_containers` | Inserting new container clears all existing ones | FAIL ✅ |
| 7 | `test_logical_valid_ptr_nonzero` | Valid pointer must be non-zero | FAIL ✅ |

## Conclusion

The specification for `alloc_page_4k_for_new_container` is **consistent** with respect to all 20 tested properties:

- **Boundary**: Invalid inputs (unaligned, out-of-range, empty-set operations) are correctly rejected.
- **Behavioral**: Mutated postconditions (page still free, not allocated, container pages modified) are correctly rejected.
- **Logical**: Unentailed properties (determinism, stronger alignment, injectivity violations, cross-set membership) are correctly rejected.

No spec weaknesses were detected by these adversarial tests.
