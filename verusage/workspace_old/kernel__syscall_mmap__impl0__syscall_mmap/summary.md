# Specification Test Summary: `syscall_mmap`

## File Under Test

`kernel__syscall_mmap__impl0__syscall_mmap.rs` defines a kernel `syscall_mmap` function and its specification (`syscall_mmap_spec`, `syscall_mmap_return_value`). The function maps virtual address ranges into a process's address space by allocating physical pages. It returns errors when the container lacks quota (`ErrorNoQuota`) or when the VA range is already in use (`ErrorVaInUse`). On success, it preserves all kernel state except the target process's address space and physical page mappings.

---

## Correctness Results (should all PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_return_value_no_quota` | Returns ErrorNoQuota when quota insufficient | PASS | PASS ✅ |
| 2 | `test_return_value_va_in_use` | Returns ErrorVaInUse when VA range occupied | PASS | PASS ✅ |
| 3 | `test_return_value_success` | Returns Success when all preconditions met | PASS | PASS ✅ |
| 4 | `test_is_error_success` | Success.is_error() == false | PASS | PASS ✅ |
| 5 | `test_is_error_no_quota` | ErrorNoQuota.is_error() == true | PASS | PASS ✅ |
| 6 | `test_is_error_va_in_use` | ErrorVaInUse.is_error() == true | PASS | PASS ✅ |
| 7 | `test_is_error_else` | Else.is_error() == true | PASS | PASS ✅ |
| 8 | `test_spec_error_preserves_state` | On error, new kernel =~= old kernel | PASS | PASS ✅ |
| 9 | `test_spec_success_thread_dom_preserved` | Thread domain unchanged on success | PASS | PASS ✅ |
| 10 | `test_spec_success_proc_dom_preserved` | Proc domain unchanged on success | PASS | PASS ✅ |
| 11 | `test_spec_success_container_dom_preserved` | Container domain unchanged on success | PASS | PASS ✅ |
| 12 | `test_spec_success_endpoint_dom_preserved` | Endpoint domain unchanged on success | PASS | PASS ✅ |
| 13 | `test_spec_success_threads_unchanged` | All threads unchanged on success | PASS | PASS ✅ |
| 14 | `test_spec_success_procs_unchanged` | All procs unchanged on success | PASS | PASS ✅ |
| 15 | `test_spec_success_other_containers_unchanged` | Non-target containers unchanged | PASS | PASS ✅ |
| 16 | `test_spec_success_other_proc_addr_space_unchanged` | Other procs' addr spaces unchanged | PASS | PASS ✅ |
| 17 | `test_spec_success_existing_page_mappings_preserved` | Existing physical page mappings preserved | PASS | PASS ✅ |
| 18 | `test_spec_success_new_vas_mapped` | New VAs are in target proc's address space | PASS | PASS ✅ |
| 19 | `test_spec_success_non_range_unchanged` | Non-VA-range addresses unchanged | PASS | PASS ✅ |
| 20 | `test_spec_success_container_owned_threads_preserved` | Container owned_threads preserved | PASS | PASS ✅ |
| 21 | `test_spec_success_container_owned_procs_preserved` | Container owned_procs preserved | PASS | PASS ✅ |
| 22 | `test_spec_success_container_owned_endpoints_preserved` | Container owned_endpoints preserved | PASS | PASS ✅ |
| 23 | `test_spec_success_container_subtree_preserved` | Container subtree_set preserved | PASS | PASS ✅ |
| 24 | `test_spec_success_container_depth_preserved` | Container depth preserved | PASS | PASS ✅ |
| 25 | `test_spec_success_new_pages_were_unmapped` | New pages were not previously mapped | PASS | PASS ✅ |
| 26 | `test_quota_subtract_mem_4k` | Quota subtraction correctness | PASS | PASS ✅ |
| 27 | `test_quota_subtract_only_mem4k_changes` | Only mem_4k field changes in subtraction | PASS | PASS ✅ |
| 28 | `test_spec_success_other_container_pages_unchanged` | Other containers' owned pages unchanged | PASS | PASS ✅ |
| 29 | `test_spec_success_endpoints_unchanged` | All endpoints unchanged on success | PASS | PASS ✅ |

