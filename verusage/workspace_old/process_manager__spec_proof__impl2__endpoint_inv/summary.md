# Summary: endpoint_inv Specification Testing

## File Under Test
`process_manager__spec_proof__impl2__endpoint_inv.rs` — Defines `ProcessManager::endpoint_inv()`, a proof function that derives endpoint invariants from the overall well-formedness condition `self.wf()`.

### `endpoint_inv` Specification
- **Requires**: `self.wf()`
- **Ensures**:
  1. For all endpoints `e_ptr` in `endpoint_dom()`: `queue.wf()` and `container_dom().contains(owning_container)`
  2. For all endpoints `e_ptr` and indices `i` in `0..queue.len()`: `thread_dom().contains(queue@[i])` and `get_thread(queue@[i]).state == ThreadState::BLOCKED`

---

## Correctness Results (should all PASS)

| Test | Description | Expected | Actual |
|------|------------|----------|--------|
| `test_param_endpoint_queue_wf` | queue.wf() holds for any endpoint | PASS | ✅ PASS |
| `test_param_endpoint_owning_container_in_dom` | owning_container in container_dom | PASS | ✅ PASS |
| `test_param_queue_thread_in_dom` | queued thread in thread_dom | PASS | ✅ PASS |
| `test_param_queue_thread_is_blocked` | queued thread has BLOCKED state | PASS | ✅ PASS |
| `test_param_queue_thread_both_postconditions` | both thread postconditions together | PASS | ✅ PASS |
| `test_param_endpoint_first_ensures` | first ensures clause (wf + container) | PASS | ✅ PASS |
| `test_param_two_endpoints` | two endpoints both satisfy invariants | PASS | ✅ PASS |
| `test_param_two_queue_indices` | two indices in same queue | PASS | ✅ PASS |
| `test_param_vacuous` | vacuous case (no specific endpoint) | PASS | ✅ PASS |
| `test_param_endpoint_inv_comprehensive` | all postconditions together | PASS | ✅ PASS |

**Verification output**: `33 verified, 0 errors`

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| `test_no_preconditions` | Call endpoint_inv with no requires | FAIL | ❌ FAIL |
| `test_only_container_perms_wf` | Only container_perms_wf, not full wf | FAIL | ❌ FAIL |
| `test_only_proc_perms_wf` | Only proc_perms_wf | FAIL | ❌ FAIL |
| `test_only_endpoint_perms_wf` | Only endpoint_perms_wf | FAIL | ❌ FAIL |
| `test_no_internal_wf` | All field wf but missing internal_wf | FAIL | ❌ FAIL |

**Verification output**: `23 verified, 5 errors`

### Round 2: Overly Strong Postconditions

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| `test_queue_is_empty` | Assert queue is empty | FAIL | ❌ FAIL |
| `test_queue_len_bounded` | Assert queue length ≤ 10 | FAIL | ❌ FAIL |
| `test_thread_state_running` | Assert thread is RUNNING (not BLOCKED) | FAIL | ❌ FAIL |
| `test_endpoint_state_receive` | Assert queue_state is RECEIVE | FAIL | ❌ FAIL |
| `test_rf_counter_zero` | Assert rf_counter == 0 | FAIL | ❌ FAIL |

**Verification output**: `23 verified, 5 errors`

### Round 3: Negated/Contradicted Postconditions

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| `test_negate_queue_wf` | Assert !queue.wf() | FAIL | ❌ FAIL |
| `test_negate_container_in_dom` | Assert owning_container NOT in dom | FAIL | ❌ FAIL |
| `test_negate_thread_in_dom` | Assert queued thread NOT in dom | FAIL | ❌ FAIL |
| `test_thread_not_blocked` | Assert state != BLOCKED | FAIL | ❌ FAIL |
| `test_thread_scheduled_not_blocked` | Assert state == SCHEDULED | FAIL | ❌ FAIL |

**Verification output**: `23 verified, 5 errors`

### Round 4: Wrong Specific Values

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| `test_owning_container_is_zero` | Assert owning_container == 0 | FAIL | ❌ FAIL |
| `test_first_thread_is_specific` | Assert queue@[0] == 42 | FAIL | ❌ FAIL |
| `test_queue_len_exactly_one` | Assert queue.len() == 1 | FAIL | ❌ FAIL |
| `test_queue_elements_equal` | Assert queue@[i] == queue@[j] for i≠j | FAIL | ❌ FAIL |
| `test_owning_container_is_root` | Assert owning_container == root_container | FAIL | ❌ FAIL |

**Verification output**: `23 verified, 5 errors`

### Round 5: Cross-Function Misuse & Edge Cases

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| `test_endpoint_in_thread_dom` | Assert endpoint_ptr in thread_dom | FAIL | ❌ FAIL |
| `test_endpoint_in_proc_dom` | Assert endpoint_ptr in proc_dom | FAIL | ❌ FAIL |
| `test_out_of_range_queue` | Access queue@[len] (out of range) | FAIL | ❌ FAIL |
| `test_thread_owns_endpoint` | Assert queued thread's container == endpoint's container | FAIL | ❌ FAIL |
| `test_different_owning_containers` | Assert two endpoints have different containers | FAIL | ❌ FAIL |

**Verification output**: `23 verified, 5 errors`

---

## Overall Assessment

- **Correctness**: ✅ All 10 correctness tests pass. The `endpoint_inv` specification is correct — its postconditions follow from the `wf()` precondition.
- **Completeness**: ✅ All 25 completeness tests fail as expected. The specification rejects:
  - Calls without proper preconditions (Round 1)
  - Overly strong claims about queue state (Round 2)
  - Negated postconditions (Round 3)
  - Wrong concrete values (Round 4)
  - Cross-domain confusion and out-of-range accesses (Round 5)
- **Spec Gaps Found**: None. The specification is both correct and appropriately tight.
