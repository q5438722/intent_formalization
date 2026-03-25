# Test Summary: `syscall_receive_endpoint`

## Target
`kernel__syscall_receive_endpoint__impl0__syscall_receive_endpoint.rs`

## Overview
22 adversarial proof tests were generated across 3 categories to probe the semantic boundary of the `syscall_receive_endpoint` specification. **All 22 tests failed verification as expected**, confirming the specification correctly rejects each undesirable property.

---

## Results by Category

### Boundary Tests (8 tests) — All FAIL ✅

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_receiver_not_in_thread_dom` | thread_ptr=999 not in domain {1,2,3} | FAIL ✅ |
| 2 | `test_boundary_endpoint_index_at_max` | blocking_endpoint_index=128 (== MAX) | FAIL ✅ |
| 3 | `test_boundary_endpoint_index_overflow` | blocking_endpoint_index=usize::MAX | FAIL ✅ |
| 4 | `test_boundary_payload_index_at_max` | receiver_endpoint_payload=128 | FAIL ✅ |
| 5 | `test_boundary_thread_not_running_blocked` | ThreadState::BLOCKED ≠ RUNNING | FAIL ✅ |
| 6 | `test_boundary_thread_not_running_scheduled` | ThreadState::SCHEDULED ≠ RUNNING | FAIL ✅ |
| 7 | `test_boundary_empty_thread_dom` | Empty domain contains no element | FAIL ✅ |
| 8 | `test_boundary_both_indices_at_max` | Both indices at 128 simultaneously | FAIL ✅ |

### Behavioral Mutation Tests (7 tests) — All FAIL ✅

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_behavioral_queue_push_order_mutated` | Prepend vs. append on queue | FAIL ✅ |
| 2 | `test_behavioral_success_skip_count_mutated` | skip(2) instead of skip(1) | FAIL ✅ |
| 3 | `test_behavioral_endpoint_descriptor_wrong_index` | Update at index 5 instead of 3 | FAIL ✅ |
| 4 | `test_behavioral_owning_threads_wrong_pair` | Insert (dst, 7) instead of (dst, 3) | FAIL ✅ |
| 5 | `test_behavioral_fail_state_changed` | Queue changed in general-error case | FAIL ✅ |
| 6 | `test_behavioral_queue_state_not_changed_to_receive` | SEND instead of RECEIVE transition | FAIL ✅ |
| 7 | `test_behavioral_ipc_payload_wrong_value` | Payload=99 instead of 5 | FAIL ✅ |

### Logical Tests (7 tests) — All FAIL ✅

| # | Test | Unintended Property | Result |
|---|------|-------------------|--------|
| 1 | `test_logical_success_implies_no_state_change` | Success with unchanged queue | FAIL ✅ |
| 2 | `test_logical_queue_length_always_strictly_decreases` | skip(1) on empty reduces length | FAIL ✅ |
| 3 | `test_logical_endpoint_state_always_send` | Endpoint state must be SEND | FAIL ✅ |
| 4 | `test_logical_thread_dom_always_nonempty` | thread_dom is always non-empty | FAIL ✅ |
| 5 | `test_logical_return_never_error` | Syscall never returns Error | FAIL ✅ |
| 6 | `test_logical_skip_preserves_head` | Head preserved after skip(1) | FAIL ✅ |
| 7 | `test_logical_endpoint_descriptors_globally_unique` | No two threads share endpoint_ptr | FAIL ✅ |

---

## Conclusion

The specification for `syscall_receive_endpoint` correctly rejects all 22 adversarial queries:
- **Boundary tests**: Invalid inputs (out-of-range indices, wrong thread states, missing domains) are properly rejected by the preconditions.
- **Behavioral mutation tests**: Incorrect output relations (wrong queue operations, wrong index updates, wrong payload values) are properly distinguished from correct behavior.
- **Logical tests**: Unintended semantic properties (determinism, structural assumptions, over-strong invariants) are not entailed by the specification.

No spec weaknesses were detected in this round of testing.
