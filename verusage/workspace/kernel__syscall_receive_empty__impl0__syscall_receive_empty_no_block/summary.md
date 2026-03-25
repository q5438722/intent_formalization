# Test Summary: `syscall_receive_empty_no_block`

## Target Function
`Kernel::syscall_receive_empty_no_block(&mut self, receiver_thread_ptr: ThreadPtr, blocking_endpoint_index: EndpointIdx) -> SyscallReturnStruct`

## Key Observation
The `ensures` clause of `syscall_receive_empty_no_block` is **EMPTY**. This means the specification guarantees **nothing** about the return value or post-state. The function only has preconditions:
- `old(self).wf()`
- `old(self).thread_dom().contains(receiver_thread_ptr)`
- `0 <= blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS`

## Results

### Boundary Tests (7 tests) — ✅ All FAILED as expected
| # | Test | Result | Description |
|---|------|--------|-------------|
| 1 | `test_boundary_thread_not_in_domain` | FAIL | Thread not in domain violates precondition |
| 2 | `test_boundary_endpoint_index_at_max` | FAIL | Index == MAX_NUM_ENDPOINT_DESCRIPTORS violates upper bound |
| 3 | `test_boundary_endpoint_index_exceeds_max` | FAIL | Index > MAX_NUM_ENDPOINT_DESCRIPTORS violates upper bound |
| 4 | `test_boundary_scheduler_at_max_capacity` | FAIL | Scheduler at max capacity cannot schedule |
| 5 | `test_boundary_send_queue_empty` | FAIL | Empty send queue means no sender exists |
| 6 | `test_boundary_endpoint_descriptor_is_none` | FAIL | None descriptor cannot be unwrapped |
| 7 | `test_boundary_receive_state_queue_not_full` | FAIL | Receive state with non-full queue is error path |

### Behavioral Mutation Tests (7 tests) — ✅ All FAILED as expected
| # | Test | Result | Description |
|---|------|--------|-------------|
| 1 | `test_mutation_success_returns_error` | FAIL | Contradicts valid conditions implying success |
| 2 | `test_mutation_none_descriptor_returns_success` | FAIL | None descriptor must return error |
| 3 | `test_mutation_receive_state_not_error` | FAIL | Receive state must be error path |
| 4 | `test_mutation_empty_send_queue_success` | FAIL | Empty send queue must be error path |
| 5 | `test_mutation_queue_unchanged_after_schedule` | FAIL | Queue shrinks by 1 after scheduling |
| 6 | `test_mutation_full_scheduler_returns_success` | FAIL | Full scheduler must be error path |
| 7 | `test_mutation_wf_not_preserved` | FAIL | Contradicts wf preservation |

### Logical Tests (8 tests) — ✅ All FAILED as expected
| # | Test | Result | Description |
|---|------|--------|-------------|
| 1 | `test_logical_determinism` | FAIL | Return value determinism not guaranteed |
| 2 | `test_logical_always_error` | FAIL | Cannot conclude always-error from valid preconditions |
| 3 | `test_logical_always_success` | FAIL | Cannot conclude always-success from receive state |
| 4 | `test_logical_thread_dom_changes` | FAIL | Thread dom is unchanged, cannot assert removal |
| 5 | `test_logical_queue_state_flips` | FAIL | Queue state flip not guaranteed |
| 6 | `test_logical_sender_equals_receiver` | FAIL | Sender ≠ receiver by construction |
| 7 | `test_logical_switch_decision_is_switch` | FAIL | Always NoSwitch, cannot assert Switch |
| 8 | `test_logical_rf_counter_decremented` | FAIL | rf_counter unchanged, not decremented |

## Spec Weakness Analysis

The specification has an **empty `ensures` clause**, which is a significant weakness:

1. **No return value guarantee**: The spec does not specify when the function returns `Error` vs `Else`. Any return value is permitted by the spec.
2. **No post-state guarantee**: The spec does not guarantee that `self.wf()` is preserved, that domains are unchanged, or that any specific state transition occurs.
3. **No endpoint queue mutation guarantee**: Although `schedule_blocked_thread` (called internally) has postconditions specifying the queue is dequeued, the outer function's empty ensures means callers cannot rely on these effects.

Despite the empty ensures, all our tests correctly fail because:
- Boundary tests encode self-contradictions based on precondition semantics
- Behavioral tests encode logical contradictions (assert P then assert !P)
- Logical tests try to derive unwarranted conclusions from unconstrained parameters

**Recommendation**: The `ensures` clause should be populated to specify at minimum:
- When `Error` vs `Else` is returned
- That `self.wf()` is preserved on success
- That domain sets are unchanged
- That the endpoint queue head is dequeued on success
