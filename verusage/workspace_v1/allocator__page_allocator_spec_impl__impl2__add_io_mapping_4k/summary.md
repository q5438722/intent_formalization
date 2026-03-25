# Test Summary: `add_io_mapping_4k` Adversarial Proof Tests

**Target**: `allocator__page_allocator_spec_impl__impl2__add_io_mapping_4k.rs`

## Overview

15 adversarial tests were generated across 3 categories to probe the semantic boundary of the `add_io_mapping_4k` specification. All tests are designed to encode properties φ that should **NOT** be entailed by the specification, meaning they should all **FAIL** verification.

## Results

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary | 5 | 5 ✅ | 0 |
| Behavioral Mutation | 5 | 5 ✅ | 0 |
| Logical | 5 | 5 ✅ | 0 |
| **Total** | **15** | **15 ✅** | **0** |

**All 15 tests failed verification as expected.** The specification correctly rejects all tested undesirable properties.

---

## Boundary Tests (`boundary_tests.rs`)

These violate preconditions or use edge-case values relevant to `add_io_mapping_4k`.

| # | Test | Property Tested | Result |
|---|---|---|---|
| 1 | `test_boundary_unaligned_page_ptr` | `page_ptr_valid(1)` — non-4096-aligned ptr rejected | FAIL ✅ |
| 2 | `test_boundary_page_index_at_limit` | `page_index_valid(NUM_PAGES)` — off-by-one boundary rejected | FAIL ✅ |
| 3 | `test_boundary_page_ptr_at_max` | `page_ptr_valid(NUM_PAGES*4096)` — overflow boundary rejected | FAIL ✅ |
| 4 | `test_boundary_duplicate_io_mapping` | Duplicate `(ioid, va)` pair cannot be asserted absent from containing set | FAIL ✅ |
| 5 | `test_boundary_ref_count_at_max` | Combined ref count ≥ `usize::MAX` violates the `< usize::MAX` precondition | FAIL ✅ |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

These start from valid inputs and mutate expected outputs to check if incorrect behaviors are rejected.

| # | Test | Property Tested | Result |
|---|---|---|---|
| 1 | `test_mutation_io_mapping_not_inserted` | After insert, new `(ioid,va)` must be present (mutation: assert absent) | FAIL ✅ |
| 2 | `test_mutation_old_io_mapping_removed` | Existing IO mappings preserved after insert (mutation: assert removed) | FAIL ✅ |
| 3 | `test_mutation_page_mappings_gained_element` | Regular `page_mappings` unchanged (mutation: assert new element) | FAIL ✅ |
| 4 | `test_mutation_mapped_pages_gained_page` | `mapped_pages_4k` unchanged (mutation: assert new page) | FAIL ✅ |
| 5 | `test_mutation_free_pages_gained_page` | `free_pages_4k` unchanged (mutation: assert freed page) | FAIL ✅ |

## Logical Tests (`logical_tests.rs`)

These assert properties NOT explicitly guaranteed, testing for unintended over-constraining.

| # | Test | Property Tested | Result |
|---|---|---|---|
| 1 | `test_logical_valid_ptr_not_unique` | `page_ptr_valid(ptr)` does not imply `ptr == 0` (uniqueness) | FAIL ✅ |
| 2 | `test_logical_io_mapping_count_up_by_two` | IO mapping count increases by exactly 1, not 2 (stronger inequality) | FAIL ✅ |
| 3 | `test_logical_roundtrip_breaks` | `ptr→index→ptr` roundtrip holds for valid values (assert it breaks) | FAIL ✅ |
| 4 | `test_logical_4k_valid_implies_1g_valid` | `page_ptr_valid` does NOT imply `page_ptr_1g_valid` (cross-level) | FAIL ✅ |
| 5 | `test_logical_io_insert_empties_set` | Inserting into non-empty set cannot produce empty set | FAIL ✅ |

---

## Conclusion

The `add_io_mapping_4k` specification is **consistent** with respect to all 15 tested adversarial queries:

- **Boundary correctness**: Invalid inputs (unaligned pointers, out-of-range indices, duplicate mappings, overflow conditions) are properly rejected by the preconditions.
- **Behavioral correctness**: Incorrect output mutations (missing insertions, spurious removals, frame violations) are properly rejected by the postconditions.
- **Logical soundness**: Unentailed properties (uniqueness, stronger inequalities, cross-level implications, destructive set operations) are not derivable from the specification.

No specification weaknesses were detected in this test campaign.
