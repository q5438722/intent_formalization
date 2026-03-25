# Test Summary: `syscall_new_proc_with_endpoint_iommu`

## Overview

24 adversarial tests were generated across 3 categories targeting the `syscall_new_proc_with_endpoint_iommu` and `new_proc_with_endpoint_iommu` specifications. **All 24 tests failed verification as expected**, confirming the spec correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

## Results

| Category | Tests | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary Tests | 8 | 8 ✅ | 0 |
| Behavioral Mutation Tests | 8 | 8 ✅ | 0 |
| Logical Tests | 8 | 8 ✅ | 0 |

## Boundary Tests (8/8 failed ✅)

| # | Test | Property Queried |
|---|---|---|
| 1 | `test_boundary_thread_not_in_dom` | Thread not in `thread_dom` — precondition violation |
| 2 | `test_boundary_endpoint_index_out_of_bounds` | `endpoint_index >= MAX_NUM_ENDPOINT_DESCRIPTORS` |
| 3 | `test_boundary_kernel_not_wf` | Kernel not well-formed — cannot derive thread invariants |
| 4 | `test_boundary_va_range_not_wf` | `va_range` not well-formed — no overflow guarantee |
| 5 | `test_boundary_va_range_overflow` | `va_range.len * 3 + 3` overflow |
| 6 | `test_boundary_page_ptr_in_closure` | `page_ptr` already in page closure |
| 7 | `test_boundary_insufficient_quota` | Container quota < 2 |
| 8 | `test_boundary_same_page_ptrs` | `page_ptr_1 == page_ptr_2` |

## Behavioral Mutation Tests (8/8 failed ✅)

| # | Test | Mutation Applied |
|---|---|---|
| 1 | `test_mutation_wrong_pcid` | New proc has wrong pcid (42 vs. new_pcid) |
| 2 | `test_mutation_proc_dom_unchanged` | proc_dom unchanged after insert |
| 3 | `test_mutation_thread_dom_unchanged` | thread_dom unchanged after insert |
| 4 | `test_mutation_new_proc_empty_threads` | owned_threads is empty (should contain page_ptr_2) |
| 5 | `test_mutation_ioid_is_none` | ioid is None (spec says Some for iommu variant) |
| 6 | `test_mutation_noswitchnew_wrong_decision` | switch_decision is Switch (should be NoSwitch) |
| 7 | `test_mutation_noswitchnew_pcid_some` | pcid is Some (should be None) |
| 8 | `test_mutation_page_closure_missing_pages` | page_closure missing page_ptr_2 |

## Logical Tests (8/8 failed ✅)

| # | Test | Unintended Property Tested |
|---|---|---|
| 1 | `test_logical_syscall_always_succeeds` | Error implies not-error (contradiction) |
| 2 | `test_logical_pcid_equals_ioid` | pcid == ioid (no such guarantee) |
| 3 | `test_logical_new_proc_in_thread_dom` | proc ptr in thread_dom (cross-domain confusion) |
| 4 | `test_logical_quota_unchanged` | Quota unchanged after subtraction |
| 5 | `test_logical_endpoint_dom_changes` | endpoint_dom changes (spec says unchanged) |
| 6 | `test_logical_container_dom_changes` | container_dom changes (spec says unchanged) |
| 7 | `test_logical_wrong_subtract_amount` | Quota subtracted by 3 instead of 2 |
| 8 | `test_logical_noswitchnew_cr3_some` | cr3 is Some (should be None) |

## Conclusion

The specification for `syscall_new_proc_with_endpoint_iommu` and `new_proc_with_endpoint_iommu` is **consistent** with respect to all 24 queried properties:
- **Boundary integrity**: All preconditions are necessary; violating any leads to rejection.
- **Behavioral correctness**: Mutated outputs (wrong pcid, missing domains, wrong ioid) are all rejected.
- **Logical soundness**: Unintended inferences (cross-domain confusion, wrong subtraction amounts, false invariance) are all rejected.

**Notable observation**: `syscall_new_proc_with_endpoint_iommu` has an **empty `ensures` clause**, meaning it provides no postcondition guarantees at the Kernel level. All verifiable postconditions come from the inner `new_proc_with_endpoint_iommu` on `ProcessManager` and `range_create_and_share_mapping`.
