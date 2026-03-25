# Test Execution Summary: `syscall_send_pages`

## Overview

Target: `kernel__syscall_send_pages__impl0__syscall_send_pages.rs`

Generated **36 adversarial proof tests** across 3 files. All tests are designed to **FAIL verification**, probing the semantic boundary of the `syscall_send_pages` specification.

## Results

| Test File | Tests | Failed (Expected) | Passed (Unexpected) |
|-----------|-------|--------------------|---------------------|
| `boundary_tests.rs` | 12 | 12 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 12 | 12 ✅ | 0 |
| `logical_tests.rs` | 12 | 12 ✅ | 0 |
| **Total** | **36** | **36** | **0** |

## Boundary Tests (12/12 failed ✅)

All precondition violations were correctly rejected:

| # | Target | Violation | Result |
|---|--------|-----------|--------|
| 1 | `syscall_send_pages` | `endpoint_payload == MAX (128)` | FAIL ✅ |
| 2 | `syscall_send_pages` | thread not in domain | FAIL ✅ |
| 3 | `syscall_send_pages` | thread state BLOCKED (not RUNNING) | FAIL ✅ |
| 4 | `syscall_send_pages` | thread state SCHEDULED (not RUNNING) | FAIL ✅ |
| 5 | `page_ptr2page_index` | non-aligned ptr (ptr=1) | FAIL ✅ |
| 6 | `page_index2page_ptr` | index == NUM_PAGES (out of range) | FAIL ✅ |
| 7 | `syscall_send_pages` | endpoint_payload == usize::MAX | FAIL ✅ |
| 8 | VA range wf | len causes overflow | FAIL ✅ |
| 9 | `va_4k_valid` | VA=0 (kernel space) invalid | FAIL ✅ |
| 10 | `block_running_thread` | queue at max capacity | FAIL ✅ |
| 11 | `schedule_blocked_thread` | scheduler at max capacity | FAIL ✅ |
| 12 | `range_create_and_share_mapping` | same src/target proc | FAIL ✅ |

## Behavioral Mutation Tests (12/12 failed ✅)

All incorrect output assertions were correctly rejected:

| # | Spec Path | Mutation | Result |
|---|-----------|----------|--------|
| 1 | no_receiver block | queue unchanged (should be pushed) | FAIL ✅ |
| 2 | success path | endpoint queue unchanged (should skip(1)) | FAIL ✅ |
| 3 | sender_queue_full | thread domain differs (should be same) | FAIL ✅ |
| 4 | blocking path | state becomes SCHEDULED (should be BLOCKED) | FAIL ✅ |
| 5 | blocking path | ipc_payload VA start wrong | FAIL ✅ |
| 6 | success path | receiver VA not in new space | FAIL ✅ |
| 7 | receiver_queue_empty | queue_state stays RECEIVE (should be SEND) | FAIL ✅ |
| 8 | NoSwitchNew | wrong error code (Else vs Error) | FAIL ✅ |
| 9 | success path | proc domain grows (should be same) | FAIL ✅ |
| 10 | success path | sender descriptors length changes | FAIL ✅ |
| 11 | quota subtraction | quota unchanged with ret=3 | FAIL ✅ |
| 12 | success path | receiver old mapping addr changes | FAIL ✅ |

## Logical Tests (12/12 failed ✅)

All unintended properties were correctly rejected:

| # | Property Tested | Why Not Entailed | Result |
|---|----------------|------------------|--------|
| 1 | return always Error | success path returns Else | FAIL ✅ |
| 2 | sender always BLOCKED | sender not blocked in success path | FAIL ✅ |
| 3 | ret always 3×len | spec allows ret ≤ 3×len | FAIL ✅ |
| 4 | determinism (ret1==ret2) | different rets possible with same bound | FAIL ✅ |
| 5 | page ptr/index roundtrip for unaligned | only valid for aligned ptrs | FAIL ✅ |
| 6 | queue_state always RECEIVE | depends on old state | FAIL ✅ |
| 7 | shared page addr always non-zero | spec doesn't constrain addr values | FAIL ✅ |
| 8 | va_range.len always > 0 | wf() allows len=0 | FAIL ✅ |
| 9 | all container quotas preserved | receiver container quota changes | FAIL ✅ |
| 10 | all page mappings identical | shared pages get new mappings | FAIL ✅ |
| 11 | receiver becomes RUNNING | spec doesn't guarantee new thread state | FAIL ✅ |
| 12 | switch decision is Switch | NoSwitchNew forces NoSwitch | FAIL ✅ |

## Conclusion

The specification for `syscall_send_pages` correctly rejects all 36 adversarial queries across boundary, behavioral, and logical dimensions. No specification weakness was detected — the spec appropriately constrains its semantic boundary for the properties tested.
