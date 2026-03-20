# Summary: Specification Testing for `syscall_send_empty_try_schedule`

## File Under Test
`kernel__syscall_send_empty_try_schedule__impl0__syscall_send_empty_try_schedule.rs`

Defines the Verus kernel syscall `syscall_send_empty_try_schedule` which handles IPC send-empty operations with try-schedule semantics. The function validates endpoint state, finds a receiver, and either returns an error or reschedules threads. Key specs include `thread_inv` and `endpoint_inv` proof lemmas, and the exec function's `requires`/`ensures` clauses (primarily `ensures self.wf()`).

---

## Correctness Results (should all PASS)

**Result: 66 verified, 0 errors ✅**

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_endpoint_state_send_not_receive` | SEND ≠ RECEIVE | PASS | ✅ PASS |
| 2 | `test_endpoint_state_receive_not_send` | RECEIVE ≠ SEND | PASS | ✅ PASS |
| 3 | `test_switch_decisions_distinct` | SwitchDecision variants are distinct | PASS | ✅ PASS |
| 4 | `test_thread_states_distinct` | ThreadState variants are distinct | PASS | ✅ PASS |
| 5 | `test_endpoint_states_distinct` | EndpointState variants are distinct | PASS | ✅ PASS |
| 6 | `test_thread_inv_container_membership` | thread_inv → container in domain | PASS | ✅ PASS |
| 7 | `test_thread_inv_proc_membership` | thread_inv → proc in domain | PASS | ✅ PASS |
| 8 | `test_thread_inv_endpoint_descriptors_wf` | thread_inv → descriptors wf | PASS | ✅ PASS |
| 9 | `test_endpoint_inv_queue_wf` | endpoint_inv → queue wf | PASS | ✅ PASS |
| 10 | `test_endpoint_inv_container_membership` | endpoint_inv → container in domain | PASS | ✅ PASS |
| 11 | `test_endpoint_inv_thread_blocked` | endpoint_inv → queue threads BLOCKED | PASS | ✅ PASS |
| 12 | `test_thread_inv_endpoint_descriptor_membership` | thread_inv → Some descriptors in endpoint_dom | PASS | ✅ PASS |
| 13 | `test_thread_inv_blocked_has_endpoint` | thread_inv → BLOCKED has endpoint_ptr | PASS | ✅ PASS |
| 14 | `test_thread_inv_proc_container_consistency` | thread_inv → proc.container == thread.container | PASS | ✅ PASS |
| 15 | `test_thread_inv_owned_threads_contains` | thread_inv → container owns thread | PASS | ✅ PASS |
| 16 | `test_thread_inv_owned_procs_contains` | thread_inv → container owns proc | PASS | ✅ PASS |
| 17 | `test_kernel_get_thread_delegates` | Kernel.get_thread == proc_man.get_thread | PASS | ✅ PASS |
| 18 | `test_kernel_get_endpoint_delegates` | Kernel.get_endpoint == proc_man.get_endpoint | PASS | ✅ PASS |
| 19 | `test_kernel_thread_dom_delegates` | Kernel.thread_dom == proc_man.thread_dom | PASS | ✅ PASS |
| 20 | `test_receiver_exist_definition` | receiver_exist holds when conditions met | PASS | ✅ PASS |
| 21 | `test_get_endpoint_ptr_by_idx` | Delegation to endpoint_descriptors@ | PASS | ✅ PASS |
| 22 | `test_ipc_payload_empty_no_va_range` | Empty payload → no va_range | PASS | ✅ PASS |
| 23 | `test_no_switch_new_spec_consistency` | NoSwitch properties are consistent | PASS | ✅ PASS |

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

**Result: 43 verified, 6 errors ✅ (all 6 tests failed as expected)**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_thread_inv_no_wf` | thread_inv without wf() | FAIL | ✅ FAIL |
| 2 | `test_endpoint_inv_no_wf` | endpoint_inv without wf() | FAIL | ✅ FAIL |
| 3 | `test_get_endpoint_ptr_bad_index` | Index out of range | FAIL | ✅ FAIL |
| 4 | `test_thread_inv_thread_not_in_dom` | Thread not in domain | FAIL | ✅ FAIL |
| 5 | `test_endpoint_inv_no_queue_check` | Access queue@[0] without len > 0 | FAIL | ✅ FAIL |
| 6 | `test_receiver_exist_no_endpoint_in_dom` | receiver_exist without conditions | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions

