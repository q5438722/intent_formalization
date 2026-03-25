# Test Summary: `syscall_send_empty_block`

## Target Function
`Kernel::syscall_send_empty_block` — a blocking IPC send syscall that either blocks the sender thread on an endpoint or schedules a waiting receiver.

### Specification
- **Preconditions**: `self.wf()`, thread in domain, `0 <= endpoint_index < 128`, thread state `RUNNING`
- **Postconditions**: `self.wf()` only (very weak — no guarantees on return value, state transitions, or frame conditions)

---

## Results: All 21 tests FAILED verification ✅

### Boundary Tests (7/7 failed)
| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_endpoint_index_at_max` | Off-by-one: `MAX == 128 < 128` is false | FAIL ✅ |
| `test_boundary_thread_not_in_domain` | Access thread outside domain | FAIL ✅ |
| `test_boundary_endpoint_index_usize_max` | `usize::MAX < 128` is false | FAIL ✅ |
| `test_boundary_endpoint_not_in_domain` | Access endpoint outside domain | FAIL ✅ |
| `test_boundary_schedule_empty_queue` | `queue.len() == 0` contradicts `> 0` | FAIL ✅ |
| `test_boundary_scheduler_full` | Full scheduler contradicts `< MAX` | FAIL ✅ |
| `test_boundary_thread_not_running` | BLOCKED ≠ RUNNING | FAIL ✅ |

### Behavioral Mutation Tests (7/7 failed)
| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_mutation_block_queue_unchanged` | Queue should have appended thread | FAIL ✅ |
| `test_mutation_block_queue_state_stays_receive` | Queue state changed to SEND | FAIL ✅ |
| `test_mutation_schedule_queue_unchanged` | Queue should skip(1) after dequeue | FAIL ✅ |
| `test_mutation_thread_stays_running` | Thread transitions to BLOCKED | FAIL ✅ |
| `test_mutation_rf_counter_changes` | rf_counter preserved, not changed to 10 | FAIL ✅ |
| `test_mutation_noswitchnew_wrong_decision` | NoSwitch ≠ Switch | FAIL ✅ |
| `test_mutation_nonextthreadnew_wrong_decision` | NoThread ≠ NoSwitch | FAIL ✅ |

### Logical Tests (7/7 failed)
| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_logical_cannot_prove_error_return` | No ensures on return value | FAIL ✅ |
| `test_logical_no_thread_dom_preservation` | Two wf kernels may differ | FAIL ✅ |
| `test_logical_receiver_exist_stronger_len` | `len != 0` does not imply `> 1` | FAIL ✅ |
| `test_logical_send_equals_receive` | SEND ≠ RECEIVE (enum distinctness) | FAIL ✅ |
| `test_logical_all_threads_running` | Not all threads are RUNNING | FAIL ✅ |
| `test_logical_queued_threads_same_container` | Queued threads may be from different containers | FAIL ✅ |
| `test_logical_endpoint_descriptors_deterministic` | Two wf kernels with same thread may have different descriptors | FAIL ✅ |

---

## Specification Weakness Analysis

The postcondition (`ensures self.wf()`) is notably weak:
1. **No return value guarantees** — the spec does not constrain which `SyscallReturnStruct` variant is returned in each branch
2. **No frame conditions** — the spec does not guarantee which parts of state are preserved vs modified
3. **No thread state transition guarantees** — the sender's state transition to BLOCKED is not specified in the ensures

However, the **helper function specifications** (`schedule_blocked_thread`, `block_running_thread_and_set_trap_frame`, etc.) are well-specified with rich postconditions, and the behavioral mutation tests confirm these sub-specifications correctly reject mutated behaviors.
