# Adversarial Test Summary: `syscall_send_endpoint`

## Target
`kernel__syscall_send_endpoint__impl0__syscall_send_endpoint.rs`

## Overview
30 adversarial proof tests were generated across 3 files, each encoding a property φ that is **NOT entailed** by the specification. All 30 tests **FAILED verification** as expected, confirming the spec correctly rejects these unintended properties.

---

## Results

| File | Tests | Failures | Passes (proof boilerplate) |
|------|-------|----------|---------------------------|
| `boundary_tests.rs` | 10 | 10 ✅ | 3 (main + type defs) |
| `behavioral_mutation_tests.rs` | 10 | 10 ✅ | 8 (main + type defs) |
| `logical_tests.rs` | 10 | 10 ✅ | 4 (main + type defs) |

All 30 adversarial assertions were correctly rejected by the verifier.

---

## Boundary Tests (10/10 FAIL)

| # | Test | Property Violated |
|---|------|-------------------|
| 1 | `test_boundary_sender_thread_not_in_domain` | `thread_dom.contains(sender_thread_ptr)` required |
| 2 | `test_boundary_endpoint_index_at_max` | `blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS` required |
| 3 | `test_boundary_payload_index_overflow` | `sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS` required |
| 4 | `test_boundary_sender_thread_not_running` | `state == ThreadState::RUNNING` required |
| 5 | `test_boundary_kernel_not_wf` | `old(self).wf()` required |
| 6 | `test_boundary_endpoint_queue_full_for_blocking` | `queue.len() < MAX_NUM_THREADS_PER_ENDPOINT` for block_running_thread |
| 7 | `test_boundary_same_sender_receiver_thread` | `src_thread_ptr != dst_thread_ptr` for pass_endpoint |
| 8 | `test_boundary_receiver_slot_occupied` | `dst endpoint slot is_None()` for pass_endpoint |
| 9 | `test_boundary_schedule_empty_queue` | `queue.len() > 0` for schedule_blocked_thread |
| 10 | `test_boundary_ancestor_not_in_container_domain` | `container_dom.contains(ancestor_ptr)` for container_check_is_ancestor |

## Behavioral Mutation Tests (10/10 FAIL)

| # | Test | Mutation |
|---|------|----------|
| 1 | `test_mutation_no_endpoint_should_not_modify_state` | Claimed thread_dom grew when endpoint missing (spec: `old =~= new`) |
| 2 | `test_mutation_full_queue_should_not_change_queue` | Claimed queue grew when full (spec: `old =~= new`) |
| 3 | `test_mutation_no_receiver_descriptors_should_be_preserved` | Claimed descriptors changed in no_receiver case (spec preserves them) |
| 4 | `test_mutation_success_queue_should_dequeue_head` | Claimed queue unchanged on success (spec: `queue = old.queue.skip(1)`) |
| 5 | `test_mutation_success_receiver_descriptors_should_update` | Claimed receiver descriptors unchanged (spec: updated with sender endpoint) |
| 6 | `test_mutation_success_owning_threads_should_grow` | Claimed owning_threads unchanged (spec: inserted new entry) |
| 7 | `test_mutation_success_should_not_return_error` | Claimed success returns Error (spec: returns Else) |
| 8 | `test_mutation_blocked_payload_should_match` | Claimed wrong payload stored in ipc_payload |
| 9 | `test_mutation_empty_receiver_queue_state_should_become_send` | Claimed queue_state stays RECEIVE (spec: changes to SEND) |
| 10 | `test_mutation_success_sender_descriptors_preserved` | Claimed sender descriptor cleared (spec preserves them) |

## Logical Tests (10/10 FAIL)

| # | Test | Unintended Property |
|---|------|---------------------|
| 1 | `test_logical_different_endpoint_index_same_result` | Determinism: same result for different endpoint indices |
| 2 | `test_logical_sender_always_blocked_after_send` | Sender always blocked (false on error paths) |
| 3 | `test_logical_endpoint_domain_grows` | Endpoint domain grows (spec preserves it) |
| 4 | `test_logical_thread_domain_grows` | Thread domain grows (spec preserves it) |
| 5 | `test_logical_queue_state_always_send` | Queue state always SEND after call |
| 6 | `test_logical_proc_domain_unchanged` | Process domain grows (spec preserves it) |
| 7 | `test_logical_blocking_equals_payload` | Blocking endpoint == payload endpoint |
| 8 | `test_logical_rf_counter_always_increases` | rf_counter always increases |
| 9 | `test_logical_queue_always_empty_after_success` | Queue always empty after success (stronger than skip(1)) |
| 10 | `test_logical_container_domain_grows` | Container domain grows (spec preserves it) |

---

## Conclusion

The specification for `syscall_send_endpoint` correctly rejects all 30 adversarial properties:
- **Boundary**: All preconditions are enforced — invalid inputs are properly rejected.
- **Behavioral**: All output mutations are caught — incorrect behaviors are not admitted.
- **Logical**: No unintended stronger properties are entailed — the spec does not over-promise.

No spec weaknesses were detected through this test suite.