**Result: 43 verified, 6 errors ✅ (all 6 tests failed as expected)**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_thread_inv_overly_strong_state` | thread_inv → state == RUNNING | FAIL | ✅ FAIL |
| 2 | `test_thread_inv_overly_strong_cpu` | thread_inv → running_cpu.is_Some() | FAIL | ✅ FAIL |
| 3 | `test_endpoint_inv_overly_strong_queue` | endpoint_inv → queue.len() > 0 | FAIL | ✅ FAIL |
| 4 | `test_endpoint_inv_overly_strong_state` | endpoint_inv → state == RECEIVE | FAIL | ✅ FAIL |
| 5 | `test_thread_inv_overly_strong_all_descriptors` | All descriptors are Some | FAIL | ✅ FAIL |
| 6 | `test_thread_inv_overly_strong_ipc` | ipc_payload == Empty | FAIL | ✅ FAIL |

### Round 3: Negated/Contradicted Postconditions

**Result: 43 verified, 8 errors ✅ (all 8 tests failed as expected)**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_thread_inv_negate_container` | ¬container_dom.contains(container) | FAIL | ✅ FAIL |
| 2 | `test_thread_inv_negate_proc` | ¬proc_dom.contains(proc) | FAIL | ✅ FAIL |
| 3 | `test_endpoint_inv_negate_queue_wf` | ¬queue.wf() | FAIL | ✅ FAIL |
| 4 | `test_endpoint_inv_negate_blocked` | queue thread state ≠ BLOCKED | FAIL | ✅ FAIL |
| 5 | `test_thread_inv_negate_proc_container` | proc.container ≠ thread.container | FAIL | ✅ FAIL |
| 6 | `test_endpoint_state_send_equals_receive` | SEND == RECEIVE | FAIL | ✅ FAIL |
| 7 | `test_thread_state_running_equals_blocked` | RUNNING == BLOCKED | FAIL | ✅ FAIL |
| 8 | `test_endpoint_inv_negate_container` | ¬container_dom.contains(endpoint.container) | FAIL | ✅ FAIL |

### Round 4: Wrong Specific Values

**Result: 43 verified, 8 errors ✅ (all 8 tests failed as expected)**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_num_cpus` | NUM_CPUS == 64 (actual: 32) | FAIL | ✅ FAIL |
| 2 | `test_wrong_max_endpoint_descriptors` | MAX_NUM_ENDPOINT_DESCRIPTORS == 64 (actual: 128) | FAIL | ✅ FAIL |
| 3 | `test_wrong_max_threads_per_endpoint` | MAX_NUM_THREADS_PER_ENDPOINT == 64 (actual: 128) | FAIL | ✅ FAIL |
| 4 | `test_wrong_scheduler_len` | MAX_CONTAINER_SCHEDULER_LEN == 100 (actual: 10) | FAIL | ✅ FAIL |
| 5 | `test_wrong_kernel_mem_end` | KERNEL_MEM_END_L4INDEX == 0 (actual: 1) | FAIL | ✅ FAIL |
| 6 | `test_wrong_num_pages` | NUM_PAGES == 1M (actual: 2M) | FAIL | ✅ FAIL |
| 7 | `test_wrong_pcid_max` | PCID_MAX == 8192 (actual: 4096) | FAIL | ✅ FAIL |
| 8 | `test_wrong_ioid_max` | IOID_MAX == 8192 (actual: 4096) | FAIL | ✅ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases

**Result: 43 verified, 7 errors ✅ (all 7 tests failed as expected)**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_receiver_exist_wrong_state` | receiver_exist with SEND (needs RECEIVE) | FAIL | ✅ FAIL |
| 2 | `test_receiver_exist_empty_queue` | receiver_exist with empty queue | FAIL | ✅ FAIL |
| 3 | `test_running_thread_has_blocking_endpoint` | RUNNING thread has blocking_endpoint | FAIL | ✅ FAIL |
| 4 | `test_different_threads_same_container` | Different threads share container | FAIL | ✅ FAIL |
| 5 | `test_endpoint_threads_same_proc` | Queue threads share proc | FAIL | ✅ FAIL |
| 6 | `test_thread_inv_wrong_count_bound` | All threads SCHEDULED | FAIL | ✅ FAIL |
| 7 | `test_kernel_wf_cpu_has_thread` | wf → CPU has current thread | FAIL | ✅ FAIL |

---

## Overall Assessment

### Correctness: ✅ All specs are correct
All 23 correctness tests (66 verification units including source) passed. The specifications for `thread_inv`, `endpoint_inv`, `receiver_exist`, and the various delegation functions are logically sound.

### Completeness: ✅ All specs are sufficiently tight
All 35 completeness tests across 5 rounds failed as expected. The specs properly reject:
- Missing preconditions (Round 1)
- Overly strong postconditions (Round 2)
- Negated/contradicted postconditions (Round 3)
- Wrong constant values (Round 4)
- Cross-function misuse and invalid assumptions (Round 5)

### Spec Gaps Found: None
No unexpected passes were observed in the completeness tests, indicating the specifications are tight enough to reject invalid claims.
