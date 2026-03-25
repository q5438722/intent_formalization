# Adversarial Proof Test Summary

**Target**: `kernel__syscall_receive_empty__impl0__syscall_receive_empty_no_block.rs`  
**Function**: `Kernel::syscall_receive_empty_no_block`

## Specification Overview

- **Preconditions**: `old(self).wf()`, `old(self).thread_dom().contains(receiver_thread_ptr)`, `0 <= blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS`
- **Postconditions**: **EMPTY** — the `ensures` clause specifies nothing about the return value or post-state.

## Results Summary

| Test File | Tests | Failed (Expected) | Passed (Unexpected) |
|-----------|-------|--------------------|---------------------|
| `boundary_tests.rs` | 7 | 7 | 0 |
| `behavioral_mutation_tests.rs` | 7 | 7 | 0 |
| `logical_tests.rs` | 7 | 7 | 0 |
| **Total** | **21** | **21** | **0** |

All 21 adversarial tests were **correctly rejected** by Verus — no unintended properties are entailed.

## Boundary Tests (7/7 FAILED ✓)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_boundary_endpoint_index_at_max` | Off-by-one: `MAX_NUM_ENDPOINT_DESCRIPTORS < MAX_NUM_ENDPOINT_DESCRIPTORS` is false |
| 2 | `test_boundary_thread_not_in_domain` | Thread not in domain → `thread_inv` guarantees don't apply |
| 3 | `test_boundary_endpoint_index_usize_max` | `usize::MAX < 128` is false |
| 4 | `test_boundary_endpoint_not_in_domain` | Endpoint not in domain → `endpoint_inv` guarantees don't apply |
| 5 | `test_boundary_schedule_empty_queue` | Queue length 0 contradicts `> 0` precondition |
| 6 | `test_boundary_scheduler_full` | Scheduler at max capacity contradicts `< MAX` precondition |
| 7 | `test_boundary_sender_exist_with_receive_state` | RECEIVE state contradicts `sender_exist` (requires SEND) |

## Behavioral Mutation Tests (7/7 FAILED ✓)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_mutation_schedule_queue_unchanged` | Queue must skip(1) after dequeue; asserting unchanged fails |
| 2 | `test_mutation_schedule_queue_state_changes` | Queue state is preserved; asserting it changes to RECEIVE fails |
| 3 | `test_mutation_other_endpoint_changes` | Other endpoints are preserved; asserting rf_counter changed fails |
| 4 | `test_mutation_rf_counter_changes` | rf_counter is preserved (==5); asserting ==10 fails |
| 5 | `test_mutation_thread_dom_shrinks` | Thread domain is preserved; asserting head was removed fails |
| 6 | `test_mutation_sender_exist_empty_queue` | sender_exist requires queue.len() != 0; empty queue fails |
| 7 | `test_mutation_noswitchnew_wrong_decision` | NoSwitchNew ensures NoSwitch; asserting Switch fails |

## Logical Tests (7/7 FAILED ✓)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_logical_cannot_prove_error_return` | Empty ensures → cannot prove any return value |
| 2 | `test_logical_no_state_preservation_on_error` | Empty ensures → cannot prove state preserved on error |
| 3 | `test_logical_sender_exist_stronger_len` | sender_exist requires len != 0, not len > 1 |
| 4 | `test_logical_send_equals_receive` | SEND and RECEIVE are distinct enum variants |
| 5 | `test_logical_schedule_adds_procs` | proc_dom is preserved; cannot add new procs |
| 6 | `test_logical_all_threads_blocked` | thread_inv does NOT imply all threads are BLOCKED |
| 7 | `test_logical_queued_threads_same_container` | Queued threads may belong to different containers |

## Key Finding: Missing Postconditions

The most significant specification weakness is the **empty `ensures` clause** on `syscall_receive_empty_no_block`. This means:

1. **No return value guarantees**: Callers cannot reason about whether `Error` or `Else` was returned.
2. **No state preservation guarantees**: Callers cannot reason about what changed in the kernel state.
3. **No framing guarantees**: No way to know which parts of state are unmodified.

Tests 1 and 2 in the logical suite directly expose this: even obvious properties (e.g., "if endpoint descriptor is None, return Error") cannot be proven. The specification relies entirely on the correctness of sub-functions (`schedule_blocked_thread`, `sender_exist`, `NoSwitchNew`), which do have proper specifications and correctly rejected all mutations.
