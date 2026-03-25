# Test Execution Summary: `alloc_page_2m`

**Target**: `allocator__page_allocator_spec_impl__impl2__alloc_page_2m.rs`  
**Date**: 2026-03-21  
**Result**: All 15 adversarial tests **FAILED verification** as expected — the specification correctly rejects all tested unintended properties.

---

## Boundary Tests (5/5 FAILED ✅)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_boundary_4k_aligned_not_2m_valid` | 4k-aligned ptr (0x1000) is 2m-valid | FAIL ✅ |
| 2 | `test_boundary_unaligned_ptr` | Odd ptr (7) is page_ptr_valid | FAIL ✅ |
| 3 | `test_boundary_overflow_2m_ptr` | Out-of-range ptr is 2m-valid | FAIL ✅ |
| 4 | `test_boundary_remove_from_empty_set` | Removing from empty set yields non-empty | FAIL ✅ |
| 5 | `test_boundary_4k_valid_implies_2m_valid` | page_ptr_valid ⟹ page_ptr_2m_valid | FAIL ✅ |

**Assessment**: The spec properly distinguishes 4k/2m alignment and rejects out-of-range pointers.

---

## Behavioral Mutation Tests (5/5 FAILED ✅)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_mutation_page_still_free` | Allocated page remains in free set | FAIL ✅ |
| 2 | `test_mutation_page_not_allocated` | Allocated page absent from allocated set | FAIL ✅ |
| 3 | `test_mutation_free_4k_changed` | free_pages_4k gained a phantom element | FAIL ✅ |
| 4 | `test_mutation_mapped_2m_gained` | mapped_pages_2m gained the allocated page | FAIL ✅ |
| 5 | `test_mutation_allocated_1g_changed` | allocated_pages_1g gained a phantom element | FAIL ✅ |

**Assessment**: The spec correctly models set membership: `remove` excludes the element, `insert` includes it, and unaffected sets remain equal.

---

## Logical Tests (5/5 FAILED ✅)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_logical_determinism` | Two choices from same free set must be equal | FAIL ✅ |
| 2 | `test_logical_2m_implies_1g` | page_ptr_2m_valid ⟹ page_ptr_1g_valid | FAIL ✅ |
| 3 | `test_logical_remove_empties_larger_set` | Removing 1 from size-2 set yields empty | FAIL ✅ |
| 4 | `test_logical_distinct_ptrs_same_index` | Different 2m-valid ptrs have same index | FAIL ✅ |
| 5 | `test_logical_spurious_allocation` | Unrelated page appears in allocated set | FAIL ✅ |

**Assessment**: The spec does not entail determinism, does not conflate alignment levels, and maintains set cardinality correctly.

---

## Overall Conclusion

The `alloc_page_2m` specification is **consistent** with respect to all 15 tested adversarial properties:

- **Boundary**: Invalid inputs (misaligned, out-of-range, empty set) are rejected.
- **Behavioral**: Incorrect output mutations (wrong set membership) are rejected.
- **Logical**: Unintended inferences (determinism, stronger alignment, cardinality violations) are rejected.

No specification weaknesses were detected in this test suite.
