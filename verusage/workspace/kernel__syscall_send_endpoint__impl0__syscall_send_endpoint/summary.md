# Summary: Verus Spec Testing for `syscall_send_endpoint`

## File Under Test

`kernel__syscall_send_endpoint__impl0__syscall_send_endpoint.rs` — Defines a kernel IPC syscall (`syscall_send_endpoint`) for the Atmosphere verified kernel. The function handles sending messages through endpoint-based IPC, with multiple execution paths: blocking the sender when no receiver is available, handling full queues, changing queue states, and passing endpoint references between threads. The spec is captured in `syscall_send_endpoint_spec`, an open spec fn with 8+ branches.

## Correctness Results (should all PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_ipc_endpoint_returns_some` | IPCPayLoad::Endpoint returns Some(42) | PASS | PASS |
| 2 | `test_ipc_empty_returns_none_endpoint` | IPCPayLoad::Empty returns None | PASS | PASS |
| 3 | `test_ipc_message_returns_none_endpoint` | IPCPayLoad::Message returns None for endpoint | PASS | PASS |
| 4 | `test_ipc_pci_returns_none_endpoint` | IPCPayLoad::Pci returns None for endpoint | PASS | PASS |
| 5 | `test_ipc_pagefault_returns_none_endpoint` | IPCPayLoad::PageFault returns None for endpoint | PASS | PASS |
| 6 | `test_ipc_pages_returns_some_va_range` | IPCPayLoad::Pages returns Some for va_range | PASS | PASS |
| 7 | `test_ipc_endpoint_returns_none_va_range` | IPCPayLoad::Endpoint returns None for va_range | PASS | PASS |
| 8 | `test_ipc_empty_returns_none_va_range` | IPCPayLoad::Empty returns None for va_range | PASS | PASS |
| 9 | `test_ipc_endpoint_boundary_0` | Endpoint index 0 returns Some(0) | PASS | PASS |
| 10 | `test_ipc_endpoint_boundary_127` | Endpoint index 127 returns Some(127) | PASS | PASS |
| 11 | `test_no_receiver_implies_send_state` | no_receiver implies SEND state | PASS | PASS |
| 12 | `test_sender_queue_full_implies_send_state` | sender_queue_full implies SEND state | PASS | PASS |
| 13 | `test_receiver_queue_empty_implies_receive_state` | receiver_queue_empty implies RECEIVE state | PASS | PASS |
| 14 | `test_receiver_exist_implies_receive_state` | receiver_exist implies RECEIVE state | PASS | PASS |
| 15 | `test_no_receiver_exclusive_with_receiver_exist` | no_receiver implies NOT receiver_exist | PASS | PASS |
| 16 | `test_sender_queue_full_exclusive_with_no_receiver` | sender_queue_full implies NOT no_receiver | PASS | PASS |
| 17 | `test_receiver_exist_implies_queue_nonempty` | receiver_exist implies queue.len != 0 | PASS | PASS |
| 18 | `test_receiver_queue_empty_implies_queue_len_zero` | receiver_queue_empty implies queue.len == 0 | PASS | PASS |
| 19 | `test_spec_no_endpoint_no_change` | Spec: no endpoint implies old =~= new | PASS | PASS |
| 20 | `test_spec_queue_full_no_change` | Spec: queue full implies old =~= new | PASS | PASS |
| 21 | `test_thread_inv_container_membership` | thread_inv: thread's container in dom | PASS | PASS |
| 22 | `test_endpoint_inv_queue_wf` | endpoint_inv: queue.wf() | PASS | PASS |
| 23 | `test_endpoint_inv_owning_container` | endpoint_inv: container in dom | PASS | PASS |
| 24 | `test_thread_inv_proc_membership` | thread_inv: thread's proc in dom | PASS | PASS |
| 25 | `test_thread_inv_endpoint_descriptors_wf` | thread_inv: endpoint_descriptors.wf() | PASS | PASS |

