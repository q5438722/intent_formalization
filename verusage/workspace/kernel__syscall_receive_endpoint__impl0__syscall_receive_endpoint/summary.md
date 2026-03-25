# Adversarial Proof Test Summary: `syscall_receive_endpoint`

## Overview

Generated **24 adversarial proof tests** across three categories targeting the `syscall_receive_endpoint` specification. All tests are designed to **FAIL verification**, probing the semantic boundary of the spec.

## Results

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary Tests | 8 | 8 ✅ | 0 |
| Behavioral Mutation Tests | 8 | 8 ✅ | 0 |
| Logical Tests | 8 | 8 ✅ | 0 |
| **Total** | **24** | **24** | **0** |

All 24 tests failed verification as expected — the specification correctly rejects all invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (8/8 FAILED ✅)

| # | Test | Precondition Violated |
|---|---|---|
| 1 | `test_boundary_receiver_thread_not_in_domain` | `thread_dom.contains(receiver_thread_ptr)` |
| 2 | `test_boundary_endpoint_index_at_max` | `blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS` |
| 3 | `test_boundary_payload_index_at_max` | `receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS` |
| 4 | `test_boundary_endpoint_index_usize_max` | `0 <= idx < MAX_NUM_ENDPOINT_DESCRIPTORS` (usize::MAX) |
| 5 | `test_boundary_queue_at_max_capacity` | `queue.len() < MAX_NUM_THREADS_PER_ENDPOINT` |
| 6 | `test_boundary_scheduler_at_max` | `scheduler.len() < MAX_CONTAINER_SCHEDULER_LEN` |
| 7 | `test_boundary_payload_slot_not_empty` | Receiver payload slot must be `is_None` |
| 8 | `test_boundary_rf_counter_at_max` | `rf_counter != usize::MAX` |

## Behavioral Mutation Tests (8/8 FAILED ✅)

| # | Test | Postcondition Mutated |
|---|---|---|
| 1 | `test_mutation_no_endpoint_kernel_changes` | Fail path: kernel state unchanged (`old =~= new`) |
| 2 | `test_mutation_success_queue_unchanged` | Success: shared endpoint queue loses head (`skip(1)`) |
| 3 | `test_mutation_block_queue_not_grown` | Block: thread pushed onto queue (`push(thread_ptr)`) |
| 4 | `test_mutation_dst_descriptors_unchanged` | Success: dst descriptors updated at payload index |
| 5 | `test_mutation_owning_threads_not_updated` | Success: owning_threads includes new `(dst, to)` pair |
| 6 | `test_mutation_error_not_flagged` | Error return has `is_error() == true` |
| 7 | `test_mutation_queue_state_not_changed_to_receive` | Sender queue empty: queue_state → RECEIVE |
| 8 | `test_mutation_success_returns_error` | Success return has `is_error() == false` |

## Logical Tests (8/8 FAILED ✅)

| # | Test | Unintended Property Queried |
|---|---|---|
| 1 | `test_logical_endpoint_indices_must_be_equal` | blocking_endpoint_index == receiver_endpoint_payload |
| 2 | `test_logical_thread_dom_grows` | thread_dom grows (it shouldn't change) |
| 3 | `test_logical_determinism_of_result` | Same inputs → same error/success result |
| 4 | `test_logical_queue_always_nonempty` | Endpoint queue is always non-empty |
| 5 | `test_logical_same_container_required` | Receiver/sender must be in same container |
| 6 | `test_logical_rf_counter_unchanged` | rf_counter stays unchanged after pass_endpoint |
| 7 | `test_logical_all_endpoints_unchanged` | All endpoints unchanged after syscall |
| 8 | `test_logical_sender_equals_receiver` | Sender == receiver thread ptr allowed |

## Conclusion

The specification for `syscall_receive_endpoint` is **consistent** with respect to all 24 adversarial queries. No unintended properties were entailed, no invalid inputs were accepted, and no incorrect behaviors were permitted. The spec correctly handles:

- Input validation boundaries (endpoint indices, queue capacities, scheduler limits)
- State transitions (queue mutations, endpoint passing, thread blocking)
- Structural constraints (container ancestry, thread distinctness, domain preservation)
