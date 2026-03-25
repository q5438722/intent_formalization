# Test Execution Summary: alloc_and_map_4k

## Target
`allocator__page_allocator_spec_impl__impl2__alloc_and_map_4k.rs` — the `alloc_and_map_4k` function that pops a free 4k page, maps it with a `(pcid, va)` mapping, and assigns it to a container.

---

## Results Overview

| Test File | Total Tests | Failed (SHOULD FAIL) | Passed (Unexpected) |
|---|---|---|---|
| `boundary_tests.rs` | 5 | 5 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 7 | 7 ✅ | 0 |
| `logical_tests.rs` | 7 | 7 ✅ | 0 |
| **Total** | **19** | **19 ✅** | **0** |

All 19 adversarial tests were correctly **rejected** by the specification.

---

## Boundary Tests (5/5 FAILED as expected)

| # | Test | Property Violated | Result |
|---|---|---|---|
| 1 | `test_boundary_empty_free_list_provides_element` | Precondition: `free_pages_4k.len() > 0` — empty set cannot provide element | ✅ FAIL |
| 2 | `test_boundary_unaligned_page_ptr_as_ret` | `page_ptr_valid` rejects non-4k-aligned ptr (0x1001) | ✅ FAIL |
| 3 | `test_boundary_ptr_at_max_limit` | `page_ptr_valid` rejects ptr at `NUM_PAGES * 4096` boundary | ✅ FAIL |
| 4 | `test_boundary_empty_container_domain` | Precondition: `container_map_4k.dom().contains(c_ptr)` — empty domain | ✅ FAIL |
| 5 | `test_boundary_ptr_one_invalid` | `page_ptr_valid` rejects ptr=1 (not aligned) | ✅ FAIL |

## Behavioral Mutation Tests (7/7 FAILED as expected)

| # | Test | Postcondition Mutated | Result |
|---|---|---|---|
| 1 | `test_mutation_ret_still_free` | ret removed from free set → assert still present | ✅ FAIL |
| 2 | `test_mutation_mapped_unchanged` | ret inserted into mapped → assert not present | ✅ FAIL |
| 3 | `test_mutation_mapping_empty` | mapping = {(pcid,va)} → assert empty | ✅ FAIL |
| 4 | `test_mutation_io_mappings_nonempty` | io_mappings = {} → assert contains element | ✅ FAIL |
| 5 | `test_mutation_2m_free_changed` | free_pages_2m unchanged → assert element removed | ✅ FAIL |
| 6 | `test_mutation_container_pages_lost` | container pages grow by insert → assert old page lost | ✅ FAIL |
| 7 | `test_mutation_ret_was_allocated` | ret was free, not allocated → assert was allocated | ✅ FAIL |

## Logical Tests (7/7 FAILED as expected)

| # | Test | Unintended Property Tested | Result |
|---|---|---|---|
| 1 | `test_logical_determinism` | Two valid returns must be equal (determinism not guaranteed) | ✅ FAIL |
| 2 | `test_logical_free_count_decreased_by_two` | Free count decreased by 2 instead of 1 | ✅ FAIL |
| 3 | `test_logical_4k_valid_implies_2m_valid` | `page_ptr_valid ⇒ page_ptr_2m_valid` (not true) | ✅ FAIL |
| 4 | `test_logical_ret_is_zero` | ret must be 0 (structural assumption) | ✅ FAIL |
| 5 | `test_logical_total_pages_decreased` | Free+mapped total decreased by 2 (conservation violated) | ✅ FAIL |
| 6 | `test_logical_ret_in_wrong_category` | ret in mapped_pages_2m after 4k alloc (cross-category) | ✅ FAIL |
| 7 | `test_logical_mapping_has_two_elements` | Mapping set has 2 elements instead of 1 | ✅ FAIL |

---

## Conclusion

The specification for `alloc_and_map_4k` correctly rejects all 19 adversarial queries across boundary violations, behavioral mutations, and logical over-approximations. No specification weakness was detected by these tests — the spec properly constrains:

- **Input validity** (alignment, range, preconditions)
- **State transitions** (free→mapped, set membership updates)
- **Frame conditions** (unrelated page categories preserved)
- **Structural properties** (mapping contents, container ownership)
