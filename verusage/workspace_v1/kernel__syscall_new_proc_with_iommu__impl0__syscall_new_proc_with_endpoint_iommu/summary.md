# Adversarial Proof Test Summary

**Target**: `kernel__syscall_new_proc_with_iommu__impl0__syscall_new_proc_with_endpoint_iommu.rs`

## Execution Results

| Test ID | Category | Description | Result | Expected |
|---------|----------|-------------|--------|----------|
| B1 | Boundary | `spec_va_4k_valid(0)` — VA=0 fails L4 index check | ❌ FAIL | FAIL ✓ |
| B2 | Boundary | `page_ptr_valid(1)` — unaligned pointer | ❌ FAIL | FAIL ✓ |
| B3 | Boundary | `page_index_valid(NUM_PAGES)` — index at upper bound | ❌ FAIL | FAIL ✓ |
| B4 | Boundary | `PageEntry{present:true}.is_empty()` — present entry | ❌ FAIL | FAIL ✓ |
| B5 | Boundary | `page_ptr_valid(NUM_PAGES*0x1000)` — pointer at max | ❌ FAIL | FAIL ✓ |
| M1 | Mutation | `spec_subtract_mem_4k` with wrong k (3 vs 5) | ❌ FAIL | FAIL ✓ |
| M2 | Mutation | `spec_subtract_mem_4k` with mutated `mem_2m` | ❌ FAIL | FAIL ✓ |
| M3 | Mutation | `PageEntry{addr:0x1000}.is_empty()` — nonzero addr | ❌ FAIL | FAIL ✓ |
| M4 | Mutation | Roundtrip `page_ptr→index` off-by-one (5→6) | ❌ FAIL | FAIL ✓ |
| M5 | Mutation | `spec_subtract_mem_4k` with mutated `ioid` | ❌ FAIL | FAIL ✓ |
| L1 | Logical | `!page_ptr_valid(0)` — zero ptr is actually valid | ❌ FAIL | FAIL ✓ |
| L2 | Logical | `!MEM_valid(0)` — zero addr is actually valid | ❌ FAIL | FAIL ✓ |
| L3 | Logical | Two valid VAs must differ (uniqueness not entailed) | ❌ FAIL | FAIL ✓ |
| L4 | Logical | `page_ptr_valid(ptr) ⟹ ptr ≥ 0x1000` (stronger bound) | ❌ FAIL | FAIL ✓ |
| L5 | Logical | `index2ptr(3) == index2ptr(7)` (non-injectivity) | ❌ FAIL | FAIL ✓ |

**Verification totals**: 43 verified (source specs), 15 errors (all test assertions correctly rejected)

## Analysis

### Spec Strengths (correctly rejected)
- **Boundary validity**: `spec_va_4k_valid`, `page_ptr_valid`, `page_index_valid`, and `PageEntry::is_empty` all correctly reject invalid inputs at their boundaries.
- **Behavioral integrity**: `Quota::spec_subtract_mem_4k` correctly constrains the subtraction semantics — wrong amounts, side-channel field mutations, and field corruption are all caught.
- **Roundtrip consistency**: `spec_page_ptr2page_index ∘ spec_page_index2page_ptr` is correctly bijective.
- **Injectivity**: `spec_page_index2page_ptr` correctly maps distinct indices to distinct pointers.

### Spec Weaknesses Identified

1. **Empty `ensures` on `syscall_new_proc_with_endpoint_iommu`** (CRITICAL):
   The top-level syscall function (line 2899) has **no postconditions**. This means callers receive zero guarantees about:
   - Whether `wf()` is preserved after the call
   - What the return value represents
   - Whether domains (proc, thread, endpoint) change correctly
   - Whether quota is decremented properly
   
   The inner function `new_proc_with_endpoint_iommu` has rich postconditions, but these are not surfaced to the syscall level.

2. **Zero pointer/address validity** (L1, L2, L4):
   Both `page_ptr_valid(0)` and `MEM_valid(0)` evaluate to `true`. In systems code, NULL/zero pointers are typically invalid. The spec does not exclude address 0, which could allow unintended null-pointer reasoning.

3. **No uniqueness guarantee for valid VAs** (L3):
   The spec does not establish that valid VAs form a specific structured set. Two arbitrary valid VAs could be equal. This is expected (valid VAs span a range), but means the spec cannot be used to infer distinctness without additional reasoning.

## Files Generated
- `boundary_tests.rs` — 5 boundary violation tests
- `mutation_tests.rs` — 5 behavioral mutation tests
- `logical_tests.rs` — 5 logical property tests
- `correctness_tests.rs` — combined file with all 15 tests