**Verification result: 74 verified, 0 errors**

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_round1_no_wf` | Use kernel without total_wf() | FAIL | FAIL ✅ |
| 2 | `test_round1_thread_not_in_dom` | Thread not in thread_dom | FAIL | FAIL ✅ |
| 3 | `test_round1_va_range_not_wf` | VaRange not well-formed | FAIL | FAIL ✅ |
| 4 | `test_round1_return_without_checks` | Assert success without checking quota/VA | FAIL | FAIL ✅ |
| 5 | `test_round1_spec_not_established` | Assert spec result without spec holding | FAIL | FAIL ✅ |

**Verification result: 45 verified, 5 errors**

### Round 2: Overly Strong Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_round2_kernel_unchanged_on_success` | Kernel unchanged on success path | FAIL | FAIL ✅ |
| 2 | `test_round2_container_fully_unchanged_on_success` | Container fully unchanged (quota changes) | FAIL | FAIL ✅ |
| 3 | `test_round2_addr_space_fully_unchanged` | Address space fully unchanged on success | FAIL | FAIL ✅ |
| 4 | `test_round2_page_mapping_dom_unchanged` | Page mapping domain unchanged on success | FAIL | FAIL ✅ |
| 5 | `test_round2_always_success` | Return always success | FAIL | FAIL ✅ |

**Verification result: 45 verified, 5 errors**

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_round3_thread_dom_changed` | Thread domain changed on success | FAIL | FAIL ✅ |
| 2 | `test_round3_proc_dom_changed` | Proc domain changed on success | FAIL | FAIL ✅ |
| 3 | `test_round3_success_is_error` | Success returns error | FAIL | FAIL ✅ |
| 4 | `test_round3_state_changes_on_error` | State changes on error path | FAIL | FAIL ✅ |
| 5 | `test_round3_new_vas_not_mapped` | New VAs not mapped on success | FAIL | FAIL ✅ |

**Verification result: 45 verified, 5 errors**

### Round 4: Wrong Specific Values

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_round4_wrong_error_type_quota` | ErrorVaInUse when should be ErrorNoQuota | FAIL | FAIL ✅ |
| 2 | `test_round4_wrong_error_type_success` | ErrorNoQuota when should be Success | FAIL | FAIL ✅ |
| 3 | `test_round4_success_is_error` | Success.is_error() == true | FAIL | FAIL ✅ |
| 4 | `test_round4_error_not_error` | ErrorNoQuota.is_error() == false | FAIL | FAIL ✅ |
| 5 | `test_round4_wrong_quota_subtraction` | Wrong subtraction amount (20 vs 10) | FAIL | FAIL ✅ |
| 6 | `test_round4_quota_wrong_field_changes` | mem_2m changed in quota subtraction | FAIL | FAIL ✅ |
| 7 | `test_round4_wrong_else_type` | Else when should be ErrorNoQuota | FAIL | FAIL ✅ |

**Verification result: 45 verified, 7 errors**

### Round 5: Cross-function Misuse & Edge Cases

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_round5_new_pages_already_mapped` | Claim new pages were already mapped | FAIL | FAIL ✅ |
| 2 | `test_round5_va_maps_to_existing_addr` | New VA maps to existing proc's address | FAIL | FAIL ✅ |
| 3 | `test_round5_owning_container_changed` | Thread's container changed on success | FAIL | FAIL ✅ |
| 4 | `test_round5_other_proc_addr_space_changed` | Other proc's addr space changed | FAIL | FAIL ✅ |
| 5 | `test_round5_existing_page_mapping_changed` | Existing page mappings changed | FAIL | FAIL ✅ |

**Verification result: 45 verified, 5 errors**

---

## Overall Assessment

### Correctness: ✅ PASS
All 29 correctness tests verify successfully. The specifications correctly express the intended behavior of `syscall_mmap`:
- Return value logic (quota check → VA availability check → success) is correct
- Error path preserves all state
- Success path preserves domains, threads, procs, endpoints, non-target containers/pages
- New VAs are properly mapped with fresh physical pages

### Completeness: ✅ PASS
All 27 completeness tests fail as expected. The specifications are tight enough to reject:
- Missing preconditions
- Overly strong claims about what is preserved (kernel/container/address space fully unchanged)
- Negations of guaranteed postconditions
- Wrong error types and wrong concrete values
- Cross-function misuse (claiming existing pages equal new ones, etc.)

### Spec Gaps Found: None
No spec gaps were identified. The specification is both correct and complete for the properties tested.
