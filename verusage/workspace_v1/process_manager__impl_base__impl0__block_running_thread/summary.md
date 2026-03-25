# Adversarial Proof Test Results: `block_running_thread`

## Target
`ProcessManager::block_running_thread(thread_ptr, endpoint_index, ipc_payload)` — blocks a RUNNING thread by pushing it onto an endpoint queue, setting state to BLOCKED.

## Summary

| Test File | Tests | Failed (Expected) | Passed (Unexpected) |
|-----------|-------|--------------------|---------------------|
| boundary_tests.rs | 6 | 6 ✅ | 0 |
| behavioral_mutation_tests.rs | 6 | 6 ✅ | 0 |
| logical_tests.rs | 5 | 5 ✅ | 0 |
| **Total** | **17** | **17 ✅** | **0** |

All 17 adversarial tests **failed verification** as expected, meaning the specification correctly rejects:
- Invalid inputs (boundary violations)
- Incorrect output behaviors (mutations)
- Unintended logical inferences

## Boundary Tests (6/6 FAILED ✅)

| Test | Violated Precondition | Result |
|------|-----------------------|--------|
| B1: `endpoint_index_at_max` | `endpoint_index >= 128` | FAIL ✅ |
| B2: `thread_not_in_domain` | `!thread_dom.contains(t_ptr)` | FAIL ✅ |
| B3: `thread_blocked_not_running` | `state == BLOCKED` | FAIL ✅ |
| B4: `thread_scheduled_not_running` | `state == SCHEDULED` | FAIL ✅ |
| B5: `endpoint_descriptor_none` | `endpoint_descriptors[idx].is_None()` | FAIL ✅ |
| B6: `endpoint_queue_full` | `queue.len() == MAX` | FAIL ✅ |

**Conclusion**: All preconditions are necessary — removing any one makes the spec unprovable.

## Behavioral Mutation Tests (6/6 FAILED ✅)

| Test | Mutated Postcondition | Result |
|------|----------------------|--------|
| M1: `thread_stays_running` | state == RUNNING (should be BLOCKED) | FAIL ✅ |
| M2: `queue_unchanged` | queue@ unchanged (should have push) | FAIL ✅ |
| M3: `proc_domain_changed` | proc_dom != old (should be ==) | FAIL ✅ |
| M4: `container_changed` | container differs (should be preserved) | FAIL ✅ |
| M5: `ipc_payload_wrong` | ipc_payload is Empty (should match input) | FAIL ✅ |
| M6: `endpoint_queue_state_changed` | queue_state changed (should be preserved) | FAIL ✅ |

**Conclusion**: The spec correctly constrains all specified output relationships — mutating any one is rejected.

## Logical Tests (5/5 FAILED ✅)

| Test | Non-guaranteed Property | Result |
|------|------------------------|--------|
| L1: `determinism` | Two executions yield identical state | FAIL ✅ |
| L2: `error_code_preserved` | Thread error_code unchanged | FAIL ✅ |
| L3: `trap_frame_preserved` | Thread trap_frame unchanged | FAIL ✅ |
| L4: `owning_proc_preserved` | Thread owning_proc unchanged | FAIL ✅ |
| L5: `endpoint_rf_counter_preserved` | Endpoint rf_counter unchanged | FAIL ✅ |

**Conclusion**: The spec does not entail these unintended properties:
- **L1**: Non-determinism is expected — the spec is relational and doesn't fully constrain all thread fields.
- **L2-L3**: `error_code` and `trap_frame` are preserved by the implementation (via `thread_set_...` helper) but NOT stated in `block_running_thread`'s `ensures` clause. This is a **spec incompleteness** — the spec is weaker than the implementation.
- **L4**: `owning_proc` is theoretically derivable from `wf()` invariants, but the SMT solver cannot chain the multi-step reasoning required.
- **L5**: `rf_counter` preservation is derivable from `wf()` + `owning_threads` preservation, but again the reasoning chain is too complex for the solver.

## Spec Quality Assessment

The specification of `block_running_thread` is **reasonably strong**:
- All boundary conditions are properly enforced
- All specified behavioral contracts are correctly constrained
- No unintended logical properties are inadvertently entailed

**Potential improvements** (from logical test findings):
1. Add `error_code` preservation to `ensures` (implementation preserves it)
2. Add `trap_frame` preservation to `ensures` (implementation preserves it)
3. Optionally add `owning_proc` and `owning_container` preservation for easier downstream reasoning
