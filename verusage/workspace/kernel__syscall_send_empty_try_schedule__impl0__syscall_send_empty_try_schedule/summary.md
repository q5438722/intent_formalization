# Test Execution Summary: `syscall_send_empty_try_schedule`

## Target Function
`Kernel::syscall_send_empty_try_schedule(&mut self, cpu_id, sender_thread_ptr, blocking_endpoint_index, pt_regs) -> SyscallReturnStruct`

**Preconditions**: cpu_id in range, kernel well-formed, CPU active with current thread matching sender, sender RUNNING, endpoint index in range.
**Postconditions**: Only `self.wf()` (kernel remains well-formed). No guarantees on return value or specific state changes.

---

## Results

| Test File | Tests | Failed (expected) | Passed (unexpected) |
|-----------|-------|--------------------|---------------------|
| `boundary_tests.rs` | 15 | 15 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 15 | 15 ✅ | 0 |
| `logical_tests.rs` | 15 | 15 ✅ | 0 |
| **Total** | **45** | **45 ✅** | **0** |

All 45 adversarial tests were **correctly rejected** by Verus, meaning no unintended properties were entailed by the specification.

---

## Boundary Tests (15/15 FAIL ✅)
Tests violated preconditions of `syscall_send_empty_try_schedule` and helper functions:
1. `cpu_id == NUM_CPUS` (out of range)
2. `cpu_id == usize::MAX` (overflow)
3. `blocking_endpoint_index == 128` (out of range)
4. Sender state `BLOCKED` (not RUNNING)
5. Sender state `SCHEDULED` (not RUNNING)
6. CPU `current_thread` is None
7. CPU not active
8. `current_thread != sender_thread_ptr`
9. Sender not in `thread_dom`
10. CPU/sender container mismatch
11. `page_ptr2page_index` with unaligned pointer
12. `page_index2page_ptr` with out-of-range index
13. `schedule_running_thread` with full scheduler
14. `run_blocked_thread` with empty queue
15. `run_blocked_thread` when CPU already has thread

## Behavioral Mutation Tests (15/15 FAIL ✅)
Tests mutated correct postcondition outcomes:
1. `NoSwitchNew` returns wrong error code
2. `NoSwitchNew` pcid is Some (should be None)
3. `NoSwitchNew` switch is Switch (should be NoSwitch)
4. `NoSwitchNew` cr3 is Some (should be None)
5. `is_send` true on RECEIVE endpoint
6. `is_receive` true on SEND endpoint
7. Scheduled thread still RUNNING (should be SCHEDULED)
8. CPU still has thread after schedule (should be None)
9. Other CPU changed (should be preserved)
10. Unblocked thread still BLOCKED (should be RUNNING)
11. Endpoint queue unchanged (should skip 1)
12. CPU has no thread after run_blocked (should have one)
13. Proc domain changed (should be preserved)
14. Scheduler unchanged (should have thread added)
15. `page_ptr2page_index`/`page_index2page_ptr` roundtrip gives wrong result

## Logical Tests (15/15 FAIL ✅)
Tests probed for properties NOT guaranteed by the spec:
1. Return always Error (not guaranteed)
2. Switch always NoSwitch (not guaranteed)
3. Kernel state unchanged (not guaranteed)
4. Sender still RUNNING after syscall (not guaranteed)
5. CPU still has thread after syscall (not guaranteed)
6. Determinism of return values (not guaranteed)
7. Endpoint queues preserved (not guaranteed)
8. Scheduler length strictly bounded (not guaranteed)
9. `is_send` XOR `is_receive` without EndpointState context (not guaranteed)
10. Return pcid always differs from sender (not guaranteed)
11. `page_ptr_valid` implies value < NUM_PAGES (not true)
12. Page closure preserved (not guaranteed by postcondition)
13. Return cr3 always None (not guaranteed)
14. All schedulers have length < 5 (not guaranteed)
15. Switch implies Else error code (not guaranteed)

---

## Observations

The postcondition of `syscall_send_empty_try_schedule` is **very weak**: it only guarantees `self.wf()`. This means:
- No information about the return value is formally guaranteed
- No specific state changes (thread states, scheduler, endpoint queues) are captured in the postcondition
- The function's internal behavior (schedule sender, run blocked receiver, context switch) is NOT reflected in the ensures clause

While the **helper functions** (`schedule_running_thread`, `run_blocked_thread`, `NoSwitchNew`) have detailed postconditions, the **top-level syscall** only propagates `self.wf()`. This is a potential **spec weakness** — callers of `syscall_send_empty_try_schedule` cannot reason about what happened beyond the kernel remaining well-formed.
