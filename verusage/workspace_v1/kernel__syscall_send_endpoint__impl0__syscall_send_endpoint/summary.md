# Test Summary: `syscall_send_endpoint`

## Target
`kernel__syscall_send_endpoint__impl0__syscall_send_endpoint.rs`

## Overview
30 adversarial proof tests were generated across three categories to probe the semantic boundaries of the `syscall_send_endpoint` specification. All 30 tests **correctly fail verification**, confirming that the specification rejects the tested undesirable properties.

---

## Results

| Category | Tests | All Failed (as expected) |
|---|---|---|
| Boundary Tests | 10 | ✅ Yes |
| Behavioral Mutation Tests | 10 | ✅ Yes |
| Logical Tests | 10 | ✅ Yes |
| **Total** | **30** | **✅ 30/30** |

---

## Boundary Tests (10/10 failed ✅)

| # | Test | Targeted Precondition |
|---|---|---|
| 1 | `sender_not_in_thread_dom` | `thread_dom().contains(sender_thread_ptr)` |
| 2 | `blocking_endpoint_index_at_max` | `blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS` |
| 3 | `sender_endpoint_payload_at_max` | `sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS` |
| 4 | `sender_endpoint_payload_overflow` | `sender_endpoint_payload < 128` (usize::MAX) |
| 5 | `receiver_endpoint_payload_at_max` | `receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS` |
| 6 | `queue_at_max_is_not_full` | `queue.len() >= MAX_NUM_THREADS_PER_ENDPOINT` triggers full |
| 7 | `scheduler_at_max_is_not_full` | `scheduler.len() >= MAX_CONTAINER_SCHEDULER_LEN` triggers full |
| 8 | `rf_counter_at_max` | `rf_counter == usize::MAX` triggers error |
| 9 | `empty_thread_domain` | Empty domain contains no thread |
| 10 | `blocking_endpoint_index_overflow` | usize::MAX exceeds index bound |

## Behavioral Mutation Tests (10/10 failed ✅)

| # | Test | Mutated Behavior |
|---|---|---|
| 1 | `queue_push_order_mutated` | Prepend instead of append to queue |
| 2 | `success_skip_count_mutated` | skip(2) instead of skip(1) |
| 3 | `endpoint_descriptor_wrong_index` | Update at wrong descriptor index |
| 4 | `owning_threads_wrong_pair` | Insert wrong (thread, payload) pair |
| 5 | `queue_state_not_changed_to_send` | RECEIVE stays instead of changing to SEND |
| 6 | `sender_descriptors_mutated` | Sender descriptors modified (should be preserved) |
| 7 | `wrong_endpoint_value_passed` | Wrong endpoint ptr placed in receiver descriptor |
| 8 | `queue_emptied_instead_of_skipped` | Queue cleared instead of head removed |
| 9 | `owning_threads_wrong_thread` | Wrong thread ptr in owning_threads insert |
| 10 | `queue_pop_last_instead_of_skip` | Remove tail instead of head |

## Logical Tests (10/10 failed ✅)

| # | Test | Unentailed Property |
|---|---|---|
| 1 | `push_commutativity` | Queue push is commutative (false) |
| 2 | `skip_preserves_length` | skip(1) preserves sequence length (false) |
| 3 | `endpoint_state_always_send` | Endpoint state is always SEND (false) |
| 4 | `all_errors_preserve_state` | All error paths leave state unchanged (false for blocking) |
| 5 | `insert_different_elements_equal` | Inserting different elements yields same set (false) |
| 6 | `skip_empty_reduces_length` | skip(1) on empty reduces length (false) |
| 7 | `send_receive_are_same` | SEND == RECEIVE (false) |
| 8 | `update_different_indices_equal` | Update at different indices yields same seq (false) |
| 9 | `push_then_skip_is_identity` | push(x) then skip(1) is identity (false) |
| 10 | `sender_equals_receiver` | sender_thread_ptr == receiver_thread_ptr (contradicts pass_endpoint) |

---

## Conclusion

The specification correctly rejects all 30 adversarial properties:
- **Invalid inputs** (boundary violations) are properly guarded by preconditions.
- **Incorrect behaviors** (mutated outputs) are properly constrained by postconditions.
- **Unintended reasoning** (logical over-claims) is not entailed by the spec.

No spec weaknesses were detected by these tests.
