# Test Summary: `syscall_new_proc_with_endpoint`

## Overview

Generated **39 adversarial proof tests** across three categories to probe the semantic boundary of `syscall_new_proc_with_endpoint` and related functions. All tests are designed to assert properties that the specification should **reject**.

## Results

| Category | Total Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary Tests | 15 | 15 ✅ | 0 |
| Behavioral Mutation Tests | 12 | 12 ✅ | 0 |
| Logical Tests | 12 | 12 ✅ | 0 |
| **Total** | **39** | **39** | **0** |

**All 39 tests failed verification as expected.** The specification correctly rejects all tested invalid inputs, incorrect behaviors, and unintended logical properties.

---

## Boundary Tests (15/15 failed) — `boundary_tests.rs`

| # | Test | Targeted Precondition |
|---|---|---|
| 1 | `test_boundary_thread_not_in_domain` | `thread_dom().contains(thread_ptr)` |
| 2 | `test_boundary_endpoint_index_at_max` | `endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS` |
| 3 | `test_boundary_va_range_len_overflow` | `va_range.len * 3 + 3 < usize::MAX` |
| 4 | `test_boundary_zero_quota_for_new_proc` | `quota.mem_4k >= va_range.len * 3 + 2` |
| 5 | `test_boundary_zero_free_pages` | `free_pages >= va_range.len * 3 + 2` |
| 6 | `test_boundary_children_list_full` | `children.len() < PROC_CHILD_LIST_LEN` |
| 7 | `test_boundary_depth_at_max` | `depth != usize::MAX` |
| 8 | `test_boundary_scheduler_full` | `scheduler.len() < MAX_CONTAINER_SCHEDULER_LEN` |
| 9 | `test_boundary_proc_list_full` | `owned_procs.len() < CONTAINER_PROC_LIST_LEN` |
| 10 | `test_boundary_pcid_exhausted` | `free_pcids.len() > 0` |
| 11 | `test_boundary_endpoint_ptr_is_none` | `endpoint_ptr.is_Some()` |
| 12 | `test_boundary_rf_counter_at_max` | `rf_counter != usize::MAX` |
| 13 | `test_boundary_page_ptr_not_aligned` | `ptr % 0x1000 == 0` |
| 14 | `test_boundary_page_index_at_num_pages` | `i < NUM_PAGES` |
| 15 | `test_boundary_va_zero_not_valid` | `spec_va_4k_valid(va)` |

---

## Behavioral Mutation Tests (12/12 failed) — `behavioral_mutation_tests.rs`

| # | Test | Mutated Postcondition |
|---|---|---|
| 1 | `test_mutant_success_returns_error` | Success should not be reported as error |
| 2 | `test_mutant_failed_req_changes_kernel` | Failed requirement ⇒ kernel unchanged |
| 3 | `test_mutant_new_proc_has_no_threads` | New proc has exactly one thread |
| 4 | `test_mutant_new_proc_wrong_container` | New proc belongs to correct container |
| 5 | `test_mutant_container_dom_changes` | Container domain unchanged |
| 6 | `test_mutant_endpoint_dom_changes` | Endpoint domain unchanged |
| 7 | `test_mutant_new_proc_already_existed` | New proc ptr was not in old domain |
| 8 | `test_mutant_old_thread_changed` | Old threads are preserved |
| 9 | `test_mutant_new_thread_no_endpoint` | New thread has endpoint at slot 0 |
| 10 | `test_mutant_container_procs_unchanged` | Container procs list grows by one |
| 11 | `test_mutant_container_endpoints_changed` | Container endpoints unchanged |
| 12 | `test_mutant_return_values_swapped` | Return values match (proc_ptr, thread_ptr) order |

---

## Logical Tests (12/12 failed) — `logical_tests.rs`

| # | Test | Probed Property |
|---|---|---|
| 1 | `test_logical_deterministic_new_proc_ptr` | Determinism of allocation |
| 2 | `test_logical_proc_ptr_equals_thread_ptr` | proc_ptr ≠ thread_ptr |
| 3 | `test_logical_new_proc_bounded_by_one_page` | No artificial page bound |
| 4 | `test_logical_endpoint_descriptors_slot1_nonempty` | Slots 1+ are None |
| 5 | `test_logical_container_has_exactly_one_proc` | Container can have multiple procs |
| 6 | `test_logical_container_depth_changed` | Depth is preserved |
| 7 | `test_logical_endpoint_old_owners_removed` | Old endpoint owners preserved |
| 8 | `test_logical_alloc_page_was_mapped` | Allocated page was not previously allocated |
| 9 | `test_logical_empty_va_range_nonempty_address_space` | Empty range ⇒ empty address space |
| 10 | `test_logical_free_pages_increase` | Free pages do not increase |
| 11 | `test_logical_page_mapping_dom_grows` | Page mapping domain unchanged |
| 12 | `test_logical_mapping_outside_va_range` | No mappings outside va_range for new proc |

---

## Conclusion

The specification for `syscall_new_proc_with_endpoint` is **consistent** with respect to all 39 tested properties:

- **Boundary completeness**: All preconditions are enforced; invalid inputs are rejected.
- **Behavioral correctness**: Mutated postconditions are correctly rejected; the spec precisely captures the expected state transitions.
- **Logical soundness**: The spec does not entail unintended properties such as determinism, stronger bounds, or cross-function misuse.

No specification weaknesses were detected in this test suite.
