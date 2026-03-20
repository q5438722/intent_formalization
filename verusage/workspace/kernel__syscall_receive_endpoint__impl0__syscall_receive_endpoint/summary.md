# Specification Test Summary

## File Under Test
`kernel__syscall_receive_endpoint__impl0__syscall_receive_endpoint.rs`

An OS kernel syscall implementing IPC "receive endpoint" — a receiver thread either blocks waiting for a sender, or receives an endpoint from an existing sender. The file defines:
- `syscall_receive_endpoint`: main function with `requires`/`ensures`
- `syscall_receive_endpoint_fail` / `syscall_receive_endpoint_success`: spec functions describing failure/success behavior
- `is_thread_blocked` / `is_pass_endpoint_completed`: spec functions for state transitions
- Helper spec functions for page management, IPC payload, endpoint state, etc.

---

## Correctness Results (should all PASS)

**Result: 78 verified, 0 errors ✅**

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | test_is_error_for_error | spec_is_error true for Error | PASS | ✅ PASS |
| 2 | test_is_error_false_for_else | spec_is_error false for Else | PASS | ✅ PASS |
| 3 | test_is_error_false_for_success_usize | spec_is_error false for SuccessUsize | PASS | ✅ PASS |
| 4 | test_is_error_false_for_noquota | spec_is_error false for NoQuota | PASS | ✅ PASS |
| 5 | test_get_payload_as_endpoint_some | Endpoint variant returns Some | PASS | ✅ PASS |
| 6 | test_get_payload_as_endpoint_none_for_empty | Empty variant returns None | PASS | ✅ PASS |
| 7 | test_get_payload_as_endpoint_none_for_pagefault | PageFault returns None | PASS | ✅ PASS |
| 8 | test_get_payload_as_endpoint_none_for_pci | Pci returns None | PASS | ✅ PASS |
| 9 | test_page_ptr_to_index_page1 | 0x1000 → index 1 | PASS | ✅ PASS |
| 10 | test_page_ptr_to_index_page0 | 0 → index 0 | PASS | ✅ PASS |
| 11 | test_page_index_to_ptr_1 | index 1 → 0x1000 | PASS | ✅ PASS |
| 12 | test_page_index_to_ptr_0 | index 0 → 0 | PASS | ✅ PASS |
| 13 | test_page_ptr_valid_true | 0x1000 is valid page ptr | PASS | ✅ PASS |
| 14 | test_page_index_valid_true | 0, 1 are valid indices | PASS | ✅ PASS |
| 15 | test_page_index_valid_boundary | 2097151 (NUM_PAGES-1) is valid | PASS | ✅ PASS |
| 16 | test_page_index_2m_valid | 512, 0 are 2m-valid | PASS | ✅ PASS |
| 17 | test_page_index_1g_valid | 0 is 1g-valid | PASS | ✅ PASS |
| 18 | test_page_ptr_2m_valid | 0x200000 is 2m-valid ptr | PASS | ✅ PASS |
| 19 | test_endpoint_state_send_not_receive | SEND ≠ RECEIVE | PASS | ✅ PASS |
| 20 | test_thread_state_discrimination | All ThreadStates distinct | PASS | ✅ PASS |
| 21 | test_page_index_truncate_2m | Truncation arithmetic | PASS | ✅ PASS |
| 22 | test_page_entry_is_empty | Zero entry is empty | PASS | ✅ PASS |
| 23 | test_page_entry_not_empty | Present entry not empty | PASS | ✅ PASS |
| 24 | test_concrete_page_roundtrip | Ptr↔index roundtrip | PASS | ✅ PASS |
| 25 | test_payload_endpoint_zero | Endpoint(0) → Some(0) | PASS | ✅ PASS |
| 26 | test_payload_endpoint_max_valid | Endpoint(127) → Some(127) | PASS | ✅ PASS |
| 27 | test_is_error_false_for_error_no_quota | ErrorNoQuota is not "error" | PASS | ✅ PASS |
| 28 | test_is_error_false_for_error_va_in_use | ErrorVaInUse is not "error" | PASS | ✅ PASS |
| 29 | test_is_error_false_for_cpu_idle | CpuIdle is not "error" | PASS | ✅ PASS |
| 30 | test_param_is_error_implies_error_code | is_error ⟹ Error variant | PASS | ✅ PASS |
| 31 | test_param_syscall_wf_preserved | Postconditions preserve wf | PASS | ✅ PASS |
| 32 | test_syscall_return_error_struct | Struct fields match | PASS | ✅ PASS |
| 33 | test_syscall_return_else_struct | Else struct not error | PASS | ✅ PASS |
| 34 | test_param_page_ptr_valid_alignment | Valid ptr is aligned | PASS | ✅ PASS |
| 35 | test_param_page_ptr_valid_range | Valid ptr in range | PASS | ✅ PASS |

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations
**Result: 10 errors ✅ (all tests correctly rejected)**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | test_fail_page_ptr2page_index_unaligned | Unaligned ptr → wrong index | FAIL | ✅ FAIL |
| 2 | test_fail_page_index_invalid | Index = NUM_PAGES (out of range) | FAIL | ✅ FAIL |
| 3 | test_fail_page_ptr_valid_unaligned | 0x1001 not valid page ptr | FAIL | ✅ FAIL |
| 4 | test_fail_page_ptr_valid_too_large | usize::MAX not valid page ptr | FAIL | ✅ FAIL |
| 5 | test_fail_page_index_2m_not_aligned | Index 1 not 2m-aligned | FAIL | ✅ FAIL |
| 6 | test_fail_page_index_1g_not_aligned | Index 512 not 1g-aligned | FAIL | ✅ FAIL |
| 7 | test_fail_endpoint_index_out_of_range | 128 ≥ MAX (128) | FAIL | ✅ FAIL |
| 8 | test_fail_payload_index_out_of_range | 128 ≥ MAX (128) | FAIL | ✅ FAIL |
| 9 | test_fail_page_ptr_2m_valid_unaligned | 0x1000 not 2MB-aligned | FAIL | ✅ FAIL |
| 10 | test_fail_page_ptr_1g_valid_unaligned | 0x200000 not 1GB-aligned | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions
**Result: 10 errors ✅ (all tests correctly rejected)**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | test_fail_is_error_too_strong_else | Else is error (wrong) | FAIL | ✅ FAIL |
| 2 | test_fail_page_index_valid_off_by_one | NUM_PAGES valid (off-by-one) | FAIL | ✅ FAIL |
| 3 | test_fail_page_ptr_valid_tighter_bound | Tighter range than spec | FAIL | ✅ FAIL |
| 4 | test_fail_page_ptr_to_index_always_zero | All ptrs map to index 0 | FAIL | ✅ FAIL |
| 5 | test_fail_page_index_to_ptr_always_small | All ptrs < 0x1000 | FAIL | ✅ FAIL |
| 6 | test_fail_all_noswitch | All returns are NoSwitch | FAIL | ✅ FAIL |
| 7 | test_fail_truncate_2m_always_zero | Truncation always returns 0 | FAIL | ✅ FAIL |
| 8 | test_fail_get_payload_always_some | All payloads have endpoint | FAIL | ✅ FAIL |
| 9 | test_fail_2m_valid_for_all_valid | All valid indices are 2m-valid | FAIL | ✅ FAIL |
| 10 | test_fail_error_always_unchanged | Error ⟹ kernel unchanged | FAIL | ✅ FAIL |

