# Test Summary: `syscall_new_proc_with_endpoint`

## Target
`source-projects/atmosphere/verified/kernel/kernel__syscall_new_proc__impl0__syscall_new_proc_with_endpoint.rs`

## Overview
Generated **32 adversarial proof tests** across three categories to probe the semantic boundary of the `syscall_new_proc_with_endpoint` specification. All tests are designed to **FAIL verification** if the spec is correct.

---

## Results

| Category | Tests | All Failed? | Spec Issues Found |
|---|---|---|---|
| Boundary Tests | 10 | ✅ Yes (10/10 failed) | None |
| Behavioral Mutation Tests | 12 | ✅ Yes (12/12 failed) | None |
| Logical Tests | 10 | ✅ Yes (10/10 failed) | None |
| **Total** | **32** | **✅ All 32 failed** | **None** |

**Combined correctness_tests.rs**: 16 verified (infrastructure), 32 errors (all 32 adversarial tests rejected). ✅

---

## Boundary Tests (10 tests)

All boundary tests correctly fail, confirming the preconditions are tight:

| # | Test | Failure Mode | Result |
|---|---|---|---|
| 1 | `test_boundary_thread_ptr_not_in_dom` | thread_ptr outside thread_dom | ✅ FAIL |
| 2 | `test_boundary_endpoint_index_out_of_range` | endpoint_index == 128 (>= MAX) | ✅ FAIL |
| 3 | `test_boundary_quota_insufficient` | mem_4k=1 < required 2 | ✅ FAIL |
| 4 | `test_boundary_proc_children_full` | children.len() == PROC_CHILD_LIST_LEN | ✅ FAIL |
| 5 | `test_boundary_scheduler_full` | scheduler.len() == MAX_CONTAINER_SCHEDULER_LEN | ✅ FAIL |
| 6 | `test_boundary_container_proc_list_full` | owned_procs.len() == CONTAINER_PROC_LIST_LEN | ✅ FAIL |
| 7 | `test_boundary_depth_overflow` | depth == usize::MAX | ✅ FAIL |
| 8 | `test_boundary_page_ptrs_equal` | page_ptr_1 == page_ptr_2 | ✅ FAIL |
| 9 | `test_boundary_pcid_collision` | new_pcid collides with existing | ✅ FAIL |
| 10 | `test_boundary_page_ptr_in_closure` | page_ptr already in page_closure | ✅ FAIL |

## Behavioral Mutation Tests (12 tests)

All behavioral mutations correctly fail, confirming the postconditions reject incorrect behaviors:

| # | Test | Mutation | Result |
|---|---|---|---|
| 1 | `test_mutation_quota_subtract_wrong_amount` | mem_4k subtract 3 instead of 2 | ✅ FAIL |
| 2 | `test_mutation_quota_mem_2m_changed` | mem_2m mutated | ✅ FAIL |
| 3 | `test_mutation_quota_ioid_changed` | ioid mutated | ✅ FAIL |
| 4 | `test_mutation_proc_dom_wrong_insertion` | wrong ptr in proc_dom | ✅ FAIL |
| 5 | `test_mutation_thread_dom_wrong_insertion` | wrong ptr in thread_dom | ✅ FAIL |
| 6 | `test_mutation_owned_procs_wrong_order` | prepend vs append | ✅ FAIL |
| 7 | `test_mutation_new_proc_wrong_pcid` | pcid mismatch | ✅ FAIL |
| 8 | `test_mutation_new_proc_ioid_not_none` | ioid is Some (should be None) | ✅ FAIL |
| 9 | `test_mutation_endpoint_dom_changes` | endpoint_dom grows | ✅ FAIL |
| 10 | `test_mutation_container_dom_changes` | container_dom grows | ✅ FAIL |
| 11 | `test_mutation_endpoint_descriptor_wrong_slot` | endpoint at index 1 vs 0 | ✅ FAIL |
| 12 | `test_mutation_owned_threads_wrong_thread` | wrong thread ptr | ✅ FAIL |

## Logical Tests (10 tests)

All logical tests correctly fail, confirming the spec does not entail unintended properties:

| # | Test | Unintended Property Tested | Result |
|---|---|---|---|
| 1 | `test_logical_determinism_new_proc_ptr` | new_proc_ptr is deterministic | ✅ FAIL |
| 2 | `test_logical_stronger_quota_stays_positive` | quota > 0 after subtract | ✅ FAIL |
| 3 | `test_logical_proc_thread_same_ptr` | page_ptr_1 == page_ptr_2 | ✅ FAIL |
| 4 | `test_logical_page_closure_grows_by_three` | 3 pages added (only 2) | ✅ FAIL |
| 5 | `test_logical_owned_threads_length_two` | owned_threads.len >= 2 | ✅ FAIL |
| 6 | `test_logical_old_thread_removed` | old thread removed from dom | ✅ FAIL |
| 7 | `test_logical_owned_endpoints_differ` | owned_endpoints changed | ✅ FAIL |
| 8 | `test_logical_subtree_unchanged` | subtree set unchanged | ✅ FAIL |
| 9 | `test_logical_cross_fn_endpoint_dom_grows` | new endpoint created | ✅ FAIL |
| 10 | `test_logical_new_thread_slot_0_is_none` | descriptor slot 0 is None | ✅ FAIL |

---

## Conclusion

The specification for `syscall_new_proc_with_endpoint` is **consistent** with respect to all 32 adversarial queries:
- **Preconditions** correctly reject all invalid inputs at boundary values.
- **Postconditions** correctly reject all mutated behaviors.
- **Logical properties** not guaranteed by the spec are correctly not entailed.

No specification weaknesses were identified by these tests.
