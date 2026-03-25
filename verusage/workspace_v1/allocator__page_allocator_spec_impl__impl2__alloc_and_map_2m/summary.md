# Test Execution Summary: `alloc_and_map_2m`

**Target**: `allocator__page_allocator_spec_impl__impl2__alloc_and_map_2m.rs`  
**Function**: `PageAllocator::alloc_and_map_2m(&mut self, pcid, va, c_ptr) -> PagePtr`

## Specification Under Test

**Preconditions:**
- `old(self).wf()` — allocator well-formed
- `old(self).free_pages_2m.len() > 0` — free 2m pages available
- `old(self).container_map_2m@.dom().contains(c_ptr)` — container exists

**Key Postconditions:**
- `ret` removed from `free_pages_2m`, inserted into `mapped_pages_2m`
- All other page sets (4k, 1g, allocated) unchanged
- `page_mappings(ret) = {(pcid, va)}`, `page_io_mappings(ret) = {}`
- Other mapped pages' mappings preserved

---

## Results

### Boundary Tests (5/5 FAILED ✅)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_empty_free_2m_list_provides_element` | Empty free set cannot provide a page | FAIL ✅ |
| 2 | `test_boundary_4k_aligned_not_2m_valid` | 4k-aligned ptr not 2m-valid | FAIL ✅ |
| 3 | `test_boundary_2m_ptr_at_max_limit` | Ptr at NUM_PAGES boundary rejected | FAIL ✅ |
| 4 | `test_boundary_empty_container_2m_domain` | Empty container domain has no elements | FAIL ✅ |
| 5 | `test_boundary_ptr_one_not_2m_valid` | Unaligned ptr (1) not 2m-valid | FAIL ✅ |

### Behavioral Mutation Tests (7/7 FAILED ✅)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_mutation_ret_still_in_free_2m` | ret removed from free set | FAIL ✅ |
| 2 | `test_mutation_mapped_2m_unchanged` | ret must be added to mapped set | FAIL ✅ |
| 3 | `test_mutation_mapping_empty` | Mapping for ret is non-empty | FAIL ✅ |
| 4 | `test_mutation_io_mappings_nonempty` | IO mappings for ret are empty | FAIL ✅ |
| 5 | `test_mutation_4k_free_changed` | 4k free pages unchanged by 2m alloc | FAIL ✅ |
| 6 | `test_mutation_ret_in_allocated_2m` | ret goes to mapped, not allocated | FAIL ✅ |
| 7 | `test_mutation_other_page_mapping_changed` | Other pages' mappings preserved | FAIL ✅ |

### Logical Tests (7/7 FAILED ✅)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_logical_determinism` | Return is nondeterministic | FAIL ✅ |
| 2 | `test_logical_free_count_decreased_by_two` | Free count decreases by 1, not 2 | FAIL ✅ |
| 3 | `test_logical_2m_valid_implies_1g_valid` | 2m-valid ≠ 1g-valid | FAIL ✅ |
| 4 | `test_logical_ret_is_zero` | ret is not a fixed value | FAIL ✅ |
| 5 | `test_logical_total_pages_decreased` | Total pages conserved, not lost | FAIL ✅ |
| 6 | `test_logical_ret_in_wrong_category` | ret not in 4k mapped (cross-category) | FAIL ✅ |
| 7 | `test_logical_mapping_has_two_elements` | Mapping has exactly 1 element | FAIL ✅ |

---

## Conclusion

**All 19 adversarial tests failed verification as expected.** The specification correctly rejects:

1. **Invalid inputs** — empty free lists, out-of-range pointers, missing container keys
2. **Incorrect behaviors** — mutated postconditions (wrong set membership, wrong mapping contents)
3. **Unintended reasoning** — determinism, stronger inequalities, cross-category misuse, structural assumptions

No specification weaknesses were detected. The `alloc_and_map_2m` spec appears consistent with respect to the queried semantic boundaries.
