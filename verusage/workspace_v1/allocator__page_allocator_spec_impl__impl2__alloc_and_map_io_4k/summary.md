# Adversarial Test Summary: `alloc_and_map_io_4k`

## Target
`allocator__page_allocator_spec_impl__impl2__alloc_and_map_io_4k.rs`

## Results Overview

| Test File | Total Tests | Failed (expected) | Passed (unexpected) |
|-----------|-------------|-------------------|---------------------|
| boundary_tests.rs | 5 | 5 | 0 |
| behavioral_mutation_tests.rs | 8 | 8 | 0 |
| logical_tests.rs | 7 | 7 | 0 |
| **Total** | **20** | **20** | **0** |

All 20 adversarial tests correctly **FAILED** verification, indicating the specification properly rejects invalid inputs, incorrect behaviors, and unintended logical reasoning.

---

## Boundary Tests (5/5 FAIL ✓)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_empty_free_list_provides_element` | Empty free set cannot yield a valid page | FAIL ✓ |
| 2 | `test_boundary_unaligned_page_ptr` | Non-4k-aligned ptr (0x1001) is not valid | FAIL ✓ |
| 3 | `test_boundary_ptr_at_max_boundary` | ptr = NUM_PAGES*4096 exceeds valid range | FAIL ✓ |
| 4 | `test_boundary_invalid_container_ptr` | c_ptr not in container domain is rejected | FAIL ✓ |
| 5 | `test_boundary_zero_ptr_invalid` | ptr=0 IS valid (0%0x1000==0, 0<NUM_PAGES); asserting invalid fails | FAIL ✓ |

## Behavioral Mutation Tests (8/8 FAIL ✓)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_ret_still_free` | ret remains in free set after removal | FAIL ✓ |
| 2 | `test_mutation_ret_not_mapped` | ret not in mapped set after insertion | FAIL ✓ |
| 3 | `test_mutation_io_mappings_empty` | IO mappings empty after alloc_and_map_io | FAIL ✓ |
| 4 | `test_mutation_regular_mappings_nonempty` | Regular mappings non-empty (should be empty) | FAIL ✓ |
| 5 | `test_mutation_free_count_unchanged` | Free count stays same (should decrease by 1) | FAIL ✓ |
| 6 | `test_mutation_allocated_contains_ret` | ret appears in allocated set (should be unchanged) | FAIL ✓ |
| 7 | `test_mutation_container_pages_unchanged` | Container owned pages unchanged for c_ptr | FAIL ✓ |
| 8 | `test_mutation_ret_was_already_mapped` | ret was already mapped before alloc | FAIL ✓ |

## Logical Tests (7/7 FAIL ✓)

| # | Test | Property Tested | Result |
|---|------|----------------|--------|
| 1 | `test_logical_determinism` | Same free set yields same ret (not guaranteed) | FAIL ✓ |
| 2 | `test_logical_ret_is_minimum` | ret is minimum ptr in free set (not guaranteed) | FAIL ✓ |
| 3 | `test_logical_4k_implies_2m_valid` | 4k-valid implies 2m-valid (false: alignment) | FAIL ✓ |
| 4 | `test_logical_double_remove_restores` | Double remove restores original set (false) | FAIL ✓ |
| 5 | `test_logical_io_mapping_globally_unique` | IO mapping (ioid,va) globally unique (not guaranteed) | FAIL ✓ |
| 6 | `test_logical_mapped_grows_by_two` | Mapped set grows by 2 elements (only 1) | FAIL ✓ |
| 7 | `test_logical_mapped_implies_was_mapped` | Being mapped now implies was mapped before (inverted) | FAIL ✓ |

---

## Conclusion

The specification for `alloc_and_map_io_4k` is **consistent** across all three test dimensions:

1. **Boundary correctness**: Invalid inputs (empty free list, unaligned pointers, out-of-range pointers, invalid containers) are properly constrained by preconditions.
2. **Behavioral correctness**: All postconditions correctly distinguish the IO-mapping variant — regular mappings are empty, IO mappings contain exactly `(ioid, va)`, free count decreases, container ownership updates correctly.
3. **Logical soundness**: The spec does not entail unintended properties such as determinism, ordering guarantees, cross-size-class validity, or global uniqueness of IO mappings.