**Verus output**: `67 verified, 0 errors`

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_thread_inv_no_wf` | Call thread_inv without wf() | FAIL | FAIL |
| 2 | `test_fail_endpoint_inv_no_wf` | Call endpoint_inv without wf() | FAIL | FAIL |
| 3 | `test_fail_spec_no_thread_dom` | Assert spec holds without thread_dom membership | FAIL | FAIL |
| 4 | `test_fail_wrong_thread_state` | Assert RUNNING when state is BLOCKED | FAIL | FAIL |
| 5 | `test_fail_assume_endpoint_exists` | Assert endpoint exists without evidence | FAIL | FAIL |

**Verus output**: `42 verified, 5 errors`

### Round 2: Overly Strong Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_no_receiver_queue_empty` | Assert queue.len==0 when only len<MAX guaranteed | FAIL | FAIL |
| 2 | `test_fail_thread_inv_too_strong_state` | Assert all threads RUNNING (not guaranteed) | FAIL | FAIL |
| 3 | `test_fail_thread_inv_blocking_endpoint` | Assert all threads have blocking_endpoint (only BLOCKED) | FAIL | FAIL |
| 4 | `test_fail_queue_full_strict` | Assert len > MAX when only len >= MAX | FAIL | FAIL |
| 5 | `test_fail_endpoint_inv_queue_empty` | Assert queue.len==0 from endpoint_inv (only wf) | FAIL | FAIL |

**Verus output**: `42 verified, 5 errors`

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_endpoint_gives_none` | IPCPayLoad::Endpoint gives None (should be Some) | FAIL | FAIL |
| 2 | `test_fail_empty_gives_some` | IPCPayLoad::Empty gives Some (should be None) | FAIL | FAIL |
| 3 | `test_fail_no_receiver_is_receive` | no_receiver implies RECEIVE (should be SEND) | FAIL | FAIL |
| 4 | `test_fail_receiver_exist_is_send` | receiver_exist implies SEND (should be RECEIVE) | FAIL | FAIL |
| 5 | `test_fail_no_receiver_and_queue_empty` | no_receiver + receiver_queue_empty (SEND vs RECEIVE) | FAIL | FAIL |

**Verus output**: `42 verified, 5 errors`

### Round 4: Wrong Specific Values

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_wrong_endpoint_value` | Endpoint(42) returns Some(43) (should be 42) | FAIL | FAIL |
| 2 | `test_fail_wrong_endpoint_value_0_vs_1` | Endpoint(0) returns Some(1) (should be 0) | FAIL | FAIL |
| 3 | `test_fail_wrong_endpoint_value_127_vs_126` | Endpoint(127) returns Some(126) (should be 127) | FAIL | FAIL |
| 4 | `test_fail_endpoint_returns_wrong_va_range` | Endpoint returns Some va_range (should be None) | FAIL | FAIL |
| 5 | `test_fail_wrong_thread_state_value` | SCHEDULED == RUNNING (different states) | FAIL | FAIL |

**Verus output**: `42 verified, 5 errors`

### Round 5: Cross-function Misuse & Edge Cases

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_no_receiver_and_receiver_exist` | Both no_receiver and receiver_exist (SEND!=RECEIVE) | FAIL | FAIL |
| 2 | `test_fail_queue_full_small_len` | sender_queue_full implies len<10 (contradicts len>=128) | FAIL | FAIL |
| 3 | `test_fail_receiver_empty_but_nonempty` | receiver_queue_empty implies len>0 (contradicts len==0) | FAIL | FAIL |
| 4 | `test_fail_queue_full_changes_kernel` | Queue full: thread state changed (spec says no change) | FAIL | FAIL |
| 5 | `test_fail_inv_gives_all_threads_running` | thread_inv+endpoint_inv implies all RUNNING (not guaranteed) | FAIL | FAIL |

**Verus output**: `42 verified, 5 errors`

## Overall Assessment

### Correctness: PASS (25/25 tests verified)
The specs are **correct** — all valid usages produce valid results. Key verified properties:
- `IPCPayLoad` spec functions correctly discriminate variants and return accurate indices
- Helper predicates (`no_receiver`, `sender_queue_full`, `receiver_queue_empty`, `receiver_exist`) correctly capture endpoint state conditions
- `syscall_send_endpoint_spec` correctly describes no-change behavior for error paths (no endpoint, queue full)
- `thread_inv` and `endpoint_inv` provide the domain membership and well-formedness guarantees they claim

### Completeness: PASS (25/25 tests rejected)
The specs are **tight enough** — all invalid claims are rejected:
- Precondition violations (missing wf(), missing domain membership) are caught
- Overly strong assertions (e.g., len==0 vs len<MAX, len>MAX vs len>=MAX) are rejected
- Negated postconditions (wrong state, wrong variant) are rejected
- Wrong concrete values (off-by-one endpoint indices) are caught
- Cross-function misuse and impossible state combinations (SEND+RECEIVE, full+small) are rejected

### Spec Gaps Found: None
No completeness tests passed unexpectedly. The specifications appear well-formed and appropriately constrained for the tested properties.
