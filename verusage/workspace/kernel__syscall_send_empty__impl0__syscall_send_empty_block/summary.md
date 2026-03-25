# Adversarial Test Results: `syscall_send_empty_block`

## Summary

All **36 adversarial tests** (12 per category) **FAIL verification** as expected, confirming the specification correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (12/12 FAIL) ✅

| # | Target | Failure Mode |
|---|--------|-------------|
| 1 | `syscall_send_empty_block` | `endpoint_index == MAX_NUM_ENDPOINT_DESCRIPTORS` (128) violates upper bound |
| 2 | `syscall_send_empty_block` | Thread not in `thread_dom` violates containment precondition |
| 3 | `syscall_send_empty_block` | Thread BLOCKED ≠ RUNNING violates state precondition |
| 4 | `syscall_send_empty_block` | Thread SCHEDULED ≠ RUNNING violates state precondition |
| 5 | `schedule_blocked_thread` | Queue length 0 violates `queue.len() > 0` |
| 6 | `schedule_blocked_thread` | Scheduler at capacity (10) violates `< MAX_CONTAINER_SCHEDULER_LEN` |
| 7 | `block_running_thread_and_set_trap_frame` | Queue at 128 violates `< MAX_NUM_THREADS_PER_ENDPOINT` |
| 8 | `block_running_thread_and_set_trap_frame` | None endpoint descriptor violates `is_Some()` |
| 9 | `page_ptr2page_index` | Non-aligned ptr (0x1001) violates `ptr % 0x1000 == 0` |
| 10 | `page_index2page_ptr` | `i == NUM_PAGES` violates `i < NUM_PAGES` |
| 11 | `syscall_send_empty_block` | `usize::MAX` far exceeds endpoint descriptor range |
| 12 | `get_endpoint` | Endpoint not in domain violates containment precondition |

## Behavioral Mutation Tests (12/12 FAIL) ✅

| # | Target Postcondition | Mutation |
|---|---------------------|----------|
| 1 | Thread becomes BLOCKED after block | Claim stays RUNNING |
| 2 | Queue gets thread pushed | Claim queue unchanged |
| 3 | Queue skips first after schedule | Claim queue unchanged |
| 4 | Endpoint domain unchanged after schedule | Claim new endpoint appears |
| 5 | NoSwitchNew → NoSwitch decision | Claim decision is Switch |
| 6 | NoNextThreadNew → pcid is None | Claim pcid is Some |
| 7 | Queue state preserved by block | Claim SEND → RECEIVE |
| 8 | Thread domain unchanged after schedule | Claim thread removed |
| 9 | Queue state set to target after change | Claim opposite state |
| 10 | Endpoint descriptors preserved | Claim length becomes 0 |
| 11 | Proc domain unchanged after schedule | Claim new proc added |
| 12 | NoSwitchNew → cr3 is None | Claim cr3 is Some |

## Logical Tests (12/12 FAIL) ✅

| # | Unintended Property Tested | Why Not Entailed |
|---|---------------------------|------------------|
| 1 | Thread domain preserved across syscall | Postcondition only guarantees `self.wf()` |
| 2 | Return always Error | Return value unspecified in postcondition |
| 3 | Sender always BLOCKED after call | Thread state change not in postcondition |
| 4 | Determinism (same decisions) | No determinism guarantee in spec |
| 5 | Container domain preserved | Not in postcondition |
| 6 | Endpoint state always preserved | Not in postcondition |
| 7 | Dequeued thread becomes RUNNING | Schedule spec only covers queue manipulation |
| 8 | Return always NoSwitch | NoThread is also possible |
| 9 | Roundtrip for out-of-range index | Index ≥ NUM_PAGES violates domain |
| 10 | Page closure preserved | Not in postcondition |
| 11 | Endpoint domain always non-empty | Not guaranteed by wf() |
| 12 | Sender ptr < MAX_THREADS_PER_ENDPOINT | Thread ptrs are arbitrary `usize` |

## Key Findings

**Spec weakness identified**: `syscall_send_empty_block` postcondition (`ensures self.wf()`) is extremely weak. It only guarantees well-formedness is maintained but says nothing about:
- Thread/endpoint/container domain preservation
- Return value properties
- Sender thread state transitions
- Endpoint queue modifications

The helper functions (`schedule_blocked_thread`, `block_running_thread_*`) have much richer postconditions, but these properties are not propagated to the caller's postcondition, making the main syscall specification incomplete from a caller's perspective.
