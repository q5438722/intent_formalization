# Test Execution Summary: `syscall_send_empty_no_block`

## Target Function
`Kernel::syscall_send_empty_no_block(&mut self, sender_thread_ptr: ThreadPtr, blocking_endpoint_index: EndpointIdx) -> SyscallReturnStruct`

### Specification
- **Requires**: `self.wf()`, `thread_dom().contains(sender_thread_ptr)`, `0 <= blocking_endpoint_index < 128`, `thread.state == RUNNING`
- **Ensures**: `self.wf()` (only)

---

## Results Overview

| Test File | Total Tests | Failed (as expected) | Passed (unexpected) |
|-----------|-------------|---------------------|---------------------|
| `boundary_tests.rs` | 10 | 10 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 7 | 7 ✅ | 0 |
| `logical_tests.rs` | 9 | 9 ✅ | 0 |
| **Total** | **26** | **26 ✅** | **0** |

All 26 adversarial tests failed verification, which is the expected outcome — each test encodes a property that is NOT entailed by the specification.

---

## Boundary Tests (10/10 FAIL ✅)

| # | Test | Violated Precondition |
|---|------|-----------------------|
| 1 | `test_boundary_sender_thread_not_in_domain` | `thread_dom.contains(sender_thread_ptr)` |
| 2 | `test_boundary_endpoint_index_at_max` | `endpoint_index < 128` (at boundary 128) |
| 3 | `test_boundary_endpoint_index_exceeds_max` | `endpoint_index < 128` (value 256) |
| 4 | `test_boundary_sender_thread_not_running` | `state == RUNNING` (BLOCKED) |
| 5 | `test_boundary_sender_thread_scheduled` | `state == RUNNING` (SCHEDULED) |
| 6 | `test_boundary_page_ptr_not_aligned` | `ptr % 0x1000 == 0` (page_ptr2page_index) |
| 7 | `test_boundary_page_index_at_max` | `i < NUM_PAGES` (page_index2page_ptr) |
| 8 | `test_boundary_endpoint_queue_full` | `queue_len < MAX_NUM_THREADS_PER_ENDPOINT` |
| 9 | `test_boundary_scheduler_at_max_capacity` | `scheduler_len < MAX_CONTAINER_SCHEDULER_LEN` |
| 10 | `test_boundary_endpoint_index_usize_max` | `endpoint_index < 128` (usize::MAX) |

## Behavioral Mutation Tests (7/7 FAIL ✅)

| # | Test | Mutated Relation |
|---|------|-----------------|
| 1 | `test_mutation_none_descriptor_returns_success` | None descriptor → claim success |
| 2 | `test_mutation_send_state_small_queue_returns_success` | SEND state → claim success |
| 3 | `test_mutation_send_state_full_queue_returns_success` | SEND + full queue → claim success |
| 4 | `test_mutation_receive_empty_queue_returns_success` | RECEIVE + empty → claim success |
| 5 | `test_mutation_success_returns_error` | Valid conditions → claim error |
| 6 | `test_mutation_full_scheduler_returns_success` | Full scheduler → claim success |
| 7 | `test_mutation_queue_unchanged_after_schedule` | Queue shrinks → claim unchanged |

## Logical Tests (9/9 FAIL ✅)

| # | Test | Unwarranted Property |
|---|------|---------------------|
| 1 | `test_logical_determinism` | Determinism of return value |
| 2 | `test_logical_always_error` | Return is always Error |
| 3 | `test_logical_always_success` | Return is always Else |
| 4 | `test_logical_thread_dom_changes` | Thread domain changes |
| 5 | `test_logical_queue_state_flips` | Queue state flips RECEIVE→SEND |
| 6 | `test_logical_switch_decision_is_switch` | Switch decision is Switch |
| 7 | `test_logical_stronger_endpoint_bound` | endpoint_index < 64 (stronger) |
| 8 | `test_logical_sender_equals_receiver` | Sender equals receiver |
| 9 | `test_logical_endpoint_dom_grows` | Endpoint domain grows |

---

## Spec Weakness Analysis

The `ensures` clause of `syscall_send_empty_no_block` only guarantees `self.wf()`. This means the specification:
- Does **not** constrain the return value (`SyscallReturnStruct`) at all
- Does **not** specify which error paths produce which return codes
- Does **not** guarantee state preservation (e.g., thread_dom, endpoint_dom unchanged)
- Does **not** guarantee the endpoint queue shrinks on success
- Does **not** guarantee the receiver is actually scheduled

The preconditions are well-defined and correctly reject invalid inputs. However, the postcondition is **too weak** to prevent many unintended behaviors — it only preserves the kernel's well-formedness invariant without constraining observable outputs.
