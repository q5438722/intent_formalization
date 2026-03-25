# Test Execution Summary: `schedule_idle_cpu`

## Target
`kernel__schedule_idle_cpu__impl0__schedule_idle_cpu.rs`

## Key Specification Under Test

```rust
pub fn schedule_idle_cpu(&mut self, cpu_id: CpuId, pt_regs: &mut Registers) -> (ret: SyscallReturnStruct)
    requires old(self).wf(), 0 <= cpu_id < NUM_CPUS,
    ensures  self.wf(),
```

The postcondition **only guarantees `self.wf()`** — it says nothing about the return value (`ret`), scheduler changes, or thread state transitions.

---

## Results Overview

| Test File | Tests | All Failed? | Status |
|---|---|---|---|
| `boundary_tests.rs` | 12 | ✅ Yes (12/12 errors) | PASS |
| `behavioral_mutation_tests.rs` | 12 | ✅ Yes (12/12 errors) | PASS |
| `logical_tests.rs` | 12 | ✅ Yes (12/12 errors) | PASS |

**Total: 36 tests, all 36 correctly fail verification.**

---

## Boundary Tests (12/12 FAIL ✅)

All precondition violations are correctly rejected:

| # | Test | Violated Precondition |
|---|---|---|
| 1 | `cpu_id = NUM_CPUS` | `cpu_id < NUM_CPUS` |
| 2 | `cpu_id = usize::MAX` | `cpu_id < NUM_CPUS` |
| 3 | `cpu_id = 33` | `cpu_id < NUM_CPUS` |
| 4 | `pcid = PCID_MAX` | `pcid < PCID_MAX` |
| 5 | `pcid = usize::MAX` | `pcid < PCID_MAX` |
| 6 | `ptr = 1` (unaligned) | `ptr % 0x1000 == 0` |
| 7 | `ptr = 0xFFF` | `ptr % 0x1000 == 0` |
| 8 | `index = NUM_PAGES` | `index < NUM_PAGES` |
| 9 | `active = false` | `active == true` |
| 10 | `current_thread = Some(t)` | `current_thread is None` |
| 11 | `scheduler_len = 0` | `scheduler.len() != 0` |
| 12 | `i = NUM_CPUS` for Array | `i < N` |

## Behavioral Mutation Tests (12/12 FAIL ✅)

All incorrect output mutations are correctly rejected:

| # | Test | Mutated Property |
|---|---|---|
| 1 | NoSwitch → Switch | `switch_decision` wrong variant |
| 2 | NoThread → NoSwitch | `switch_decision` wrong variant |
| 3 | SwitchNew pcid is None | `pcid` should be Some |
| 4 | SwitchNew cr3 is None | `cr3` should be Some |
| 5 | NoSwitch pcid is Some | `pcid` should be None |
| 6 | NoSwitch cr3 is Some | `cr3` should be None |
| 7 | NoThread has pcid+cr3 | Both should be None |
| 8 | Switch → NoThread | `switch_decision` wrong variant |
| 9 | `perm(0).present == true` | Should be false |
| 10 | `perm(0).write == true` | Should be false |
| 11 | `usize2pa(0)` not MEM_valid | Should be valid |
| 12 | Popped thread not RUNNING | Should be RUNNING |

## Logical Tests (12/12 FAIL ✅)

All unintended reasoning attempts are correctly rejected:

| # | Test | Unwarranted Property |
|---|---|---|
| 1 | Always switches | Spec doesn't constrain return |
| 2 | Deterministic decision | No determinism guarantee |
| 3 | `usize2pa` injective | Masking loses bits |
| 4 | Ptr↔index roundtrip for all | Only valid for aligned ptrs |
| 5 | `va_4k_valid` ⇒ `page_ptr_valid` | Different address domains |
| 6 | NoSwitch == NoThread | Distinct enum variants |
| 7 | Scheduler always shrinks by 1 | Only on Switch path |
| 8 | Popped thread was already RUNNING | Was SCHEDULED before pop |
| 9 | cr3 always non-zero | No such guarantee |
| 10 | pcid < NUM_CPUS | pcid ∈ [0, PCID_MAX), not NUM_CPUS |
| 11 | Scheduler empty after pop | Could have >1 entries |
| 12 | thread_dom ⊆ proc_dom | Different pointer spaces |

---

## Specification Weakness Analysis

The `schedule_idle_cpu` specification has a notably **weak postcondition**: it only guarantees `self.wf()`. This means:

1. **No return value contract**: Callers cannot determine from the spec whether scheduling succeeded, failed, or what switch decision was made.
2. **No scheduler change guarantee**: The spec doesn't expose that the scheduler is popped on success.
3. **No thread state transition guarantee**: The spec doesn't export that the popped thread transitions to RUNNING.
4. **No error code semantics**: The spec doesn't distinguish between the three error paths (inactive CPU, busy CPU, empty scheduler).

These are by design (the internal implementation is correct), but callers must rely solely on `self.wf()` — making the function essentially opaque from a caller's perspective regarding its return value.
