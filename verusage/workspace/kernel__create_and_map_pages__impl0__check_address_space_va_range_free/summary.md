# Adversarial Test Results Summary

**Target**: `check_address_space_va_range_free` and related functions  
**Spec under test**: `address_space_range_free` — checks that no VA in a range is mapped in a process's address space.

## Results Overview

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 7 | ✅ 7/7 |
| `behavioral_mutation_tests.rs` | 5 | ✅ 5/5 |
| `logical_tests.rs` | 5 | ✅ 5/5 |
| **Total** | **17** | **✅ 17/17** |

All 17 adversarial tests were **correctly rejected** by Verus, meaning the specification is robust against these attack vectors.

---

## Boundary Tests (7/7 FAILED as expected)

| # | Test | Violated Precondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_boundary_invalid_proc_ptr_assert_free` | `proc_dom` membership | ✅ Rejected |
| 2 | `test_boundary_invalid_proc_ptr_assert_not_free` | `proc_dom` membership | ✅ Rejected |
| 3 | `test_boundary_invalid_va_range_assert_free` | `va_range.wf()` | ✅ Rejected |
| 4 | `test_boundary_kernel_not_wf_assert_free` | `kernel.wf()` | ✅ Rejected |
| 5 | `test_boundary_zero_len_assert_not_free` | Asserts vacuously-true is false | ✅ Rejected |
| 6 | `test_boundary_page_ptr_unaligned` | `ptr % 0x1000 == 0` | ✅ Rejected |
| 7 | `test_boundary_page_index_out_of_range` | `i < NUM_PAGES` | ✅ Rejected |

## Behavioral Mutation Tests (5/5 FAILED as expected)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_occupied_range_assert_free` | Flip: mapped VA → assert free | ✅ Rejected |
| 2 | `test_mutation_free_range_assert_not_free` | Flip: all unmapped → assert not free | ✅ Rejected |
| 3 | `test_mutation_last_element_mapped_assert_free` | Off-by-one: last mapped → assert free | ✅ Rejected |
| 4 | `test_mutation_resolve_mapped_va_returns_none` | Mapped VA → assert not in domain | ✅ Rejected |
| 5 | `test_mutation_half_range_implies_full` | Half free + half mapped → assert full free | ✅ Rejected |

## Logical Tests (5/5 FAILED as expected)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_free_range_cross_proc` | Free for proc1 ⇒ free for proc2 | ✅ Rejected |
| 2 | `test_logical_free_range_implies_empty_space` | Free range ⇒ empty address space | ✅ Rejected |
| 3 | `test_logical_single_va_free_implies_range_free` | Single VA free ⇒ full range free | ✅ Rejected |
| 4 | `test_logical_monotonicity_range_extension` | Free sub-range ⇒ free super-range | ✅ Rejected |
| 5 | `test_logical_free_ranges_imply_disjoint` | Two free ranges ⇒ disjoint | ✅ Rejected |

---

## Conclusion

The specification for `check_address_space_va_range_free` is **consistent** with respect to all 17 adversarial queries:

- **Boundary robustness**: Invalid inputs (wrong proc_ptr, missing `wf()`, edge values) do not allow proving any postcondition.
- **Behavioral correctness**: Mutated outputs (flipped results, off-by-one, partial checks) are rejected.
- **Logical soundness**: Unintended inferences (cross-process leakage, stronger-than-stated properties, incorrect monotonicity) are not entailed.

No specification weaknesses were found. The spec correctly constrains the semantic boundary of `address_space_range_free`.
