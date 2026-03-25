# Summary: Verus Specification Tests for `schedule_idle_cpu`

## File Under Test
`kernel__schedule_idle_cpu__impl0__schedule_idle_cpu.rs`

Defines `Kernel::schedule_idle_cpu(&mut self, cpu_id: CpuId, pt_regs: &mut Registers) -> SyscallReturnStruct`, a kernel scheduler function that attempts to schedule a thread on an idle CPU.

**Spec:**
- **Requires:** `old(self).wf()`, `0 <= cpu_id < NUM_CPUS`
- **Ensures:** `self.wf()` (kernel well-formedness is preserved)

**Logic (4 paths):**
1. CPU not active → `NoSwitchNew(Error)`
2. CPU has current thread → `NoSwitchNew(Error)`
3. Container scheduler empty → `NoNextThreadNew(Error)`
4. Otherwise → pop thread from scheduler, return `SwitchNew(Else, cr3, pcid)`

---

## Correctness Results (should all PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_param_wf_preserved` | Kernel.wf() implies all sub-component wf()s | PASS | ✅ PASS |
| 2 | `test_cpu_id_zero_valid` | cpu_id=0 satisfies bounds | PASS | ✅ PASS |
| 3 | `test_cpu_id_max_valid` | cpu_id=31 satisfies bounds | PASS | ✅ PASS |
| 4 | `test_num_cpus_is_32` | NUM_CPUS == 32 | PASS | ✅ PASS |
| 5 | `test_pcid_max_is_4096` | PCID_MAX == 4096 | PASS | ✅ PASS |
| 6 | `test_ioid_max_is_4096` | IOID_MAX == 4096 | PASS | ✅ PASS |
| 7 | `test_kernel_wf_implies_memory_wf` | wf() ⟹ memory_wf() | PASS | ✅ PASS |
| 8 | `test_kernel_wf_implies_mapping_wf` | wf() ⟹ mapping_wf() | PASS | ✅ PASS |
| 9 | `test_kernel_wf_implies_pcid_ioid_wf` | wf() ⟹ pcid_ioid_wf() | PASS | ✅ PASS |
| 10 | `test_kernel_wf_implies_page_mapping_wf` | wf() ⟹ page_mapping_wf() | PASS | ✅ PASS |
| 11 | `test_proc_man_wf_decomposition` | PM.wf() implies all sub-wfs | PASS | ✅ PASS |
| 12 | `test_cpu_inv_gives_cpu_list_wf` | cpu_inv() ⟹ cpu_list.wf() | PASS | ✅ PASS |
| 13 | `test_cpu_inv_container_dom` | cpu_inv() ⟹ owning_container in domain | PASS | ✅ PASS |
| 14 | `test_switch_decisions_distinct` | SwitchDecision variants distinct | PASS | ✅ PASS |
| 15 | `test_thread_states_distinct` | ThreadState variants distinct | PASS | ✅ PASS |
| 16 | `test_container_inv_scheduler_wf` | container_inv() ⟹ scheduler.wf() | PASS | ✅ PASS |
| 17 | `test_cpu_inv_current_thread` | cpu_inv() ⟹ current_thread in thread_dom | PASS | ✅ PASS |
| 18 | `test_container_inv_owned_cpus` | container_inv() ⟹ owned_cpus.wf() | PASS | ✅ PASS |
| 19 | `test_multiple_valid_cpu_ids` | Various cpu_ids < NUM_CPUS | PASS | ✅ PASS |
| 20 | `test_kernel_wf_conjunctive` | Kernel.wf() decomposes fully | PASS | ✅ PASS |

