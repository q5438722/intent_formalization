# Adversarial Test Summary: `alloc_page_4k`

## Target
`allocator__page_allocator_spec_impl__impl2__alloc_page_4k.rs`

## Results

All **17 adversarial tests FAILED verification** as expected, meaning the specification correctly rejects all tested invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (5/5 failed ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_boundary_unaligned_ptr_not_valid` | Unaligned pointer (7) rejected by `page_ptr_valid` | FAIL ✅ |
| `test_boundary_out_of_range_ptr` | Out-of-range pointer (`NUM_PAGES*4096`) rejected | FAIL ✅ |
| `test_boundary_remove_from_empty_set` | Removing from empty set doesn't produce non-empty | FAIL ✅ |
| `test_boundary_off_by_one_alignment` | Off-by-one misalignment (0x1001) rejected | FAIL ✅ |
| `test_boundary_4k_valid_implies_2m_valid` | 4k-valid does NOT imply 2m-valid | FAIL ✅ |

### Behavioral Mutation Tests (6/6 failed ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_page_still_free` | Returned page still in free set (should be removed) | FAIL ✅ |
| `test_mutation_page_not_allocated` | Returned page NOT in allocated set (should be added) | FAIL ✅ |
| `test_mutation_free_2m_changed` | Phantom page in free_pages_2m (should be unchanged) | FAIL ✅ |
| `test_mutation_mapped_4k_gained` | Returned page in mapped set (should be unchanged) | FAIL ✅ |
| `test_mutation_free_list_length_unchanged` | Free list length unchanged (should decrease by 1) | FAIL ✅ |
| `test_mutation_ret_was_already_allocated` | Returned page was previously allocated (should be new) | FAIL ✅ |

### Logical Tests (6/6 failed ✅)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_determinism` | Two choices from same free set must be equal | FAIL ✅ |
| `test_logical_4k_implies_2m` | 4k-valid implies 2m-valid (stronger alignment) | FAIL ✅ |
| `test_logical_distinct_ptrs_same_index` | Distinct valid pointers map to same index | FAIL ✅ |
| `test_logical_remove_empties_larger_set` | Removing one of two elements empties the set | FAIL ✅ |
| `test_logical_spurious_allocation` | Third page appears in allocated set | FAIL ✅ |
| `test_logical_container_gains_page` | Container gains page it didn't own | FAIL ✅ |

## Conclusion

The `alloc_page_4k` specification is **consistent** with respect to all 17 tested adversarial properties:
- Invalid inputs are properly rejected by the preconditions and type constraints.
- Mutated behavioral outputs are correctly distinguished from valid postconditions.
- Unintended logical inferences (determinism, stronger inequalities, spurious side effects) are not entailed.

No spec weaknesses were detected.
