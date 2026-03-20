# Test Summary: `syscall_send_empty_block`

## File Under Test
`kernel__syscall_send_empty__impl0__syscall_send_empty_block.rs`

Defines `Kernel::syscall_send_empty_block`, a syscall handler for sending an empty IPC message with blocking semantics. The function handles 5 code paths:
1. Endpoint descriptor is `None` → return error (NoSwitch)
2. Endpoint in SEND state with queue room → block sender thread
3. Endpoint in SEND state, queue full → return error (NoSwitch)
4. Endpoint in RECEIVE state, empty queue → block sender, change state to SEND
5. Receiver exists → schedule receiver thread and return success (NoSwitch/Else)

**Requires**: `self.wf()`, thread in domain, valid endpoint index, thread RUNNING  
**Ensures**: `self.wf()` (kernel remains well-formed)

---

## Correctness Results (should all PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_preserves_wf` | Basic wf preservation with arbitrary valid inputs | PASS | PASS |
| 2 | `test_endpoint_index_min` | Boundary: endpoint index = 0 | PASS | PASS |
| 3 | `test_endpoint_index_max` | Boundary: endpoint index = MAX-1 | PASS | PASS |
| 4 | `test_none_endpoint_path` | Path: endpoint descriptor is None | PASS | PASS |
| 5 | `test_send_with_room_path` | Path: SEND state with queue room | PASS | PASS |
| 6 | `test_send_full_path` | Path: SEND state, queue full | PASS | PASS |
| 7 | `test_receive_empty_queue_path` | Path: RECEIVE state, empty queue | PASS | PASS |
| 8 | `test_receiver_exists_path` | Path: receiver exists | PASS | PASS |
| 9 | `test_receiver_exist_spec` | Spec fn `receiver_exist` is well-defined | PASS | PASS |
| 10 | `test_get_endpoint_ptr_consistency` | Spec fn `get_endpoint_ptr_by_endpoint_idx` consistent | PASS | PASS |

**Verification**: 53 verified, 0 errors

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_no_wf` | Call without `wf()` precondition | FAIL | FAIL |
| 2 | `test_thread_not_in_domain` | Thread not in `thread_dom()` | FAIL | FAIL |
| 3 | `test_invalid_endpoint_index_too_large` | Endpoint index = MAX (out of range) | FAIL | FAIL |
| 4 | `test_thread_not_running_blocked` | Thread in BLOCKED state | FAIL | FAIL |
| 5 | `test_thread_not_running_scheduled` | Thread in SCHEDULED state | FAIL | FAIL |

**Verification**: 43 verified, 5 errors

### Round 2: Overly Strong Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_always_no_switch` | Assert switch_decision always NoSwitch | FAIL | FAIL |
| 2 | `test_always_error` | Assert error_code always Error | FAIL | FAIL |
| 3 | `test_thread_dom_unchanged` | Assert thread_dom unchanged (not in spec) | FAIL | FAIL |
| 4 | `test_page_closure_unchanged` | Assert page_closure unchanged (not in spec) | FAIL | FAIL |
| 5 | `test_pcid_always_none` | Assert pcid always None | FAIL | FAIL |

**Verification**: 43 verified, 5 errors

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_not_wf_after_call` | Assert `!kernel.wf()` | FAIL | FAIL |
| 2 | `test_mem_man_not_wf` | Assert `!kernel.mem_man.wf()` | FAIL | FAIL |
| 3 | `test_proc_man_not_wf` | Assert `!kernel.proc_man.wf()` | FAIL | FAIL |
| 4 | `test_page_alloc_not_wf` | Assert `!kernel.page_alloc.wf()` | FAIL | FAIL |
| 5 | `test_mapping_not_wf` | Assert `!kernel.mapping_wf()` | FAIL | FAIL |

**Verification**: 43 verified, 5 errors

### Round 4: Wrong Specific Values

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_always_else` | Assert error_code always Else | FAIL | FAIL |
| 2 | `test_none_endpoint_wrong_switch` | Assert NoThread when endpoint is None | FAIL | FAIL |
| 3 | `test_cr3_always_some` | Assert cr3 always Some | FAIL | FAIL |
| 4 | `test_receiver_wrong_error_code` | Assert Error when success path taken | FAIL | FAIL |
| 5 | `test_always_switch` | Assert switch_decision always Switch | FAIL | FAIL |

**Verification**: 43 verified, 5 errors

### Round 5: Cross-Function Misuse & Edge Cases

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_double_call_no_recheck` | Call twice without re-establishing RUNNING | FAIL | FAIL |
| 2 | `test_sender_still_running` | Assert sender thread still RUNNING after call | FAIL | FAIL |
| 3 | `test_endpoint_dom_grows` | Assert new endpoint appears in domain | FAIL | FAIL |
| 4 | `test_container_dom_changes` | Assert container_dom changes | FAIL | FAIL |
| 5 | `test_memory_not_wf` | Assert `!kernel.memory_wf()` | FAIL | FAIL |

**Verification**: 43 verified, 5 errors

---

## Overall Assessment

### Correctness: PASS
All 10 correctness tests pass. The spec correctly guarantees `self.wf()` across all code paths.

### Completeness: PASS
All 25 completeness tests fail as expected. The spec rejects:
- All precondition violations (5/5 rejected)
- All overly strong postcondition assertions (5/5 rejected)
- All negated postconditions (5/5 rejected)
- All wrong specific value claims (5/5 rejected)
- All cross-function misuse patterns (5/5 rejected)

### Spec Gaps
The spec is **minimal** — it only ensures `self.wf()` without exposing any information about:
- The return value (error code, switch decision, pcid, cr3)
- Which code path was taken
- Whether thread state changed
- Whether domain sets are preserved

This is a design choice: the function's contract only guarantees kernel integrity, delegating all functional behavior to internal implementation details. This is correct but not informative — callers cannot reason about which result they will get. However, for a kernel syscall where the caller is typically not other verified code, this may be intentional.
