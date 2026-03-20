# Summary: Specification Testing for `syscall_new_container_with_endpoint`

## File Under Test

`kernel__syscall_new_container__impl0__syscall_new_container_with_endpoint.rs`

This file defines a kernel syscall for creating a new container with an endpoint in the Atmosphere verified OS kernel. The key specs are:

- **`syscall_new_container_with_endpoint_requirement`**: A spec function defining 13 preconditions that must all hold for the syscall to succeed (thread list not full, sufficient quotas for mem_4k/mem_2m/mem_1g/pcid/ioid, container depth not MAX, enough free pages, PCID available, endpoint shareable, address space shareable, children list not full, init_quota large enough for va_range).

- **`syscall_new_container_with_endpoint_spec`**: A spec function describing what changes on success (new container/proc/thread created, domains updated, ownership relationships established, endpoint shared) and that on failure the state is unchanged.

- **`syscall_new_container_with_endpoint`**: The actual function with ensures clauses linking requirement failure to error returns and the spec holding.

---

## Correctness Results (should all PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_requirement_thread_list_full` | Thread list full → requirement false | PASS | ✅ PASS |
| 2 | `test_requirement_insufficient_mem_4k` | mem_4k quota insufficient → requirement false | PASS | ✅ PASS |
| 3 | `test_requirement_insufficient_mem_2m` | mem_2m quota insufficient → requirement false | PASS | ✅ PASS |
| 4 | `test_requirement_depth_max` | Container depth at MAX → requirement false | PASS | ✅ PASS |
| 5 | `test_requirement_pcid_exhausted` | PCID exhausted → requirement false | PASS | ✅ PASS |
| 6 | `test_requirement_children_full` | Children list full → requirement false | PASS | ✅ PASS |
| 7 | `test_requirement_all_satisfied` | All conditions met → requirement true | PASS | ✅ PASS |
| 8 | `test_spec_failure_preserves_state` | Requirement false + spec holds → old == new | PASS | ✅ PASS |
| 9 | `test_requirement_init_quota_too_small` | init_quota.mem_4k < 3*va_range.len → false | PASS | ✅ PASS |
| 10 | `test_requirement_endpoint_not_shareable` | Endpoint not shareable → requirement false | PASS | ✅ PASS |
| 11 | `test_error_is_error` | Error variant is_error == true | PASS | ✅ PASS |
| 12 | `test_success_not_error` | SuccessThreeUsize is_error == false | PASS | ✅ PASS |
| 13 | `test_success_three_usize` | get_return_vaule_three_usize extracts values | PASS | ✅ PASS |
| 14 | `test_requirement_insufficient_free_pages` | Not enough free pages → requirement false | PASS | ✅ PASS |
| 15 | `test_quota_greater_reflexive` | Quota spec_greater is reflexive | PASS | ✅ PASS |
| 16 | `test_quota_subtract_zero` | Quota subtract with k=0 preserves all | PASS | ✅ PASS |
| 17 | `test_is_error_distinction` | Error vs non-Error variant distinction | PASS | ✅ PASS |
| 18 | `test_spec_is_error_matches_error` | spec_is_error matches Error variant | PASS | ✅ PASS |
| 19 | `test_spec_is_error_false_for_else` | spec_is_error false for Else variant | PASS | ✅ PASS |
| 20 | `test_get_return_value_none_for_error` | get_return_vaule_three_usize returns None for Error | PASS | ✅ PASS |

**Verification output**: `63 verified, 0 errors`

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_precond_assert_true_without_valid_thread` | Assert requirement true without thread membership | FAIL | ✅ FAIL |
| 2 | `test_precond_assert_false_unconditionally` | Assert requirement false without any context | FAIL | ✅ FAIL |
| 3 | `test_precond_spec_without_requirement` | Use spec without ensuring thread_dom membership | FAIL | ✅ FAIL |
| 4 | `test_precond_requirement_true_invalid_thread` | Assert requirement true with no preconditions | FAIL | ✅ FAIL |

**Verification output**: `43 verified, 4 errors`

### Round 2: Overly Strong Postconditions

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_overly_strong_no_change_on_success` | old == new when requirement true (wrong) | FAIL | ✅ FAIL |
| 2 | `test_overly_strong_container_dom_unchanged` | container_dom unchanged on success (wrong) | FAIL | ✅ FAIL |
| 3 | `test_overly_strong_proc_dom_unchanged` | proc_dom unchanged on success (wrong) | FAIL | ✅ FAIL |
| 4 | `test_overly_strong_thread_dom_unchanged` | thread_dom unchanged on success (wrong) | FAIL | ✅ FAIL |

**Verification output**: `43 verified, 4 errors`

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_negate_new_container_has_children` | New container has non-empty children (wrong) | FAIL | ✅ FAIL |
| 2 | `test_negate_new_proc_no_threads` | New proc has empty thread list (wrong) | FAIL | ✅ FAIL |
| 3 | `test_negate_endpoint_dom_changed` | Endpoint domain changed (wrong) | FAIL | ✅ FAIL |
| 4 | `test_negate_parent_owned_procs_changed` | Parent's owned_procs changed (wrong) | FAIL | ✅ FAIL |

**Verification output**: `43 verified, 4 errors`

### Round 4: Wrong Specific Values

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_new_thread_container` | New thread belongs to old container (wrong) | FAIL | ✅ FAIL |
| 2 | `test_wrong_new_proc_container` | New proc belongs to old container (wrong) | FAIL | ✅ FAIL |
| 3 | `test_wrong_new_container_two_procs` | New container has 2 procs (wrong, has 1) | FAIL | ✅ FAIL |
| 4 | `test_wrong_error_not_error` | Error variant is not an error (wrong) | FAIL | ✅ FAIL |

**Verification output**: `43 verified, 4 errors`

### Round 5: Cross-function Misuse & Edge Cases

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_cross_requirement_implies_unrelated` | Requirement true implies mem_1g == 0 (wrong) | FAIL | ✅ FAIL |
| 2 | `test_cross_parent_children_unchanged` | Parent's children list unchanged (wrong) | FAIL | ✅ FAIL |
| 3 | `test_cross_new_thread_all_none_descriptors` | New thread's endpoint_descriptors[0] is None (wrong) | FAIL | ✅ FAIL |
| 4 | `test_edge_quota_greater_strict` | spec_greater implies strict > (wrong, allows =) | FAIL | ✅ FAIL |

**Verification output**: `43 verified, 4 errors`

---

## Overall Assessment

### Correctness: ✅ PASS (20/20 tests pass)
The specifications correctly model the syscall behavior:
- Each individual precondition in the requirement function correctly gates the syscall
- When all preconditions are met, the requirement evaluates to true
- The spec correctly preserves state on failure (old == new)
- Helper specs (is_error, get_return_vaule_three_usize, Quota operations) work correctly

### Completeness: ✅ PASS (20/20 tests fail as expected)
The specifications are tight enough to reject invalid claims:
- Cannot assert requirement without proper preconditions (Round 1)
- Cannot claim state is unchanged on success (Round 2)
- Cannot negate postconditions like empty children, proc ownership (Round 3)
- Cannot assert wrong ownership values or wrong cardinalities (Round 4)
- Cannot derive unrelated properties from requirement, or deny spec-guaranteed changes (Round 5)

### Spec Gaps Found: None
No spec gaps were discovered. The specifications are both correct and complete for the tested properties.