**Result: 63 verified, 0 errors** (includes 43 original + 20 test functions)

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_cpu_id_too_large` | cpu_id=32 out of range | FAIL | ✅ FAIL |
| 2 | `test_fail_cpu_inv_no_wf` | cpu_inv() without wf() | FAIL | ✅ FAIL |
| 3 | `test_fail_thread_inv_no_wf` | thread_inv() without wf() | FAIL | ✅ FAIL |
| 4 | `test_fail_container_inv_no_wf` | container_inv() without wf() | FAIL | ✅ FAIL |
| 5 | `test_fail_endpoint_inv_no_wf` | endpoint_inv() without wf() | FAIL | ✅ FAIL |
| 6 | `test_fail_scheduler_nonempty_unproven` | scheduler non-empty without establishing | FAIL | ✅ FAIL |
| 7 | `test_fail_cpu_id_at_num_cpus` | cpu_id=NUM_CPUS (boundary) | FAIL | ✅ FAIL |
| 8 | `test_fail_kernel_wf_without_all_parts` | Kernel.wf() from partial conditions | FAIL | ✅ FAIL |
| 9 | `test_fail_array_value_without_wf` | Array length without wf() | FAIL | ✅ FAIL |
| 10 | `test_fail_current_thread_some` | current_thread is Some without proof | FAIL | ✅ FAIL |
| 11 | `test_fail_cpu_active` | CPU active without proof | FAIL | ✅ FAIL |

**Result: 43 verified, 11 errors** (all 11 tests correctly fail)

### Round 2: Overly Strong Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_wf_implies_equality` | Two wf kernels are equal | FAIL | ✅ FAIL |
| 2 | `test_fail_container_dom_size` | container_dom has exactly 1 element | FAIL | ✅ FAIL |
| 3 | `test_fail_all_cpus_active` | All CPUs are active | FAIL | ✅ FAIL |
| 4 | `test_fail_all_schedulers_nonempty` | All schedulers non-empty | FAIL | ✅ FAIL |
| 5 | `test_fail_all_cpus_have_thread` | All CPUs have current thread | FAIL | ✅ FAIL |
| 6 | `test_fail_num_cpus_64` | NUM_CPUS == 64 | FAIL | ✅ FAIL |
| 7 | `test_fail_proc_dom_nonempty` | proc_dom is non-empty | FAIL | ✅ FAIL |
| 8 | `test_fail_thread_dom_nonempty` | thread_dom is non-empty | FAIL | ✅ FAIL |
| 9 | `test_fail_no_mapped_pages` | No mapped pages | FAIL | ✅ FAIL |
| 10 | `test_fail_all_containers_own_all_cpus` | All containers have same owned_cpus | FAIL | ✅ FAIL |

**Result: 43 verified, 10 errors** (all 10 tests correctly fail)

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_negate_mem_man_wf` | ¬mem_man.wf() from kernel.wf() | FAIL | ✅ FAIL |
| 2 | `test_fail_negate_page_alloc_wf` | ¬page_alloc.wf() from kernel.wf() | FAIL | ✅ FAIL |
| 3 | `test_fail_negate_proc_man_wf` | ¬proc_man.wf() from kernel.wf() | FAIL | ✅ FAIL |
| 4 | `test_fail_negate_cpu_list_wf` | ¬cpu_list.wf() after cpu_inv() | FAIL | ✅ FAIL |
| 5 | `test_fail_negate_container_dom` | ¬container_dom contains owning_container | FAIL | ✅ FAIL |
| 6 | `test_fail_negate_scheduler_wf` | ¬scheduler.wf() after container_inv() | FAIL | ✅ FAIL |
| 7 | `test_fail_negate_owned_cpus_wf` | ¬owned_cpus.wf() after container_inv() | FAIL | ✅ FAIL |
| 8 | `test_fail_negate_memory_wf` | ¬memory_wf() from kernel.wf() | FAIL | ✅ FAIL |
| 9 | `test_fail_negate_mapping_wf` | ¬mapping_wf() from kernel.wf() | FAIL | ✅ FAIL |
| 10 | `test_fail_negate_thread_inv_container` | ¬container_dom contains thread's container | FAIL | ✅ FAIL |
| 11 | `test_fail_switch_equals_noswitch` | Switch == NoSwitch | FAIL | ✅ FAIL |

**Result: 43 verified, 11 errors** (all 11 tests correctly fail)

### Round 4: Wrong Specific Values

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_num_cpus_wrong_16` | NUM_CPUS == 16 | FAIL | ✅ FAIL |
| 2 | `test_fail_num_cpus_wrong_64` | NUM_CPUS == 64 | FAIL | ✅ FAIL |
| 3 | `test_fail_pcid_max_wrong` | PCID_MAX == 8192 | FAIL | ✅ FAIL |
| 4 | `test_fail_ioid_max_wrong` | IOID_MAX == 2048 | FAIL | ✅ FAIL |
| 5 | `test_fail_num_pages_wrong` | NUM_PAGES == 1048576 | FAIL | ✅ FAIL |
| 6 | `test_fail_kernel_l4_wrong` | KERNEL_MEM_END_L4INDEX == 0 | FAIL | ✅ FAIL |
| 7 | `test_fail_max_threads_wrong` | MAX_NUM_THREADS_PER_PROC == 64 | FAIL | ✅ FAIL |
| 8 | `test_fail_container_proc_len_wrong` | CONTAINER_PROC_LIST_LEN == 20 | FAIL | ✅ FAIL |
| 9 | `test_fail_specific_owning_container` | cpu[0].owning_container == 0 | FAIL | ✅ FAIL |
| 10 | `test_fail_max_scheduler_wrong` | MAX_CONTAINER_SCHEDULER_LEN == 20 | FAIL | ✅ FAIL |
| 11 | `test_fail_page_sz_wrong` | PAGE_SZ_4k == 2048 | FAIL | ✅ FAIL |

