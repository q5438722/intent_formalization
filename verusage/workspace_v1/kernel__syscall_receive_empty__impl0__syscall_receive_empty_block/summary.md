# Test Summary: `syscall_receive_empty_block`

## Target Function
`Kernel::syscall_receive_empty_block` — handles a receiver thread blocking on an empty endpoint for IPC.

## Critical Specification Observation
The function has an **EMPTY `ensures` clause** — it provides NO postconditions. This means the specification guarantees nothing about the return value, kernel state changes, or thread/endpoint mutations after the call. This is a significant spec weakness.

## Results: All 15 tests FAILED verification as intended (5 per category)

### Boundary Tests (5/5 failed ✓)
| # | Test | Property Challenged |
|---|------|-------------------|
| 1 | `test_boundary_send_is_receive` | `EndpointState::SEND` ≠ `RECEIVE` |
| 2 | `test_boundary_receive_is_send` | `EndpointState::RECEIVE` ≠ `SEND` |
| 3 | `test_boundary_blocked_is_not_running` | `ThreadState::BLOCKED` ≠ `RUNNING` |
| 4 | `test_boundary_scheduled_is_not_running` | `ThreadState::SCHEDULED` ≠ `RUNNING` |
| 5 | `test_boundary_max_endpoint_descriptors_nonzero` | `MAX_NUM_ENDPOINT_DESCRIPTORS` ≠ 0 |

**Verdict**: The spec correctly distinguishes enum variants and rejects invalid boundary values.

### Behavioral Mutation Tests (5/5 failed ✓)
| # | Test | Mutation Applied |
|---|------|-----------------|
| 1 | `test_mutation_noswitch_returns_nothread` | `NoSwitch` → assert `NoThread` |
| 2 | `test_mutation_nonextthread_returns_noswitch` | `NoThread` → assert `NoSwitch` |
| 3 | `test_mutation_noswitch_pcid_some` | `pcid.is_None()` → assert `is_Some()` |
| 4 | `test_mutation_empty_payload_has_va_range` | `Empty` payload → assert has va_range |
| 5 | `test_mutation_noswitch_error_is_else` | `error_code == Error` → assert `Else` |

**Verdict**: Helper function specs (`NoSwitchNew`, `NoNextThreadNew`, `IPCPayLoad`) correctly reject mutated behaviors.

### Logical Tests (5/5 failed ✓)
| # | Test | Unintended Property Tested |
|---|------|--------------------------|
| 1 | `test_logical_noswitch_equals_nothread` | `NoSwitch == NoThread` (conflating decisions) |
| 2 | `test_logical_noswitch_nonext_same_decision` | Two structs with different decisions are same |
| 3 | `test_logical_sender_exist_under_receive` | `sender_exist` holds under RECEIVE state |
| 4 | `test_logical_wf_not_trivially_true` | `wf()` holds for any arbitrary kernel |
| 5 | `test_logical_switch_equals_noswitch` | `Switch == NoSwitch` (conflating decisions) |

**Verdict**: The spec correctly rejects unintended logical inferences. Enum variants are provably distinct, `sender_exist` requires `SEND` state, and `wf()` is non-trivial.

## Specification Weakness Identified
The **empty `ensures` clause** on `syscall_receive_empty_block` is the primary weakness. While the helper functions and types have adequate specs, the main syscall function provides zero guarantees about:
- Return value correctness (which `SyscallReturnStruct` variant is returned)
- Kernel well-formedness preservation (`self.wf()` after call)
- Thread state transitions (receiver becoming BLOCKED)
- Endpoint queue mutations (thread added to queue)
- Endpoint state transitions (SEND → RECEIVE when empty queue)

This means a caller cannot reason about the effects of `syscall_receive_empty_block` from its specification alone.
