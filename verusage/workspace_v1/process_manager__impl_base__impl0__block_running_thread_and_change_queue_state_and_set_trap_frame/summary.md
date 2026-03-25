# Test Summary: `block_running_thread_and_change_queue_state_and_set_trap_frame`

## Overview

15 adversarial proof tests were generated across 3 categories to probe the semantic boundary of the specification for `block_running_thread_and_change_queue_state_and_set_trap_frame`. All 15 tests **FAIL verification** as intended, confirming the spec correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning (within the tested scope).

---

## Results

### Boundary Tests (5/5 FAIL ✓)

| Test | Target | Result |
|------|--------|--------|
| `test_boundary_va_zero_not_4k_valid` | VA=0 rejected by `spec_va_4k_valid` (L4 index 0 < 1) | FAIL ✓ |
| `test_boundary_blocked_thread_has_running_cpu` | BLOCKED thread cannot have `running_cpu.is_Some()` per `threads_cpu_wf` | FAIL ✓ |
| `test_boundary_inactive_cpu_has_thread` | Inactive CPU cannot have `current_thread` per `cpus_wf` | FAIL ✓ |
| `test_boundary_blocked_is_not_running` | `ThreadState::BLOCKED != ThreadState::RUNNING` | FAIL ✓ |
| `test_boundary_queue_at_max_capacity` | Queue at `MAX_NUM_THREADS_PER_ENDPOINT` violates `< MAX` precondition | FAIL ✓ |

### Behavioral Mutation Tests (5/5 FAIL ✓)

| Test | Target | Result |
|------|--------|--------|
| `test_mutation_thread_state_running_after_block` | State must be BLOCKED, not RUNNING | FAIL ✓ |
| `test_mutation_thread_state_scheduled_after_block` | State must be BLOCKED, not SCHEDULED | FAIL ✓ |
| `test_mutation_queue_unchanged_after_push` | Queue length must increase after push | FAIL ✓ |
| `test_mutation_ipc_payload_not_preserved` | New payload matches input, not old payload | FAIL ✓ |
| `test_mutation_queue_state_wrong` | Queue state matches input (RECEIVE ≠ SEND) | FAIL ✓ |

### Logical Tests (5/5 FAIL ✓)

| Test | Target | Result |
|------|--------|--------|
| `test_logical_trap_frame_not_guaranteed` | **Spec weakness**: `ensures` never mentions `trap_frame` despite fn name "set_trap_frame" | FAIL ✓ |
| `test_logical_running_cpu_not_explicit` | `running_cpu.is_None()` derivable from `wf()` + BLOCKED but not explicit in `ensures` | FAIL ✓ |
| `test_logical_error_code_not_preserved` | `error_code` preservation not stated in `ensures` | FAIL ✓ |
| `test_logical_blocking_endpoint_not_explicit` | `blocking_endpoint_ptr.is_Some()` derivable from `wf()` + BLOCKED but not explicit | FAIL ✓ |
| `test_logical_cpu_thread_cleared` | CPU's `current_thread` clearing not stated in `ensures` | FAIL ✓ |

---

## Spec Weakness Identified

**Missing `trap_frame` postcondition**: The function is named `block_running_thread_and_change_queue_state_and_set_trap_frame`, and internally calls `thread_set_trap_frame_fast` which guarantees `trap_frame.is_Some()` and `trap_frame.unwrap() == *pt_regs`. However, the outer function's `ensures` clause **does not expose** any guarantee about the trap frame to callers. This means callers have no formal assurance that the trap frame was set, despite it being the function's documented purpose.

**Other implicit properties**: Several properties (`running_cpu`, `blocking_endpoint_ptr`, `error_code`, `cpu_list` changes) are derivable from `wf()` but not explicitly stated in the `ensures`. While `wf()` is preserved (so these can be recovered via `reveal_process_manager_wf`), making them explicit would improve spec clarity and usability.
