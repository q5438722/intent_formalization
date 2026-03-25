# Test Summary: `syscall_send_empty_no_block`

## Target Specification

**Function**: `Kernel::syscall_send_empty_no_block(&mut self, sender_thread_ptr, blocking_endpoint_index) -> SyscallReturnStruct`

**Preconditions (`requires`)**:
1. `old(self).wf()` — kernel is well-formed
2. `old(self).thread_dom().contains(sender_thread_ptr)` — sender thread exists
3. `0 <= blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS` — valid endpoint index
4. `old(self).get_thread(sender_thread_ptr).state == ThreadState::RUNNING` — thread is running

**Postconditions (`ensures`)**:
1. `self.wf()` — kernel remains well-formed

**Key observation**: The postcondition is extremely weak — it only guarantees that `self.wf()` is preserved. There are no postconditions about the return value, state changes, or preservation of unmodified state.

---

## Execution Results

### Boundary Tests (7/7 FAILED ✅)

| Test | Target | Result |
|------|--------|--------|
| 1. `test_boundary_endpoint_index_at_max` | Off-by-one: index == MAX_NUM_ENDPOINT_DESCRIPTORS | FAILED ✅ |
| 2. `test_boundary_thread_not_in_domain` | Thread not in domain → properties inaccessible | FAILED ✅ |
| 3. `test_boundary_endpoint_index_usize_max` | Extreme value: usize::MAX as index | FAILED ✅ |
| 4. `test_boundary_endpoint_not_in_domain` | Endpoint not in domain → no queue.wf() | FAILED ✅ |
| 5. `test_boundary_schedule_empty_queue` | Empty queue violates schedule_blocked_thread precond | FAILED ✅ |
| 6. `test_boundary_scheduler_full` | Full scheduler violates schedule_blocked_thread precond | FAILED ✅ |
| 7. `test_boundary_receiver_exist_with_send_state` | SEND state ≠ RECEIVE → receiver_exist is false | FAILED ✅ |

**Verdict**: All boundary preconditions are properly enforced. Invalid inputs are correctly rejected.

---

### Behavioral Mutation Tests (7/7 FAILED ✅)

| Test | Target | Result |
|------|--------|--------|
| 1. `test_mutation_schedule_queue_unchanged` | Queue should change (skip head) after scheduling | FAILED ✅ |
| 2. `test_mutation_schedule_queue_state_changes` | Queue state is preserved, not mutated | FAILED ✅ |
| 3. `test_mutation_other_endpoint_changes` | Other endpoints are preserved | FAILED ✅ |
| 4. `test_mutation_rf_counter_changes` | rf_counter is preserved on target endpoint | FAILED ✅ |
| 5. `test_mutation_thread_dom_shrinks` | thread_dom is preserved | FAILED ✅ |
| 6. `test_mutation_receiver_exist_empty_queue` | Empty queue → receiver_exist is false | FAILED ✅ |
| 7. `test_mutation_noswitchnew_wrong_decision` | NoSwitchNew returns NoSwitch, not Switch | FAILED ✅ |

**Verdict**: All behavioral mutations are correctly rejected. The `schedule_blocked_thread` postconditions and `receiver_exist` spec correctly constrain behavior.

---

### Logical Tests (7/7 FAILED ✅)

| Test | Target | Result |
|------|--------|--------|
| 1. `test_logical_cannot_prove_error_return` | Cannot derive return value from weak ensures | FAILED ✅ |
| 2. `test_logical_no_state_preservation_on_error` | Cannot prove state preservation (weak ensures) | FAILED ✅ |
| 3. `test_logical_receiver_exist_stronger_len` | receiver_exist → queue.len() > 1 (too strong) | FAILED ✅ |
| 4. `test_logical_send_equals_receive` | SEND ≠ RECEIVE (enum distinctness) | FAILED ✅ |
| 5. `test_logical_schedule_adds_procs` | proc_dom preserved, no new procs added | FAILED ✅ |
| 6. `test_logical_all_threads_running` | Not all threads are RUNNING | FAILED ✅ |
| 7. `test_logical_queued_threads_same_container` | Queued threads need not share a container | FAILED ✅ |

**Verdict**: All unintended logical properties are correctly rejected.

---

## Specification Weakness Analysis

While all 21 adversarial tests failed as expected, the tests reveal significant spec weaknesses:

1. **Missing return value postcondition** (Tests L1, L2): The `ensures` clause only states `self.wf()`. There is no postcondition about the return value (`SyscallReturnStruct`). This means the spec cannot be used to reason about whether the function returns `Error` or `Else` in any scenario. A caller cannot prove anything about the return value.

2. **Missing state preservation on error paths** (Test L2): When the function returns `Error` (e.g., endpoint descriptor is `None`), the implementation does not modify `self`. But the spec does not guarantee this — it only says `self.wf()` holds post-call. The spec should ideally express that on error paths, `self` is unchanged.

3. **Missing endpoint/thread state postconditions**: The spec doesn't tell callers which endpoint was modified, which thread was scheduled, or what happened to the receiver. All that information is lost — only `wf()` is preserved.

These weaknesses mean the specification is **sound but incomplete**: it correctly rejects incorrect reasoning, but it does not expose enough information for callers to reason about the effects of the syscall.

## Overall: 21/21 tests FAILED verification ✅

All adversarial tests were correctly rejected by the specification.
