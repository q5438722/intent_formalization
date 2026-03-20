# Summary: Verus Spec Testing for `syscall_send_pages`

## File Under Test

`kernel__syscall_send_pages__impl0__syscall_send_pages.rs` — defines a kernel IPC syscall (`syscall_send_pages`) for sharing memory pages between processes. The spec `syscall_send_pages_spec` is a ~200-line open spec function with ~13 conditional branches covering: endpoint existence, queue states, scheduler capacity, VA range validity, quota checks, and the success case where pages are shared and mappings updated.

## Correctness Results (should all PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_endpoint_not_exists` | When endpoint doesn't exist, `old =~= new` | PASS | PASS |
| 2 | `test_sender_queue_full` | When sender queue full, `old =~= new` | PASS | PASS |
| 3 | `test_no_receiver_domains` | No receiver: thread/proc/container/endpoint domains preserved | PASS | PASS |
| 4 | `test_receiver_queue_empty_domains` | Receiver queue empty: all domains preserved | PASS | PASS |
| 5 | `test_success_domains` | Success case: all domains preserved | PASS | PASS |
| 6 | `test_no_receiver_sender_descriptors` | No receiver: thread_dom preserved | PASS | PASS |
| 7 | `test_receiver_queue_empty_sender_descriptors` | Receiver queue empty: endpoint_dom preserved | PASS | PASS |
| 8 | `test_no_switch_struct` | SyscallReturnStruct field consistency | PASS | PASS |
| 9 | `test_ipc_payload_pages` | IPCPayLoad::Pages returns Some(va_range) | PASS | PASS |
| 10 | `test_ipc_payload_empty` | IPCPayLoad::Empty returns None | PASS | PASS |
| 11 | `test_no_receiver_and_full_exclusive` | no_receiver and sender_queue_full are mutually exclusive | PASS | PASS |
| 12 | `test_receiver_exist_and_empty_exclusive` | receiver_exist and receiver_queue_empty are mutually exclusive | PASS | PASS |
| 13 | `test_receiver_exist_and_no_receiver_exclusive` | receiver_exist and no_receiver are mutually exclusive | PASS | PASS |
| 14 | `test_address_space_free_def` | Empty VA range is always free | PASS | PASS |

**Result: 58 verified, 0 errors**

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_no_spec_unchanged` | Claim `old =~= new` without spec assumption | FAIL | FAIL |
| 2 | `test_no_spec_domains` | Claim domains match without spec | FAIL | FAIL |
| 3 | `test_wrong_derivation` | Derive no_receiver when endpoint doesn't exist | FAIL | FAIL |
| 4 | `test_receiver_exist_without_conditions` | Claim receiver_exist without queue info | FAIL | FAIL |
| 5 | `test_arbitrary_va_range` | Claim `old =~= new` when no_receiver (state changes) | FAIL | FAIL |

**Result: 43 verified, 5 errors**

### Round 2: Overly Strong Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_too_strong_return_code` | Assert specific ret.error_code when spec doesn't constrain it | FAIL | FAIL |
| 2 | `test_no_receiver_queue_empty` | Assert queue empties (spec says it grows) | FAIL | FAIL |
| 3 | `test_overly_strong_range_free` | Assert address_space_range_free without checking | FAIL | FAIL |
| 4 | `test_sender_queue_full_specific_ret` | Assert Else return code in error case | FAIL | FAIL |
| 5 | `test_no_receiver_unchanged` | Assert `old =~= new` in no_receiver case (wrong) | FAIL | FAIL |

**Result: 43 verified, 5 errors**

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_negate_thread_dom_preserved` | Negate thread_dom preservation in no_receiver | FAIL | FAIL |
| 2 | `test_negate_state_unchanged` | Negate `old =~= new` in endpoint-not-exists | FAIL | FAIL |
| 3 | `test_negate_proc_dom` | Negate proc_dom preservation in no_receiver | FAIL | FAIL |
| 4 | `test_negate_sender_queue_full_unchanged` | Negate `old =~= new` in sender_queue_full | FAIL | FAIL |
| 5 | `test_negate_container_dom_success` | Negate container_dom preservation in success | FAIL | FAIL |

**Result: 43 verified, 5 errors**

### Round 4: Wrong Specific Values

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_spec_unsatisfiable` | Assert `false` after assuming spec (spec is satisfiable) | FAIL | FAIL |
| 2 | `test_empty_payload_is_some` | IPCPayLoad::Empty gives Some (wrong) | FAIL | FAIL |
| 3 | `test_overly_strong_shareable` | Assert shareable without address space info | FAIL | FAIL |
| 4 | `test_sender_full_domains_change` | Assert domains change in sender_queue_full (state unchanged) | FAIL | FAIL |
| 5 | `test_full_implies_no_receiver` | Assert sender_queue_full implies no_receiver (mutually exclusive) | FAIL | FAIL |

**Result: 43 verified, 5 errors**

### Round 5: Cross-function Misuse & Edge Cases

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_endpoint_not_exists_implies_receiver` | Derive endpoint_exists from its negation | FAIL | FAIL |
| 2 | `test_no_receiver_implies_state_unchanged` | Claim `old =~= new` when no_receiver modifies state | FAIL | FAIL |
| 3 | `test_sender_queue_full_state_changed` | Claim domains differ when state is unchanged | FAIL | FAIL |
| 4 | `test_no_receiver_specific_return` | Claim specific switch_decision without guarantee | FAIL | FAIL |
| 5 | `test_success_state_unchanged` | Claim `old =~= new` in success case (state changes) | FAIL | FAIL |

**Result: 43 verified, 5 errors**

## Overall Assessment

### Correctness
The specs are **correct**. All 15 correctness tests pass, confirming:
- Error branches correctly preserve kernel state (`old =~= new`)
- Blocking branches correctly preserve domain sets
- Helper spec functions (`no_receiver`, `sender_queue_full`, `receiver_exist`, `receiver_queue_empty`) are mutually exclusive as expected
- `IPCPayLoad` spec functions correctly discriminate variants
- `address_space_range_free` is trivially true for empty ranges

### Completeness
The specs are **reasonably complete**. All 25 completeness tests fail as expected, confirming:
- The spec cannot be satisfied vacuously (`assert(false)` fails)
- The spec rejects incorrect postconditions (wrong return codes, wrong queue operations)
- The spec rejects negated postconditions (contradicting proven properties)
- Helper predicates reject unsupported derivations (no_receiver != sender_queue_full)
- Cross-branch confusion is rejected (no_receiver != state-unchanged, success != unchanged)

### Spec Gaps Noted
- The spec `syscall_send_pages_spec` does not constrain the return value `ret` in most branches (only relates `old` and `new` kernel states). This is by design — the return struct is constrained by the function's ensures clause but the spec function focuses on kernel state transitions.
- Some properties (e.g., sender's IPC payload being set, queue push/skip) are deeply nested inside forall-implies chains in the spec, making them difficult to extract directly in tests. This is a structural property of the spec, not a correctness issue.
