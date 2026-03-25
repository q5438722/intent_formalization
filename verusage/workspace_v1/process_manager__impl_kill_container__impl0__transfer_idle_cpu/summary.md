# Adversarial Proof Test Summary

**Target**: `process_manager__impl_kill_container__impl0__transfer_idle_cpu.rs`
**Function under test**: `ProcessManager::transfer_idle_cpu`

## Overview

The `transfer_idle_cpu` function transfers an idle CPU from a child container to its parent container. It requires: well-formed PM (`wf()`), valid CPU ID, container in domain, container owns the CPU, CPU is idle (no current thread), container is not root (depth ≠ 0).

**13 adversarial proof tests** were generated across 3 categories. All tests **correctly failed verification**, meaning the specification properly rejects invalid inputs, incorrect behaviors, and unintended logical reasoning.

---

## Results

### Boundary Tests (5/5 failed ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_boundary_1_cpu_out_of_range` | Owned CPU must have id < NUM_CPUS | FAIL ✅ |
| `test_boundary_2_inactive_cpu_has_thread` | Inactive CPU must have no current thread | FAIL ✅ |
| `test_boundary_3_non_running_has_cpu` | Non-RUNNING thread must have no running_cpu | FAIL ✅ |
| `test_boundary_4_container_self_child` | Container cannot be its own child | FAIL ✅ |
| `test_boundary_5_blocked_no_endpoint` | BLOCKED thread must have blocking endpoint | FAIL ✅ |

**Analysis**: The spec correctly constrains CPU ID ranges (`container_cpu_wf`), CPU-thread consistency (`cpus_wf`), thread state-CPU biconditional (`threads_cpu_wf`), structural container validity (`container_perms_wf`), and blocked thread invariants (`endpoints_queue_wf`).

### Behavioral Mutation Tests (4/4 failed ✅)

| Test | Mutation Applied | Result |
|------|-----------------|--------|
| `test_mutation_1_cpu_wrong_owner` | CPU's owning_container ≠ container (should be ==) | FAIL ✅ |
| `test_mutation_2_proc_wrong_container` | Process's owning_container ≠ container (should be ==) | FAIL ✅ |
| `test_mutation_3_scheduler_wrong_state` | Scheduled thread state = BLOCKED (should be SCHEDULED) | FAIL ✅ |
| `test_mutation_4_same_pcid` | Two distinct processes have same PCID (should differ) | FAIL ✅ |

**Analysis**: The spec correctly enforces the CPU-container ownership bijection (`container_cpu_wf`), process-container membership consistency (`processes_container_wf`), scheduler-state correspondence (`schedulers_wf`), and PCID uniqueness (`pcid_ioid_wf`).

### Logical Tests (4/4 failed ✅)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_1_two_containers_same_cpu` | Two containers can share the same CPU | FAIL ✅ |
| `test_logical_2_all_cpus_active` | wf() implies all CPUs are active | FAIL ✅ |
| `test_logical_3_thread_two_schedulers` | Thread can be in two schedulers simultaneously | FAIL ✅ |
| `test_logical_4_domains_equal` | Container domain equals process domain | FAIL ✅ |

**Analysis**: The spec correctly prevents shared CPU ownership (via bijection in `container_cpu_wf`), does not over-constrain CPU activity status, prevents multi-scheduler membership (via `schedulers_wf` owning_container uniqueness), and maintains domain disjointness (`memory_disjoint`).

---

## Conclusion

All 13 tests correctly failed verification. The specification of `transfer_idle_cpu` and its supporting invariants (`wf()`) appear **consistent** with respect to the tested properties:

- **Precondition boundaries** are properly enforced
- **Behavioral relationships** (ownership, state, identity) reject mutations
- **Logical properties** (uniqueness, exclusivity, non-over-constraining) are sound

**Notable observation**: `transfer_idle_cpu` has **no `ensures` clause**, meaning callers receive no formal postcondition guarantees about the result state. While internal assertions verify invariant preservation, this limits compositional reasoning. This is a potential area for specification strengthening.
