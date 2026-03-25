# Test Summary: `endpoint_inv` (ProcessManager)

## Target
`process_manager__spec_proof__impl2__endpoint_inv.rs` — Proves that for a well-formed `ProcessManager`:
1. Every endpoint's queue is well-formed and its owning container is in the container domain.
2. Every thread in an endpoint's queue is in the thread domain and has state `BLOCKED`.

## Results

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary | 7 | 7 ✓ | 0 |
| Behavioral Mutation | 7 | 7 ✓ | 0 |
| Logical | 7 | 7 ✓ | 0 |
| **Total** | **21** | **21** | **0** |

All 21 adversarial tests **failed verification** as expected, confirming the specification correctly rejects the tested invalid properties.

---

## Boundary Tests (7/7 failed ✓)

| # | Test | Failure Mode |
|---|---|---|
| 1 | `test_boundary_no_wf_container_in_dom` | Partial wf (endpoint_perms_wf + container_perms_wf) insufficient to prove owning_container ∈ container_dom |
| 2 | `test_boundary_endpoint_not_in_dom` | Cannot dereference endpoint not in domain |
| 3 | `test_boundary_queue_index_at_len` | Off-by-one: queue@[len] is out of bounds |
| 4 | `test_boundary_zero_endpoint_in_dom` | 0usize not guaranteed in endpoint_dom |
| 5 | `test_boundary_nonempty_endpoints` | wf() does not imply endpoint_dom is non-empty |
| 6 | `test_boundary_endpoint_ptr_in_thread_dom` | Endpoint ptr ∉ thread_dom (disjoint memory) |
| 7 | `test_boundary_negative_queue_index` | Negative index (-1) out of bounds |

## Behavioral Mutation Tests (7/7 failed ✓)

| # | Test | Failure Mode |
|---|---|---|
| 1 | `test_mutation_queue_thread_running` | Queue thread state ≠ RUNNING (must be BLOCKED) |
| 2 | `test_mutation_queue_thread_scheduled` | Queue thread state ≠ SCHEDULED (must be BLOCKED) |
| 3 | `test_mutation_thread_not_in_dom` | Queue thread IS in thread_dom |
| 4 | `test_mutation_owning_container_not_in_dom` | Owning container IS in container_dom |
| 5 | `test_mutation_queue_not_wf` | Queue IS well-formed |
| 6 | `test_mutation_last_queue_thread_running` | Last queue element also BLOCKED, not RUNNING |
| 7 | `test_mutation_arbitrary_thread_not_blocked` | Any thread in queue must be BLOCKED |

## Logical Tests (7/7 failed ✓)

| # | Test | Failure Mode |
|---|---|---|
| 1 | `test_logical_blocked_thread_implies_nonempty_sibling` | Queue with 1 blocked thread need not have ≥2 |
| 2 | `test_logical_queue_thread_same_container` | Queue thread's container ≠ endpoint's container not provable |
| 3 | `test_logical_queue_length_bounded` | Queue length not bounded by MAX_NUM_THREADS_PER_ENDPOINT |
| 4 | `test_logical_state_iff_queue_empty` | Empty queue ⇏ endpoint state is RECEIVE |
| 5 | `test_logical_rf_counter_equals_queue_len` | rf_counter ≠ queue.len() not guaranteed |
| 6 | `test_logical_determinism` | Two wf() PMs need not have identical endpoint queues |
| 7 | `test_logical_queue_thread_has_descriptor` | Queue thread need not have endpoint in its descriptors |

---

## Findings During Development

Two initially-designed tests **passed unexpectedly**, revealing properties that ARE entailed by the spec (though not stated in `endpoint_inv`'s postconditions):

1. **Converse of BLOCKED → in queue**: The spec's `endpoints_queue_wf()` guarantees that BLOCKED threads have a `blocking_endpoint_ptr` pointing to an endpoint whose queue contains them. So the converse (all BLOCKED threads are in some queue) IS entailed.

2. **Cross-queue uniqueness**: Since `blocking_endpoint_ptr` is a single-valued field, a thread can only be in one endpoint queue. The spec's bidirectional consistency between thread state and queue membership ensures this.

These properties are valid consequences of the broader `wf()` invariant (specifically `endpoints_queue_wf` inside `internal_wf`), even though `endpoint_inv` doesn't explicitly export them.

3. **Partial-wf sufficiency for queue.wf()**: The original boundary test showed `endpoint_perms_wf()` alone is sufficient to derive `queue.wf()`, since `endpoint_perms_wf` directly includes `queue.wf()` in its definition. This means the `endpoint_inv` postcondition about `queue.wf()` is somewhat redundant — it's already guaranteed by a weaker precondition than `wf()`.
