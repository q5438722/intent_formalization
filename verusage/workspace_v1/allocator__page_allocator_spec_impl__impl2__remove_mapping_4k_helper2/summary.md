# Test Summary: `remove_mapping_4k_helper2`

## Overview

15 adversarial proof tests were generated across three categories to probe the semantic boundary of the `remove_mapping_4k_helper2` specification. **All 15 tests failed verification as expected**, confirming the specification correctly rejects invalid inputs, incorrect behaviors, and unintended logical reasoning.

---

## Boundary Tests (5/5 FAILED ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_unaligned_ptr_valid` | Non-4k-aligned pointer (7) asserted valid | FAIL ✅ |
| `test_boundary_ptr_overflow` | Pointer at NUM_PAGES*4096 asserted valid | FAIL ✅ |
| `test_boundary_ref_count_zero_with_mapping` | ref_count=0 with non-empty mappings | FAIL ✅ |
| `test_boundary_ref_count_two_remove_yields_empty` | ref_count=2: removing one of two mappings yields empty | FAIL ✅ |
| `test_boundary_remove_absent_mapping` | Removing non-member asserted to shrink set | FAIL ✅ |

**Conclusion**: The specification properly constrains pointer validity, ref_count consistency, and set membership preconditions.

---

## Behavioral Mutation Tests (5/5 FAILED ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_mutation_mapping_still_present` | Removed mapping asserted still present | FAIL ✅ |
| `test_mutation_io_mappings_gained` | Phantom element asserted in unchanged io_mappings | FAIL ✅ |
| `test_mutation_allocated_pages_4k_gained` | New page asserted in unchanged allocated_pages_4k | FAIL ✅ |
| `test_mutation_container_map_4k_target_not_removed` | target_ptr asserted still in container set after removal | FAIL ✅ |
| `test_mutation_container_map_2m_changed` | Key asserted missing from unchanged container_map_2m | FAIL ✅ |

**Conclusion**: The postconditions correctly constrain all output relations — mapping removal, set preservation, and container map updates are all properly specified.

---

## Logical Tests (5/5 FAILED ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_logical_page_still_mapped_after_last_remove` | After removing last mapping (ref_count 1→0), asserted ref_count > 0 | FAIL ✅ |
| `test_logical_distinct_ptrs_same_index` | Two distinct valid pointers asserted to have same page index | FAIL ✅ |
| `test_logical_singleton_remove_nonempty` | Removing sole element from singleton asserted non-empty | FAIL ✅ |
| `test_logical_4k_implies_2m` | 4k-valid pointer asserted to be 2M-valid | FAIL ✅ |
| `test_logical_remove_mapping_creates_io_mapping` | Empty io_mappings asserted non-empty after identity operation | FAIL ✅ |

**Conclusion**: The specification does not admit unintended logical inferences — injectivity of pointer indexing holds, set arithmetic is sound, and no unwarranted cross-property entailments are possible.

---

## Overall Assessment

The specification for `remove_mapping_4k_helper2` is **consistent** across all three query dimensions:
- **Boundary**: Invalid inputs are properly rejected by the precondition structure.
- **Behavioral**: Incorrect output relations are rejected by the postconditions.
- **Logical**: No unintended properties are entailed by the specification.

No specification weaknesses were detected in this round of adversarial testing.
