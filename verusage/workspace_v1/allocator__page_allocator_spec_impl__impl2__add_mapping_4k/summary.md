# Test Execution Summary: `add_mapping_4k`

**Target**: `allocator__page_allocator_spec_impl__impl2__add_mapping_4k.rs`
**Function**: `PageAllocator::add_mapping_4k(&mut self, target_ptr, pcid, va)`

## Specification Under Test

The function adds a `(pcid, va)` mapping to an already-mapped 4k page. Key postconditions:
- `page_mappings(target_ptr) =~= old.page_mappings(target_ptr).insert((pcid, va))`
- `page_io_mappings`, `mapped_pages_*`, `free_pages_*`, `allocated_pages_*` all unchanged
- `wf()` preserved

---

## Results: All 15 tests FAILED verification ✅

This confirms the specification **correctly rejects** all tested invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (5/5 FAIL ✅)
| Test | Property Tested | Result |
|------|----------------|--------|
| `test_boundary_unaligned_page_ptr` | `page_ptr_valid(1)` — non-4k-aligned ptr rejected | FAIL ✅ |
| `test_boundary_page_index_at_limit` | `page_index_valid(NUM_PAGES)` — off-by-one rejected | FAIL ✅ |
| `test_boundary_non_512_aligned_index` | `page_index_2m_valid(1)` — non-512-aligned index rejected | FAIL ✅ |
| `test_boundary_4k_valid_not_2m_valid` | `page_ptr_2m_valid(0x1000)` — 4k ptr ≠ 2M ptr | FAIL ✅ |
| `test_boundary_2m_valid_not_1g_valid` | `page_ptr_1g_valid(0x200000)` — 2M ptr ≠ 1G ptr | FAIL ✅ |

### Behavioral Mutation Tests (5/5 FAIL ✅)
| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_mapping_not_inserted` | Assert inserted element is NOT in result set | FAIL ✅ |
| `test_mutation_old_mapping_removed` | Assert existing mapping was removed by insert | FAIL ✅ |
| `test_mutation_io_mappings_gained_element` | Assert unchanged io_mappings gained an element | FAIL ✅ |
| `test_mutation_mapped_pages_gained_page` | Assert unchanged mapped_pages gained a page | FAIL ✅ |
| `test_mutation_free_pages_gained_page` | Assert unchanged free_pages gained a page | FAIL ✅ |

### Logical Tests (5/5 FAIL ✅)
| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_valid_ptr_not_unique` | `page_ptr_valid(ptr) ⟹ ptr == 0` (uniqueness) | FAIL ✅ |
| `test_logical_mapping_count_up_by_two` | Insert increases len by 2, not 1 (stronger ineq.) | FAIL ✅ |
| `test_logical_roundtrip_breaks` | `page_index2ptr(ptr2index(4096)) ≠ 4096` (roundtrip) | FAIL ✅ |
| `test_logical_4k_valid_implies_2m_valid` | 4k-valid ⟹ 2m-valid (over-generalization) | FAIL ✅ |
| `test_logical_insert_empties_set` | Insert into non-empty set yields empty set | FAIL ✅ |

## Conclusion

The `add_mapping_4k` specification is **consistent** with respect to all 15 adversarial queries:
- **Boundary**: Invalid inputs (unaligned ptrs, out-of-range indices) are correctly rejected by the spec predicates.
- **Behavioral**: Incorrect mutations (missing inserts, spurious additions) are properly caught by `=~=` postconditions.
- **Logical**: Unintended inferences (uniqueness, stronger inequalities, over-generalizations) cannot be derived from the spec.
