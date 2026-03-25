# Adversarial Proof Test Summary: `syscall_mmap`

## Target
`kernel__syscall_mmap__impl0__syscall_mmap.rs` — the `syscall_mmap` kernel syscall that maps virtual address ranges to freshly allocated physical pages.

## Results

All 15 adversarial tests **correctly fail verification**, confirming that the specification rejects these unintended properties.

### Boundary Tests (5/5 FAIL ✓)

| # | Test | Target | Result |
|---|------|--------|--------|
| 1 | `test_boundary_error_va_in_use_is_error` | Assert `ErrorVaInUse` is not an error | **FAIL** ✓ |
| 2 | `test_boundary_va_zero_not_valid` | Assert VA=0 is 4k-valid | **FAIL** ✓ |
| 3 | `test_boundary_unaligned_page_ptr` | Assert ptr=1 is a valid page pointer | **FAIL** ✓ |
| 4 | `test_boundary_page_index_at_max` | Assert index=NUM_PAGES is valid | **FAIL** ✓ |
| 5 | `test_boundary_error_no_quota_is_error` | Assert `ErrorNoQuota` is not an error | **FAIL** ✓ |

### Behavioral Mutation Tests (5/5 FAIL ✓)

| # | Test | Target | Result |
|---|------|--------|--------|
| 1 | `test_mutation_error_state_changed` | Assert state changes on error | **FAIL** ✓ |
| 2 | `test_mutation_quota_wrong_amount` | Assert wrong quota subtraction passes | **FAIL** ✓ |
| 3 | `test_mutation_other_proc_addr_space_changed` | Assert other proc's address space changed on success | **FAIL** ✓ |
| 4 | `test_mutation_thread_dom_changed` | Assert thread domain grew on success | **FAIL** ✓ |
| 5 | `test_mutation_endpoint_changed` | Assert endpoint changed on success | **FAIL** ✓ |

### Logical Tests (5/5 FAIL ✓)

| # | Test | Target | Result |
|---|------|--------|--------|
| 1 | `test_logical_mmap_determinism` | Assert two mmaps produce same physical pages | **FAIL** ✓ |
| 2 | `test_logical_physical_contiguity` | Assert allocated pages are physically contiguous | **FAIL** ✓ |
| 3 | `test_logical_zero_len_exact_noop` | Assert zero-length mmap preserves exact kernel state | **FAIL** ✓ |
| 4 | `test_logical_error_code_specificity` | Assert ret.error_code matches specific error variant | **FAIL** ✓ |
| 5 | `test_logical_free_pages_decrease_exact` | Assert free pages decrease by exactly va_range.len | **FAIL** ✓ |

## Interpretation

The specification correctly rejects all 15 adversarial queries:

- **Boundary**: Invalid inputs (zero VA, unaligned pointers, off-by-one indices, error classifications) are properly guarded.
- **Behavioral**: The spec correctly constrains error behavior (state preservation), quota subtraction, address space isolation, domain preservation, and endpoint immutability.
- **Logical**: The spec avoids over-constraining: it does not guarantee deterministic page allocation, physical contiguity, exact kernel equality on zero-length success, specific error code propagation, or exact free-page count relationship.

No specification weaknesses were detected in the tested semantic boundary.
