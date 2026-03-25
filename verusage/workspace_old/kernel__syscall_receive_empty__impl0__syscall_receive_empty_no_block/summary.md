# Summary: Specification Testing for `syscall_receive_empty_no_block`

## File Under Test

`kernel__syscall_receive_empty__impl0__syscall_receive_empty_no_block.rs` defines a kernel syscall for non-blocking receive on an empty endpoint. The main function `syscall_receive_empty_no_block` is an exec function with:
- **Preconditions**: `self.wf()`, receiver thread in thread domain, valid endpoint index
- **Postconditions**: **EMPTY** (no ensures clauses)

The function checks endpoint state and queue, then either returns an error or schedules a blocked sender thread.

Key testable specs are in the helper proof functions (`thread_inv`, `endpoint_inv`) and external-body accessor functions.

---

## Correctness Results (should PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_thread_inv_thread_in_container` | thread_inv: thread's container in container_dom | PASS | ✅ PASS |
| 2 | `test_thread_inv_proc_containment` | thread_inv: thread's proc in proc_dom | PASS | ✅ PASS |
| 3 | `test_thread_inv_container_owns_proc` | thread_inv: container owns thread's proc | PASS | ✅ PASS |
| 4 | `test_thread_inv_container_owns_thread` | thread_inv: container owns the thread | PASS | ✅ PASS |
| 5 | `test_thread_inv_endpoint_descriptors_wf` | thread_inv: endpoint descriptors well-formed | PASS | ✅ PASS |
| 6 | `test_thread_inv_endpoint_descriptor_exists` | thread_inv: Some descriptor → endpoint exists | PASS | ✅ PASS |
| 7 | `test_thread_inv_blocked_has_endpoint` | thread_inv: BLOCKED thread has blocking endpoint | PASS | ✅ PASS |
| 8 | `test_endpoint_inv_queue_wf` | endpoint_inv: queue is well-formed | PASS | ✅ PASS |
| 9 | `test_endpoint_inv_container_containment` | endpoint_inv: endpoint's container in domain | PASS | ✅ PASS |
| 10 | `test_endpoint_inv_queued_threads_in_domain` | endpoint_inv: queued threads in thread_dom | PASS | ✅ PASS |
| 11 | `test_endpoint_inv_queued_threads_blocked` | endpoint_inv: queued threads are BLOCKED | PASS | ✅ PASS |
| 12 | `test_sender_exist_definition` | sender_exist matches its definition | PASS | ✅ PASS |
| 13 | `test_both_invariants` | thread_inv + endpoint_inv usable together | PASS | ✅ PASS |
| 14 | `test_thread_inv_proc_container_match` | proc's container == thread's container | PASS | ✅ PASS |
| 15 | `test_kernel_wf_implies_components_wf` | Kernel wf → components wf | PASS | ✅ PASS |
| 16 | `test_get_endpoint_ptr_spec` | get_endpoint_ptr spec consistency | PASS | ✅ PASS |
| 17 | `test_thread_dom_equivalence` | Kernel.thread_dom == proc_man.thread_dom | PASS | ✅ PASS |
| 18 | `test_endpoint_dom_equivalence` | Kernel.endpoint_dom == proc_man.endpoint_dom | PASS | ✅ PASS |

**Verification output**: 61 verified, 0 errors

---

## Completeness Results (should FAIL)

