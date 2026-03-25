# Adversarial Proof Test Summary

## Target
`kernel__syscall_new_thread_with_endpoint__impl0__syscall_new_thread_with_endpoint.rs`

Function under test: `Kernel::syscall_new_thread_with_endpoint`

---

## Results Overview

| Test File | Tests | Failed (expected) | Passed (unexpected) |
|-----------|-------|--------------------|---------------------|
| boundary_tests.rs | 10 | 10 | 0 |
| behavioral_mutation_tests.rs | 10 | 10 | 0 |
| logical_tests.rs | 10 | 10 | 0 |
| **Total** | **30** | **30** | **0** |

**All 30 adversarial tests were correctly rejected by Verus**, confirming that none of the tested undesirable properties are entailed by the specification.

---

## Boundary Tests (10/10 FAIL ✅)

| # | Test | Property Challenged |
|---|------|---------------------|
| 1 | `test_boundary_endpoint_index_at_max` | endpoint_index == 128 violates `< MAX_NUM_ENDPOINT_DESCRIPTORS` |
| 2 | `test_boundary_endpoint_index_overflow` | endpoint_index == usize::MAX far exceeds valid range |
| 3 | `test_boundary_thread_not_in_dom` | thread_ptr not in thread_dom violates precondition |
| 4 | `test_boundary_thread_list_full` | thread count at MAX (128) violates capacity check |
| 5 | `test_boundary_zero_mem_quota` | mem_4k == 0 violates quota requirement |
| 6 | `test_boundary_scheduler_full` | scheduler at MAX_CONTAINER_SCHEDULER_LEN violates capacity |
| 7 | `test_boundary_no_free_pages` | zero free pages violates allocation requirement |
| 8 | `test_boundary_endpoint_rf_counter_full` | rf_counter == usize::MAX means endpoint is full |
| 9 | `test_boundary_no_endpoint_ptr` | None endpoint pointer violates shareability check |
| 10 | `test_boundary_page_already_in_closure` | page already in closure cannot be freshly allocated |

---

## Behavioral Mutation Tests (10/10 FAIL ✅)

| # | Test | Mutated Postcondition |
|---|------|-----------------------|
| 1 | `test_mutation_new_thread_not_in_dom` | New thread NOT in thread_dom (should be) |
| 2 | `test_mutation_proc_dom_changed` | proc_dom gained an element (should be unchanged) |
| 3 | `test_mutation_container_dom_shrank` | container_dom lost an element (should be unchanged) |
| 4 | `test_mutation_endpoint_dom_changed` | endpoint_dom gained an element (should be unchanged) |
| 5 | `test_mutation_new_thread_no_endpoint` | New thread's endpoint slot 0 is None (should be Some) |
| 6 | `test_mutation_owned_threads_not_pushed` | owned_threads length unchanged (should grow by 1) |
| 7 | `test_mutation_endpoint_owning_threads_no_new` | Endpoint doesn't track new thread (should) |
| 8 | `test_mutation_quota_unchanged` | mem_4k quota unchanged (should decrease by 1) |
| 9 | `test_mutation_owned_procs_changed` | Container owned_procs grew (should be preserved) |
| 10 | `test_mutation_error_path_state_changed` | State changed on error path (should be unchanged) |

---

## Logical Tests (10/10 FAIL ✅)

| # | Test | Unentailed Property |
|---|------|---------------------|
| 1 | `test_logical_determinism` | Two calls produce same new_thread_ptr (not guaranteed) |
| 2 | `test_logical_new_thread_ptr_is_zero` | New thread pointer must be 0 (arbitrary) |
| 3 | `test_logical_thread_dom_grows_by_two` | Thread domain grows by 2 (only 1 thread created) |
| 4 | `test_logical_owning_container_is_zero` | New thread's container must be 0 (arbitrary) |
| 5 | `test_logical_quota_becomes_zero` | Quota becomes zero after subtraction (not necessarily) |
| 6 | `test_logical_all_endpoint_slots_filled` | All endpoint slots filled (only slot 0 is set) |
| 7 | `test_logical_other_containers_have_no_threads` | Non-target containers have empty thread sets |
| 8 | `test_logical_address_space_empty` | Preserved address space is empty (not guaranteed) |
| 9 | `test_logical_new_thread_at_wrong_index` | New thread at index 0 of owned_threads (it's pushed to end) |
| 10 | `test_logical_queue_state_flipped` | Endpoint queue state flipped (should be preserved) |

---

## Conclusion

The specification for `syscall_new_thread_with_endpoint` correctly rejects all 30 adversarial queries across three categories:
- **Boundary**: All precondition violations are properly guarded.
- **Behavioral**: All mutated postconditions are rejected — incorrect state transitions are not admitted.
- **Logical**: No unintended stronger properties are entailed — the spec does not over-constrain or allow spurious reasoning.

The specification appears **consistent** with respect to the tested semantic boundaries.