### Round 3: Negated/Contradicted Postconditions
**Result: 10 errors ✅ (all tests correctly rejected)**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | test_fail_negate_is_error | Error is not error (negated) | FAIL | ✅ FAIL |
| 2 | test_fail_negate_endpoint_payload | Endpoint(5) returns None | FAIL | ✅ FAIL |
| 3 | test_fail_negate_empty_payload | Empty returns Some | FAIL | ✅ FAIL |
| 4 | test_fail_negate_page_ptr_valid | 0x1000 not valid (negated) | FAIL | ✅ FAIL |
| 5 | test_fail_negate_page_index_valid | 0 not valid (negated) | FAIL | ✅ FAIL |
| 6 | test_fail_negate_page_entry_empty | Zero entry not empty (negated) | FAIL | ✅ FAIL |
| 7 | test_fail_negate_2m_valid | 512 not 2m-valid (negated) | FAIL | ✅ FAIL |
| 8 | test_fail_negate_thread_state | RUNNING == BLOCKED | FAIL | ✅ FAIL |
| 9 | test_fail_negate_endpoint_state | SEND == RECEIVE | FAIL | ✅ FAIL |
| 10 | test_fail_negate_page_ptr_index | 0x1000 → index ≠ 1 | FAIL | ✅ FAIL |

