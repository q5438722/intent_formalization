# Adversarial Proof Test Summary

**Target**: `process_manager__impl_base__impl0__schedule_blocked_thread.rs`
**Function**: `ProcessManager::schedule_blocked_thread(&mut self, endpoint_ptr: EndpointPtr)`

## Function Description

`schedule_blocked_thread` pops the head thread from an endpoint's blocking queue, transitions it from `BLOCKED` to `SCHEDULED`, and pushes it onto its owning container's scheduler.

## Test Results

All **15 tests** across 3 categories **FAILED verification** as expected, confirming the specification correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (5/5 FAILED ✓)

| # | Test Name | Property Tested | Result |
|---|-----------|----------------|--------|
| 1 | `test_boundary_va_zero_invalid` | `spec_va_4k_valid(0)` should be rejected (L4 index too low) | FAIL ✓ |
| 2 | `test_boundary_blocked_equals_running` | `BLOCKED == RUNNING` is false (distinct enum variants) | FAIL ✓ |
| 3 | `test_boundary_blocked_without_endpoint` | Under `wf()`, BLOCKED thread must have `blocking_endpoint_ptr.is_Some()` | FAIL ✓ |
| 4 | `test_boundary_running_without_cpu` | Under `wf()`, RUNNING thread must have `running_cpu.is_Some()` | FAIL ✓ |
| 5 | `test_boundary_inactive_cpu_has_thread` | Under `wf()`, inactive CPU must have `current_thread.is_None()` | FAIL ✓ |

### Behavioral Mutation Tests (5/5 FAILED ✓)

| # | Test Name | Property Tested | Result |
|---|-----------|----------------|--------|
| 1 | `test_mutation_queue_unchanged` | Endpoint queue must change (postcondition says `skip(1)`) | FAIL ✓ |
| 2 | `test_mutation_rf_counter_decreased` | `rf_counter` is preserved, not decremented | FAIL ✓ |
| 3 | `test_mutation_container_dom_changed` | `container_dom()` is preserved | FAIL ✓ |
| 4 | `test_mutation_process_changed` | All processes are unchanged | FAIL ✓ |
| 5 | `test_mutation_queue_state_changed` | `queue_state` is preserved | FAIL ✓ |

### Logical Tests (5/5 FAILED ✓)

| # | Test Name | Property Tested | Result |
|---|-----------|----------------|--------|
| 1 | `test_logical_thread_running_after_schedule` | Thread should NOT be RUNNING (it's SCHEDULED) | FAIL ✓ |
| 2 | `test_logical_queue_empty_after_schedule` | Queue with >1 elements is NOT empty after `skip(1)` | FAIL ✓ |
| 3 | `test_logical_scheduled_in_endpoint_queue` | SCHEDULED thread cannot be in an endpoint queue under `wf()` | FAIL ✓ |
| 4 | `test_logical_removed_thread_still_in_queue` | Removed head thread is NOT contained in `skip(1)` (no\_duplicates) | FAIL ✓ |
| 5 | `test_logical_same_proc_different_containers` | Threads sharing `owning_proc` must share `owning_container` | FAIL ✓ |

## Conclusion

The specification for `schedule_blocked_thread` is **consistent** with respect to all 15 adversarial queries:

- **Boundary**: Invalid inputs are correctly rejected by `wf()` invariants (`endpoints_queue_wf`, `threads_cpu_wf`, `cpus_wf`).
- **Behavioral**: Mutated postconditions (unchanged queue, decremented counter, changed domains/processes/state) are correctly rejected.
- **Logical**: Unintended properties (wrong thread state, premature queue emptiness, cross-invariant violations, duplicate containment) are correctly rejected.

No spec weaknesses were detected.
