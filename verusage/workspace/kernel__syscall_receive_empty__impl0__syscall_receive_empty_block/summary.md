# Test Execution Summary: `syscall_receive_empty_block`

## Target Function
`Kernel::syscall_receive_empty_block` — handles a blocking receive-empty IPC syscall.

### Key Specification Observation
The `ensures` clause is **EMPTY**. This means the specification provides **no postconditions** to callers. The function's internal assertions (`assert(self.wf())`) confirm implementation correctness but are not exposed as guarantees.

---

## Results Overview

| Test Category          | Tests | All Failed (Expected) |
|------------------------|-------|-----------------------|
| Boundary Tests         | 7     | ✅ 7/7 failed         |
| Behavioral Mutation    | 7     | ✅ 7/7 failed         |
| Logical Tests          | 7     | ✅ 7/7 failed         |
| **Total**              | **21**| ✅ **21/21 failed**   |

All tests correctly fail verification, meaning the specification does **not** entail these undesirable properties.

---

## Boundary Tests (7/7 FAILED ✅)

| # | Test | Property Queried | Result |
|---|------|------------------|--------|
| 1 | `test_boundary_send_equals_receive` | SEND == RECEIVE | ❌ Rejected |
| 2 | `test_boundary_blocked_not_running` | BLOCKED == RUNNING | ❌ Rejected |
| 3 | `test_boundary_scheduled_not_running` | SCHEDULED == RUNNING | ❌ Rejected |
| 4 | `test_boundary_endpoint_index_at_max` | MAX < MAX (index at boundary) | ❌ Rejected |
| 5 | `test_boundary_max_threads_per_endpoint_is_zero` | MAX_THREADS == 0 | ❌ Rejected |
| 6 | `test_boundary_max_scheduler_len_is_zero` | MAX_SCHEDULER == 0 | ❌ Rejected |
| 7 | `test_boundary_empty_queue_positive_len` | 0 > 0 (empty queue valid) | ❌ Rejected |

## Behavioral Mutation Tests (7/7 FAILED ✅)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_noswitch_is_nothread` | NoSwitch → NoThread | ❌ Rejected |
| 2 | `test_mutation_nonextthread_is_noswitch` | NoThread → NoSwitch | ❌ Rejected |
| 3 | `test_mutation_noswitch_pcid_is_some` | pcid None → Some | ❌ Rejected |
| 4 | `test_mutation_empty_payload_has_va_range` | Empty payload → has va_range | ❌ Rejected |
| 5 | `test_mutation_error_code_is_else` | Error → Else | ❌ Rejected |
| 6 | `test_mutation_blocked_becomes_scheduled` | BLOCKED → SCHEDULED | ❌ Rejected |
| 7 | `test_mutation_queue_not_modified` | queue.skip(1) == queue | ❌ Rejected |

## Logical Tests (7/7 FAILED ✅)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_wf_not_trivially_true` | Any kernel is wf() | ❌ Rejected |
| 2 | `test_logical_noswitch_equals_nothread` | NoSwitch == NoThread | ❌ Rejected |
| 3 | `test_logical_switch_equals_noswitch` | Switch == NoSwitch | ❌ Rejected |
| 4 | `test_logical_sender_exist_under_receive` | sender_exist under RECEIVE | ❌ Rejected |
| 5 | `test_logical_same_decision_implies_same_error` | same decision → same error | ❌ Rejected |
| 6 | `test_logical_sender_exist_with_empty_send_queue` | sender_exist with empty SEND queue | ❌ Rejected |
| 7 | `test_logical_noswitch_cr3_is_some` | cr3 is Some for NoSwitch | ❌ Rejected |

---

## Specification Weakness Analysis

The **empty `ensures` clause** on `syscall_receive_empty_block` is the most significant finding. While the implementation internally asserts `self.wf()` and returns specific `SyscallReturnStruct` values, none of these guarantees are exposed to callers. This means:

1. **No wf() preservation guarantee** — callers cannot prove the kernel remains well-formed after the call.
2. **No return value guarantees** — callers cannot reason about error codes or switch decisions.
3. **No state change guarantees** — callers cannot prove what happened to threads, endpoints, or queues.

The helper functions (`NoSwitchNew`, `NoNextThreadNew`, `block_running_thread_and_set_trap_frame`, etc.) have strong postconditions, but these are invisible through the empty ensures of the top-level syscall.
