# Test Execution Summary: `free_page_4k`

**Target**: `allocator__page_allocator_spec_impl__impl2__free_page_4k.rs`
**Function**: `PageAllocator::free_page_4k(&mut self, target_ptr, Tracked(target_perm))`

## Specification Under Test

The function frees an allocated 4k page, moving it from `allocated_pages_4k` to `free_pages_4k`. Key postconditions:
- `free_pages_4k() =~= old.free_pages_4k().insert(target_ptr)`
- `allocated_pages_4k() =~= old.allocated_pages_4k().remove(target_ptr)`
- All 2m/1g free, allocated, and mapped page sets are unchanged
- Container map domains are preserved
- Mapped page mappings and io_mappings are preserved
- `wf()` is maintained

---

## Results: All 15 tests FAILED verification ✅

This confirms the specification **correctly rejects** all tested invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (5/5 FAIL ✅)
| Test | Property Tested | Result |
|------|----------------|--------|
| `test_boundary_unaligned_page_ptr` | `page_ptr_valid(1)` — non-4k-aligned ptr rejected | FAIL ✅ |
| `test_boundary_page_index_at_limit` | `page_index_valid(NUM_PAGES)` — off-by-one rejected | FAIL ✅ |
| `test_boundary_max_usize_ptr` | `page_ptr_valid(usize::MAX)` — overflow boundary rejected | FAIL ✅ |
| `test_boundary_remove_nonexistent` | `remove(p)` on set not containing `p` — still no `p` | FAIL ✅ |
| `test_boundary_4k_ptr_not_2m_valid` | `page_ptr_2m_valid(0x1000)` — 4k ptr ≠ 2M ptr | FAIL ✅ |

### Behavioral Mutation Tests (5/5 FAIL ✅)
| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_freed_not_in_free_set` | Assert freed page is NOT in free set after insert | FAIL ✅ |
| `test_mutation_freed_still_allocated` | Assert freed page is still in allocated set after remove | FAIL ✅ |
| `test_mutation_2m_free_gained_page` | Assert unchanged free_pages_2m gained a page | FAIL ✅ |
| `test_mutation_mapped_pages_lost_page` | Assert unchanged mapped_pages lost a page | FAIL ✅ |
| `test_mutation_other_page_also_removed` | Assert another allocated page was also removed | FAIL ✅ |

### Logical Tests (5/5 FAIL ✅)
| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_valid_ptr_not_unique` | `page_ptr_valid(ptr) ⟹ ptr == 0` (false determinism) | FAIL ✅ |
| `test_logical_remove_empties_set` | Removing one element empties a multi-element set | FAIL ✅ |
| `test_logical_4k_valid_implies_2m_valid` | 4k-valid ⟹ 2m-valid (false over-generalization) | FAIL ✅ |
| `test_logical_roundtrip_breaks` | `page_index2ptr(ptr2index(4096)) ≠ 4096` (false negation) | FAIL ✅ |
| `test_logical_insert_empties_set` | Inserting into a set yields an empty set | FAIL ✅ |

## Conclusion

The `free_page_4k` specification is **consistent** with respect to all 15 adversarial queries:
- **Boundary**: Invalid inputs (unaligned ptrs, out-of-range indices, overflow values) are correctly rejected by the spec predicates.
- **Behavioral**: Incorrect mutations (freed page missing from free set, freed page still allocated, unchanged sets modified, collateral removal of other pages) are properly caught by `=~=` postconditions.
- **Logical**: Unintended inferences (pointer uniqueness/determinism, set emptying via remove/insert, over-generalization of alignment, roundtrip negation) cannot be derived from the spec.
