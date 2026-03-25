# Test Summary: `block_running_thread_and_change_queue_state`

## Target Function
`ProcessManager::block_running_thread_and_change_queue_state(thread_ptr, endpoint_index, ipc_payload, queue_state)`

Transitions a RUNNING thread to BLOCKED state, pushes it onto the target endpoint's queue, sets the endpoint's `queue_state` to the given input value, and stores the IPC payload.

## Results Overview

| Test File | Tests | Failed (Expected) | Passed (Unexpected) |
|-----------|-------|--------------------|---------------------|
| `boundary_tests.rs` | 6 | 6 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 6 | 6 ✅ | 0 |
| `logical_tests.rs` | 6 | 6 ✅ | 0 |
| **Total** | **18** | **18 ✅** | **0** |

All 18 adversarial tests were correctly rejected by Verus, meaning the specification is consistent with respect to all tested properties.

---

## Boundary Tests (precondition violations)

| Test | Property Violated | Result |
|------|-------------------|--------|
| B1 | `endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS` (index at max) | FAILED ✅ |
| B2 | `thread_dom().contains(thread_ptr)` (thread not in domain) | FAILED ✅ |
| B3 | `state == RUNNING` (thread is BLOCKED) | FAILED ✅ |
| B4 | `state == RUNNING` (thread is SCHEDULED) | FAILED ✅ |
| B5 | `endpoint_descriptors[idx].is_Some()` (descriptor is None) | FAILED ✅ |
| B6 | `queue.len() < MAX_NUM_THREADS_PER_ENDPOINT` (queue full) | FAILED ✅ |

**Conclusion:** All preconditions are effective — invalid inputs are properly rejected.

---

## Behavioral Mutation Tests (output mutations)

| Test | Mutated Property | Result |
|------|------------------|--------|
| M1 | Thread state stays RUNNING (should be BLOCKED) | FAILED ✅ |
| M2 | Endpoint queue unchanged (should have push) | FAILED ✅ |
| M3 | Process domain changed (should be preserved) | FAILED ✅ |
| M4 | Container state changed (should be preserved) | FAILED ✅ |
| M5 | IPC payload is Empty (should match input) | FAILED ✅ |
| M6 | Queue state not set to input (SEND→claim RECEIVE) | FAILED ✅ |

**Conclusion:** All behavioral postconditions are tight — incorrect outputs are properly rejected. Notably, M6 tests the key differentiator from `block_running_thread`: the queue state is correctly constrained to the input `queue_state` parameter.

---

## Logical Tests (unstated properties)

| Test | Unstated Property Tested | Result |
|------|--------------------------|--------|
| L1 | Determinism (two outputs equal) | FAILED ✅ |
| L2 | Thread `error_code` preserved | FAILED ✅ |
| L3 | Thread `trap_frame` preserved | FAILED ✅ |
| L4 | Thread `owning_proc` preserved | FAILED ✅ |
| L5 | Endpoint `rf_counter` preserved | FAILED ✅ |
| L6 | Thread `running_cpu` is None | FAILED ✅ |

**Conclusion:** The specification correctly does NOT entail these unstated properties:
- **L1:** The relational spec is non-deterministic — thread fields like `error_code`, `trap_frame`, `blocking_endpoint_ptr`, and CPU state are unconstrained.
- **L2–L3:** `error_code` and `trap_frame` preservation is not guaranteed by the spec (potential spec weakness — the implementation does preserve them via the utility function).
- **L4:** `owning_proc` preservation requires multi-step reasoning through `wf()` + process invariants that the solver cannot chain automatically.
- **L5:** `rf_counter` preservation requires chaining `wf()` constraints (`rf_counter == owning_threads.len()`) across old and new states.
- **L6:** `running_cpu.is_None()` for BLOCKED threads is implied by `wf()` but not directly stated.

### Spec Weakness Observations

The logical test failures reveal that the specification is **intentionally under-specified** for certain thread fields (`error_code`, `trap_frame`, `owning_proc`, `owning_container`, `running_cpu`, `blocking_endpoint_ptr`). While the implementation preserves or correctly sets these through the `thread_set_blocking_endpoint_...` utility function, the **relational postcondition does not explicitly guarantee** all of them. This is a potential source of spec incompleteness if callers rely on these preservation properties.
