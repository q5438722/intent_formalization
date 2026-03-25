# Adversarial Test Summary: syscall_send_empty_try_schedule

## Target Function
`Kernel::syscall_send_empty_try_schedule` — IPC syscall that sends an empty message and attempts to schedule a blocked receiver thread.

**Specification:**
- **Requires:** cpu_id in range, kernel wf, CPU has current thread == sender, CPU active, sender in same container as CPU, sender in thread_dom, endpoint_index in range, sender state RUNNING
- **Ensures:** `self.wf()` (only)

## Test Results

### Boundary Tests (7/7 FAILED ✓)
| # | Test | Target | Result |
|---|------|--------|--------|
| 1 | `test_boundary_cpu_id_at_max` | cpu_id == NUM_CPUS (off-by-one) | FAIL ✓ |
| 2 | `test_boundary_endpoint_index_at_max` | endpoint_index == MAX (off-by-one) | FAIL ✓ |
| 3 | `test_boundary_thread_not_in_domain` | thread_ptr not in domain | FAIL ✓ |
| 4 | `test_boundary_cpu_no_current_thread` | CPU has no current thread | FAIL ✓ |
| 5 | `test_boundary_thread_not_running` | Thread state BLOCKED != RUNNING | FAIL ✓ |
| 6 | `test_boundary_cpu_not_active` | CPU not active | FAIL ✓ |
| 7 | `test_boundary_receiver_exist_with_send_state` | receiver_exist with SEND queue_state | FAIL ✓ |

### Behavioral Mutation Tests (7/7 FAILED ✓)
| # | Test | Target | Result |
|---|------|--------|--------|
| 1 | `test_mutation_queue_unchanged_after_run` | Queue unchanged after run_blocked_thread | FAIL ✓ |
| 2 | `test_mutation_sender_still_running` | Sender still RUNNING after schedule | FAIL ✓ |
| 3 | `test_mutation_noswitchnew_wrong_decision` | NoSwitchNew has Switch decision | FAIL ✓ |
| 4 | `test_mutation_queue_state_changes` | Queue state changed after run_blocked_thread | FAIL ✓ |
| 5 | `test_mutation_other_endpoint_changes` | Other endpoint rf_counter changed | FAIL ✓ |
| 6 | `test_mutation_receiver_exist_empty_queue` | receiver_exist with empty queue | FAIL ✓ |
| 7 | `test_mutation_cpu_still_has_thread` | CPU still has thread after schedule | FAIL ✓ |

### Logical Tests (7/7 FAILED ✓)
| # | Test | Target | Result |
|---|------|--------|--------|
| 1 | `test_logical_cannot_prove_return_type` | Return value derivable from spec | FAIL ✓ |
| 2 | `test_logical_no_state_preservation` | State preservation between two kernels | FAIL ✓ |
| 3 | `test_logical_receiver_exist_stronger_len` | queue.len() > 1 from receiver_exist | FAIL ✓ |
| 4 | `test_logical_send_equals_receive` | SEND == RECEIVE | FAIL ✓ |
| 5 | `test_logical_all_threads_running` | All threads are RUNNING | FAIL ✓ |
| 6 | `test_logical_queued_threads_same_container` | Queued threads share container | FAIL ✓ |
| 7 | `test_logical_receiver_sender_same_proc` | Container equality implies process equality | FAIL ✓ |

## Findings

**All 21 adversarial tests correctly FAIL verification**, meaning the specification properly rejects:
- Invalid inputs at precondition boundaries
- Incorrect behavioral mutations of sub-function postconditions
- Unwarranted logical inferences not entailed by the spec

**Spec Weakness Noted:** The `ensures` clause only guarantees `self.wf()`. It provides no postcondition about:
- The return value (error code, switch decision, pcid, cr3)
- What specific state changes occur on different code paths
- Whether state is preserved on error paths

This means a caller cannot reason about the syscall's outcome from the specification alone — only that the kernel remains well-formed.
