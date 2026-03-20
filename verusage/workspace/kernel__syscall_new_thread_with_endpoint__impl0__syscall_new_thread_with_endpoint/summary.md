# Specification Test Summary

## File Under Test

`kernel__syscall_new_thread_with_endpoint__impl0__syscall_new_thread_with_endpoint.rs`

This file defines the `syscall_new_thread_with_endpoint` syscall for a verified microkernel (Atmosphere). The syscall creates a new thread with an endpoint descriptor, modifying the kernel's process manager, page allocator, and memory state. Key specs:

- **`syscall_new_thread_with_endpoint_requirement`**: Defines when the syscall can proceed (thread list not full, quota available, scheduler not full, free pages exist, endpoint is shareable).
- **`syscall_new_thread_with_endpoint_spec`**: Defines the full state transition — on failure the state is unchanged; on success, a new thread is created, domains/quotas/endpoint refs are updated.
- **`Quota::spec_subtract_mem_4k`**: Specifies quota subtraction (only `mem_4k` changes).
- **`SyscallReturnStruct::get_return_vaule_usize`**: Extracts usize from a `SuccessUsize` return.
- **`Endpoint::rf_counter_is_full`**: Checks if endpoint reference counter is at `usize::MAX`.

---

## Correctness Results (should all PASS)

**File**: `correctness_tests.rs`
**Result**: ✅ 68 verified, 0 errors

| Test Name | Description | Expected | Actual |
|-----------|-------------|----------|--------|
| `test_quota_subtract_concrete_1` | Quota 10→9, subtract 1 | PASS | ✅ PASS |
| `test_quota_subtract_concrete_2` | Quota 1→0, subtract 1 | PASS | ✅ PASS |
| `test_quota_subtract_concrete_3` | Quota 100→50, subtract 50 | PASS | ✅ PASS |
| `test_quota_subtract_zero` | Quota unchanged, subtract 0 | PASS | ✅ PASS |
| `test_quota_subtract_parametric` | Parametric quota subtraction | PASS | ✅ PASS |
| `test_return_value_success_usize` | SuccessUsize(42) → Some(42) | PASS | ✅ PASS |
| `test_return_value_success_usize_zero` | SuccessUsize(0) → Some(0) | PASS | ✅ PASS |
| `test_return_value_error` | Error → None | PASS | ✅ PASS |
| `test_return_value_else` | Else → None | PASS | ✅ PASS |
| `test_rf_counter_full_param` | rf_counter == MAX → is_full | PASS | ✅ PASS |
| `test_rf_counter_not_full_param` | rf_counter < MAX → !is_full | PASS | ✅ PASS |
| `test_rf_counter_not_full_zero` | rf_counter == 0 → !is_full | PASS | ✅ PASS |
| `test_requirement_false_thread_list_full` | Thread list full → requirement false | PASS | ✅ PASS |
| `test_requirement_false_no_quota` | No quota → requirement false | PASS | ✅ PASS |
| `test_requirement_false_scheduler_full` | Scheduler full → requirement false | PASS | ✅ PASS |
| `test_requirement_false_no_free_pages` | No free pages → requirement false | PASS | ✅ PASS |
| `test_requirement_false_endpoint_not_shareable` | Endpoint not shareable → requirement false | PASS | ✅ PASS |
| `test_requirement_true_all_conditions_met` | All conditions met → requirement true | PASS | ✅ PASS |
| `test_spec_error_case_no_state_change` | Requirement false → state unchanged | PASS | ✅ PASS |
| `test_spec_success_proc_dom_unchanged` | Success → proc_dom unchanged | PASS | ✅ PASS |
| `test_spec_success_container_dom_unchanged` | Success → container_dom unchanged | PASS | ✅ PASS |
| `test_spec_success_endpoint_dom_unchanged` | Success → endpoint_dom unchanged | PASS | ✅ PASS |
| `test_spec_success_thread_dom_grows` | Success → thread_dom gains new thread | PASS | ✅ PASS |
| `test_spec_success_return_is_some` | Success → return value is Some | PASS | ✅ PASS |
| `test_spec_success_physical_page_mapping_unchanged` | Success → physical page mapping preserved | PASS | ✅ PASS |
| `test_spec_success_old_threads_unchanged` | Success → old threads preserved | PASS | ✅ PASS |

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

