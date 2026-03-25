# Test Execution Summary: `schedule_idle_cpu`

## Target Function
`Kernel::schedule_idle_cpu(&mut self, cpu_id: CpuId, pt_regs: &mut Registers) -> SyscallReturnStruct`

### Specification
- **Requires**: `old(self).wf()`, `0 <= cpu_id < NUM_CPUS`
- **Ensures**: `self.wf()`

### Key Observation
The postcondition is **very weak** — it only guarantees well-formedness is preserved. It says **nothing** about:
- The return value (switch decision, error code, pcid, cr3)
- Which code path was taken (inactive CPU, no current thread, empty scheduler, or success)
- State changes beyond well-formedness preservation

---

## Results: ALL 15 TESTS FAILED VERIFICATION ✅

All tests correctly fail, meaning the specification **rejects** all queried unintended properties.

### Boundary Tests (5/5 FAIL) ✅
| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_cpu_id_at_num_cpus` | cpu_id == NUM_CPUS out of range | FAIL ✅ |
| 2 | `test_boundary_cpu_id_usize_max` | usize::MAX violates range | FAIL ✅ |
| 3 | `test_boundary_noswitch_is_switch` | NoSwitch ≠ Switch | FAIL ✅ |
| 4 | `test_boundary_switch_pcid_none` | Switch has Some(pcid), not None | FAIL ✅ |
| 5 | `test_boundary_running_equals_scheduled` | RUNNING ≠ SCHEDULED | FAIL ✅ |

### Behavioral Mutation Tests (5/5 FAIL) ✅
| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_mutation_noswitch_becomes_nothread` | Mutated decision NoSwitch→NoThread | FAIL ✅ |
| 2 | `test_mutation_nothread_becomes_switch` | Mutated decision NoThread→Switch | FAIL ✅ |
| 3 | `test_mutation_switch_cr3_none` | Mutated cr3 Some→None | FAIL ✅ |
| 4 | `test_mutation_switch_error_code` | Mutated error_code Else→Error | FAIL ✅ |
| 5 | `test_mutation_thread_state_blocked` | Mutated thread state RUNNING→BLOCKED | FAIL ✅ |

### Logical Tests (5/5 FAIL) ✅
| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_logical_wf_implies_switch_decision` | wf() alone doesn't determine return | FAIL ✅ |
| 2 | `test_logical_noswitch_deterministic_error` | No determinism guarantee on error_code | FAIL ✅ |
| 3 | `test_logical_nothread_equals_noswitch` | Distinct enum variants not equal | FAIL ✅ |
| 4 | `test_logical_cpu_id_must_be_even` | No parity constraint on cpu_id | FAIL ✅ |
| 5 | `test_logical_wf_implies_mem_man_unchanged` | wf() doesn't imply identical mem_man | FAIL ✅ |

---

## Conclusions

The specification for `schedule_idle_cpu` correctly rejects all 15 adversarial queries across boundary, behavioral, and logical categories. No unintended entailments were discovered.

**Spec weakness note**: While the spec correctly rejects invalid properties, it is notably weak — the only postcondition is `self.wf()`. It does not specify:
- Return value semantics (which `SyscallReturnStruct` variant corresponds to which input condition)
- Whether `mem_man` or `page_alloc` are preserved
- Relationship between input CPU state and output decision

This weakness means the spec **cannot be used** to prove return-value properties at call sites — a potential completeness concern, though not an inconsistency.