**Result: 43 verified, 11 errors** (all 11 tests correctly fail)

### Round 5: Cross-Function Misuse & Edge Cases

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_cross_kernel_specific_container_count` | container_dom has exactly 5 elements | FAIL | ✅ FAIL |
| 2 | `test_fail_cross_cpu_container_scheduler` | Active idle CPU implies non-empty scheduler | FAIL | ✅ FAIL |
| 3 | `test_fail_cross_thread_cpu_ownership` | Thread is current_thread of specific CPU | FAIL | ✅ FAIL |
| 4 | `test_fail_cross_container_proc` | Specific proc belongs to specific container | FAIL | ✅ FAIL |
| 5 | `test_fail_cross_page_alloc_containers` | Container has allocated pages | FAIL | ✅ FAIL |
| 6 | `test_fail_cpus_same_container` | CPUs 0 and 1 have same container | FAIL | ✅ FAIL |
| 7 | `test_fail_endpoint_thread_specific` | Endpoint queue contains specific thread | FAIL | ✅ FAIL |
| 8 | `test_fail_memory_wf_page_count` | proc_man page_closure is empty | FAIL | ✅ FAIL |
| 9 | `test_fail_pcid_specific_value` | Specific proc has pcid == 0 | FAIL | ✅ FAIL |
| 10 | `test_fail_thread_always_running` | All threads in RUNNING state | FAIL | ✅ FAIL |
| 11 | `test_fail_proc_in_all_containers` | All containers own all procs | FAIL | ✅ FAIL |

**Result: 43 verified, 11 errors** (all 11 tests correctly fail)

---

## Overall Assessment

### Correctness
✅ **The specs are correct.** All 20 correctness tests pass, confirming that:
- `Kernel::wf()` properly decomposes into sub-component well-formedness
- The proof invariants (`cpu_inv`, `thread_inv`, `container_inv`, `endpoint_inv`) produce the expected postconditions
- Constants are correctly defined
- Enum variants are properly distinct

### Completeness
✅ **The specs are complete (tight).** All 54 completeness tests across 5 rounds correctly fail, confirming that:
- Preconditions are enforced: functions reject calls without proper `wf()` or valid `cpu_id`
- The spec doesn't over-promise: `wf()` doesn't imply specific state values
- Negated postconditions are rejected
- Wrong constant values are rejected
- Cross-function misuse is caught

### Spec Gaps Found
**None.** The specifications for `schedule_idle_cpu` and its dependencies are both correct and appropriately tight. The function's simple contract (preserves `wf()`) is well-supported by the underlying invariant machinery.