**File**: `completeness_round1.rs`
**Result**: ✅ 5 errors (all tests correctly rejected)

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| `test_no_wf_requirement` | Assert requirement true without `wf()` | FAIL | ✅ FAIL |
| `test_thread_not_in_domain` | Assert requirement true with thread not in domain | FAIL | ✅ FAIL |
| `test_endpoint_index_out_of_range` | Assert requirement true with out-of-range index | FAIL | ✅ FAIL |
| `test_no_preconditions_assert_success` | Assert requirement true with no preconditions | FAIL | ✅ FAIL |
| `test_negative_endpoint_index` | Assert requirement false with invalid index | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions

**File**: `completeness_round2.rs`
**Result**: ✅ 5 errors (all tests correctly rejected)

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| `test_thread_dom_unchanged_on_success` | thread_dom unchanged (wrong: grows) | FAIL | ✅ FAIL |
| `test_quota_unchanged_on_success` | Quota unchanged (wrong: mem_4k decreases) | FAIL | ✅ FAIL |
| `test_proc_threads_unchanged_on_success` | Proc owned_threads unchanged (wrong: gains member) | FAIL | ✅ FAIL |
| `test_container_threads_unchanged_on_success` | Container owned_threads unchanged (wrong: gains member) | FAIL | ✅ FAIL |
| `test_endpoint_owning_threads_unchanged_on_success` | Endpoint owning_threads unchanged (wrong: gains member) | FAIL | ✅ FAIL |

### Round 3: Negated/Contradicted Postconditions

**File**: `completeness_round3.rs`
**Result**: ✅ 5 errors (all tests correctly rejected)

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| `test_spec_not_satisfied` | Assert return is None on success (wrong: is Some) | FAIL | ✅ FAIL |
| `test_proc_dom_changed` | Assert proc_dom gains member (wrong: unchanged) | FAIL | ✅ FAIL |
| `test_state_changed_on_error` | Assert state changed on error (wrong: unchanged) | FAIL | ✅ FAIL |
| `test_page_mapping_changed` | Assert page mapping changed (wrong: preserved) | FAIL | ✅ FAIL |
| `test_owned_procs_changed` | Assert container owned_procs changed (wrong: preserved) | FAIL | ✅ FAIL |

### Round 4: Wrong Specific Values

**File**: `completeness_round4.rs`
**Result**: ✅ 5 errors (all tests correctly rejected)

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| `test_wrong_quota_subtraction` | Assert quota subtracted by 2 (wrong: 1) | FAIL | ✅ FAIL |
| `test_wrong_endpoint_descriptor_index` | Assert endpoint at index 1 (wrong: index 0) | FAIL | ✅ FAIL |
| `test_quota_subtract_zero_wrong` | Assert `spec_subtract_mem_4k(q, 0)` after subtracting 1 | FAIL | ✅ FAIL |
| `test_wrong_owning_container` | Assert new thread owned by wrong container | FAIL | ✅ FAIL |
| `test_wrong_error_on_success` | Assert Error return on success | FAIL | ✅ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases

**File**: `completeness_round5.rs`
**Result**: ✅ 5 errors (all tests correctly rejected)

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| `test_address_space_changed` | Assert address spaces changed (wrong: preserved) | FAIL | ✅ FAIL |
| `test_container_owned_pages_changed` | Assert container pages changed (wrong: preserved) | FAIL | ✅ FAIL |
| `test_endpoint_queue_state_changed` | Assert endpoint queue_state changed (wrong: preserved) | FAIL | ✅ FAIL |
| `test_unrelated_proc_changed` | Assert unrelated proc changed (wrong: preserved) | FAIL | ✅ FAIL |
| `test_unrelated_endpoint_changed` | Assert unrelated endpoint changed (wrong: preserved) | FAIL | ✅ FAIL |

---

## Overall Assessment

- **Correctness**: ✅ All 26 correctness tests pass (68 total verified items including background definitions). The specs correctly describe valid behaviors.
- **Completeness**: ✅ All 25 completeness tests fail as expected. The specs are tight enough to reject:
  - Precondition violations (5/5 rejected)
  - Overly strong postconditions (5/5 rejected)
  - Negated postconditions (5/5 rejected)
  - Wrong specific values (5/5 rejected)
  - Cross-function misuse (5/5 rejected)
- **Spec Gaps Found**: None. The specifications are both correct and complete for the properties tested.
