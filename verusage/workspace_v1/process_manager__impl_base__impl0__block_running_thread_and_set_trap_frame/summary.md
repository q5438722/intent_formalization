# Adversarial Proof Test Summary

## Target
`process_manager__impl_base__impl0__block_running_thread_and_set_trap_frame.rs`

Function: `ProcessManager::block_running_thread_and_set_trap_frame` — blocks a running thread, sets its IPC payload & trap frame, pushes it onto an endpoint queue, and clears the CPU's current thread.

## Results Overview

| Test File | Tests | All Failed (as expected) |
|-----------|-------|------------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (5/5 errors) |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5/5 errors) |
| `logical_tests.rs` | 5 | ✅ Yes (5/5 errors) |

**All 15 adversarial tests correctly FAIL verification**, confirming the spec rejects the tested invalid properties.

---

## Boundary Tests (Precondition / Invariant Violations)

| # | Test | Property Tested | Result |
|---|------|----------------|--------|
| 1 | `test_boundary_blocked_with_running_cpu` | BLOCKED thread must not have `running_cpu.is_Some()` (threads_cpu_wf) | ✅ FAIL |
| 2 | `test_boundary_inactive_cpu_with_thread` | Inactive CPU must not have `current_thread` (cpus_wf) | ✅ FAIL |
| 3 | `test_boundary_blocked_without_endpoint` | BLOCKED thread must have `blocking_endpoint_ptr` (endpoints_queue_wf) | ✅ FAIL |
| 4 | `test_boundary_in_queue_but_running` | Thread in endpoint queue must be BLOCKED (endpoints_queue_wf) | ✅ FAIL |
| 5 | `test_boundary_endpoint_index_oob` | `endpoint_index >= MAX_NUM_ENDPOINT_DESCRIPTORS` rejected | ✅ FAIL |

## Behavioral Mutation Tests (Incorrect Output Rejection)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_state_stays_running` | Thread state stays RUNNING (should be BLOCKED) | ✅ FAIL |
| 2 | `test_mutation_queue_not_pushed` | Queue length unchanged (should increase by 1) | ✅ FAIL |
| 3 | `test_mutation_ipc_not_set` | IPC payload not set to input (should equal input) | ✅ FAIL |
| 4 | `test_mutation_queue_state_changes` | Endpoint queue_state changes (should be preserved) | ✅ FAIL |
| 5 | `test_mutation_other_thread_changes` | Other thread state changed (should be preserved) | ✅ FAIL |

## Logical Tests (Unguaranteed Property Probing)

| # | Test | Unguaranteed Property | Result | Spec Gap? |
|---|------|-----------------------|--------|-----------|
| 1 | `test_logical_trap_frame_set` | `trap_frame.is_Some()` after blocking | ✅ FAIL | **Yes** — implementation sets trap frame but postcondition omits it |
| 2 | `test_logical_error_code_preserved` | `error_code` preserved after blocking | ✅ FAIL | **Yes** — postcondition doesn't specify error_code preservation |
| 3 | `test_logical_rf_counter_preserved` | Endpoint `rf_counter` preserved | ✅ FAIL | **Yes** — postcondition preserves queue_state/owning_threads but not rf_counter |
| 4 | `test_logical_blocking_endpoint_set` | `blocking_endpoint_ptr` points to target endpoint | ✅ FAIL | **Yes** — postcondition doesn't specify which endpoint the thread blocks on |
| 5 | `test_logical_endpoint_owning_container_preserved` | Endpoint `owning_container` preserved | ✅ FAIL | **Yes** — postcondition omits owning_container for the target endpoint |

---

## Identified Spec Gaps

The logical tests reveal **5 properties** the implementation establishes but the postcondition does **not** guarantee:

1. **Trap frame**: `thread_set_trap_frame_fast` sets `trap_frame` to `*pt_regs`, but postcondition is silent.
2. **Error code**: The thread's `error_code` is preserved by the helper functions, but not postconditioned.
3. **Endpoint rf_counter**: Preserved by `endpoint_push`, but postcondition only specifies `queue_state` and `owning_threads`.
4. **Blocking endpoint identity**: The thread blocks on the endpoint at `endpoint_descriptors[endpoint_index]`, but postcondition only says state is BLOCKED (not which endpoint).
5. **Endpoint owning_container**: Preserved by `endpoint_push`, but not in postcondition.

These gaps mean callers cannot rely on these properties without additional reasoning through `wf()` invariants, potentially limiting compositional verification.