### Round 4: Wrong Specific Values
**Result: 10 errors ✅ (all tests correctly rejected)**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | test_fail_wrong_page_index_for_0x2000 | 0x2000 → 3 (should be 2) | FAIL | ✅ FAIL |
| 2 | test_fail_wrong_page_ptr_for_index_3 | index 3 → 0x4000 (should be 0x3000) | FAIL | ✅ FAIL |
| 3 | test_fail_wrong_truncate_2m | trunc(513) = 0 (should be 512) | FAIL | ✅ FAIL |
| 4 | test_fail_wrong_endpoint_index | Endpoint(10) → Some(11) | FAIL | ✅ FAIL |
| 5 | test_fail_wrong_endpoint_index_2 | Endpoint(0) → Some(1) | FAIL | ✅ FAIL |
| 6 | test_fail_wrong_page0_index | ptr 0 → index 1 | FAIL | ✅ FAIL |
| 7 | test_fail_wrong_index0_ptr | index 0 → ptr 0x1000 | FAIL | ✅ FAIL |
| 8 | test_fail_wrong_truncate_1023 | trunc(1023) = 1024 (should be 512) | FAIL | ✅ FAIL |
| 9 | test_fail_wrong_is_empty | Non-zero addr is empty | FAIL | ✅ FAIL |
| 10 | test_fail_wrong_error_code | Error == Else | FAIL | ✅ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases
**Result: 10 errors ✅ (all tests correctly rejected)**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | test_fail_ptr_valid_implies_index_valid | page_ptr_valid(ptr) ⟹ page_index_valid(ptr) | FAIL | ✅ FAIL |
| 2 | test_fail_all_payloads_same | Empty and Endpoint give same result | FAIL | ✅ FAIL |
| 3 | test_fail_2m_implies_1g | 2m-valid ⟹ 1g-valid | FAIL | ✅ FAIL |
| 4 | test_fail_ptr_2m_implies_1g | ptr 2m-valid ⟹ ptr 1g-valid | FAIL | ✅ FAIL |
| 5 | test_fail_success_implies_error | Success ⟹ is_error (contradiction) | FAIL | ✅ FAIL |
| 6 | test_fail_truncate_greater_than_input | trunc(x) > x | FAIL | ✅ FAIL |
| 7 | test_fail_error_else_interchangeable | Error == Else | FAIL | ✅ FAIL |
| 8 | test_fail_payload_always_endpoint | All payloads are Endpoint | FAIL | ✅ FAIL |
| 9 | test_fail_max_page_ptr_valid | usize::MAX is valid page ptr | FAIL | ✅ FAIL |
| 10 | test_fail_success_kernel_unchanged | Success ⟹ kernel unchanged | FAIL | ✅ FAIL |

---

## Overall Assessment

### Correctness: ✅ All 35 tests pass (78 verification units)
The specs are **correct** — all open spec functions behave as documented. The `syscall_receive_endpoint` function's `ensures` clauses are consistent: error returns imply the fail spec, non-error returns imply the success spec, and `wf()` is always preserved.

### Completeness: ✅ All 50 tests fail as expected
The specs are **sufficiently tight** — invalid inputs, wrong values, negated assertions, overly strong claims, and cross-function misuse are all correctly rejected by the verifier. Key findings:
- `spec_is_error` correctly discriminates only `RetValueType::Error` (not `ErrorNoQuota`, `ErrorVaInUse`, etc.)
- Page utility functions have precise arithmetic that rejects wrong values
- The `syscall_receive_endpoint_fail` spec correctly allows kernel state changes (thread blocking), so asserting `old_k =~= new_k` on error is properly rejected
- Success path correctly modifies kernel state (endpoint passing), rejecting `old_k =~= new_k`

### Spec Gaps: None found
No completeness test passed unexpectedly. All tested spec functions provide appropriately tight guarantees.

### Notes
- The `Kernel` type is extremely complex (deeply nested with `Tracked` types, `Ghost` types, and `#[verifier::external_body]` closed specs), making it impossible to construct concrete instances in proof mode. Testing of the main syscall spec was done via parametric assume-assert patterns.
- Many internal specs (`ProcessManager::wf()`, `PageAllocator::wf()`, etc.) are `closed spec fn` with `external_body`, so their internal behavior cannot be directly tested — only their interface contracts.