### Round 1: Precondition Violations

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_thread_inv_no_wf` | Call thread_inv without wf | FAIL | ✅ FAIL |
| 2 | `test_endpoint_inv_no_wf` | Call endpoint_inv without wf | FAIL | ✅ FAIL |
| 3 | `test_get_thread_not_in_domain` | Use thread_inv for non-member thread | FAIL | ✅ FAIL |
| 4 | `test_endpoint_inv_out_of_bounds` | Access queue beyond length | FAIL | ✅ FAIL |
| 5 | `test_sender_exist_no_endpoint` | sender_exist when endpoint is None | FAIL | ✅ FAIL |

**Verification output**: 43 verified, 5 errors

### Round 2: Overly Strong Postconditions

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_thread_must_be_scheduled` | Thread always SCHEDULED (not guaranteed) | FAIL | ✅ FAIL |
| 2 | `test_endpoint_queue_always_nonempty` | Queue always non-empty (not guaranteed) | FAIL | ✅ FAIL |
| 3 | `test_all_endpoint_descriptors_some` | All descriptors are Some (not guaranteed) | FAIL | ✅ FAIL |
| 4 | `test_endpoint_always_send` | Endpoint always SEND (not guaranteed) | FAIL | ✅ FAIL |
| 5 | `test_thread_always_in_root` | Thread always in root container | FAIL | ✅ FAIL |

**Verification output**: 43 verified, 5 errors

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_negate_thread_container_in_domain` | Container NOT in domain (contradicts thread_inv) | FAIL | ✅ FAIL |
| 2 | `test_negate_thread_proc_in_domain` | Proc NOT in domain (contradicts thread_inv) | FAIL | ✅ FAIL |
| 3 | `test_negate_queued_thread_in_domain` | Queued thread NOT in domain (contradicts endpoint_inv) | FAIL | ✅ FAIL |
| 4 | `test_negate_queued_thread_blocked` | Queued thread NOT blocked (contradicts endpoint_inv) | FAIL | ✅ FAIL |
| 5 | `test_negate_container_owns_proc` | Container NOT owns proc (contradicts thread_inv) | FAIL | ✅ FAIL |

**Verification output**: 43 verified, 5 errors

### Round 4: Wrong Specific Values

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_thread_dom_is_empty` | thread_dom is empty (contradicts precondition) | FAIL | ✅ FAIL |
| 2 | `test_wrong_max_endpoint_descriptors` | MAX_NUM_ENDPOINT_DESCRIPTORS == 64 (wrong, is 128) | FAIL | ✅ FAIL |
| 3 | `test_wrong_max_threads_per_endpoint` | MAX_NUM_THREADS_PER_ENDPOINT == 64 (wrong, is 128) | FAIL | ✅ FAIL |
| 4 | `test_wrong_max_container_scheduler_len` | MAX_CONTAINER_SCHEDULER_LEN == 128 (wrong, is 10) | FAIL | ✅ FAIL |
| 5 | `test_wrong_queue_rf_counter_relationship` | queue.len == rf_counter (wrong relationship) | FAIL | ✅ FAIL |

**Verification output**: 43 verified, 5 errors

### Round 5: Cross-Function Misuse & Edge Cases

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_queued_thread_empty_scheduler` | Queued thread's container scheduler is empty | FAIL | ✅ FAIL |
| 2 | `test_sender_exist_implies_receiver_blocked` | sender_exist → receiver is BLOCKED | FAIL | ✅ FAIL |
| 3 | `test_different_threads_different_containers` | Different threads → different containers | FAIL | ✅ FAIL |
| 4 | `test_all_endpoints_same_container` | All endpoints share same container | FAIL | ✅ FAIL |
| 5 | `test_thread_endpoint_wrong_chain` | SCHEDULED thread has blocking endpoint | FAIL | ✅ FAIL |

**Verification output**: 43 verified, 5 errors

---

## Overall Assessment

### Correctness
The helper specs (`thread_inv`, `endpoint_inv`, `sender_exist`, spec accessors) are **correct**. All 18 correctness tests pass, confirming that the postconditions are valid and usable.

### Completeness
The specs are **complete for what they claim** — all 25 completeness tests correctly fail, showing the specs reject invalid assertions. The specs are tight enough to prevent precondition violations, overly strong claims, contradictions, wrong values, and invalid cross-function inferences.

### Spec Gap: Empty Ensures on Main Function
The main function `syscall_receive_empty_no_block` has an **empty ensures clause**, meaning callers receive no guarantees about the return value or post-state. The function's body verifies internally, but the spec provides no information to callers about the return value, state modifications, or preserved invariants. This is likely intentional during development but would need ensures clauses for a complete API.
