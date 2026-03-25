# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__spec_proof__impl2__container_owned_procs_disjoint_inv.rs`
**Date:** 2026-03-24T08:12:12Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. The CPU ownership disjointness across containers is a correct consequence of the `container_cpu_wf` bijection (each CPU has exactly one `owning_container` value, so the reverse mapping naturally enforces exclusivity). The inactive CPU having a valid owning container is intentional — the spec assigns every CPU to a container unconditionally, with `cpus_wf` only constraining that inactive CPUs have no `current_thread`. Neither finding reveals a spec gap.

## All Candidates

### φ1: get_proc_container_dom_without_full_wf
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `get_proc`'s external_body ensures `self.wf() ==> container_dom.contains(ret.owning_container)` — but without full `wf()`, the SMT solver might still derive this from the weaker preconditions, leaking a conclusion that should only hold under full well-formedness

### φ2: owned_cpus_shared_across_containers
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `container_cpu_wf` establishes a bijection between `owned_cpus` and `cpu_list[].owning_container`, but if `owned_cpus` is a Ghost field without an explicit disjointness predicate, two containers could claim the same CPU
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `container_cpu_wf` spec establishes a precise bijection: if a container's `owned_cpus` contains `cpu`, then `cpu_list[cpu].owning_container == c_ptr`, and conversely. Since `owning_container` is a single value per CPU, two distinct containers cannot both satisfy `cpu_list[cpu].owning_container == c1` and `cpu_list[cpu].owning_container == c2`. This disjointness is an intentional and desirable consequence of the ownership model.

### φ3: inactive_cpu_owning_container_unconstrained
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `container_cpu_wf` requires every CPU's `owning_container` to be in `container_dom` regardless of whether the CPU is active — an inactive CPU should arguably not need a valid owning container
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `container_cpu_wf` predicate universally quantifies over all CPUs (`0 <= cpu_i < NUM_CPUS`) requiring each to have a valid `owning_container` in `container_dom`, regardless of active status. This is a deliberate design choice — every CPU is always owned by some container, and activity status only governs whether it has a current thread. The property confirms the spec works as written.

### φ4: scheduler_thread_not_in_thread_dom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The `scheduler` StaticLinkedList contains ThreadPtrs, but neither `container_fields_wf` nor `container_tree_wf` constrains that scheduler entries are in `thread_dom` — stale or invalid thread pointers could appear in the scheduler

### φ5: owned_threads_not_subset_thread_dom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Container's `owned_threads` is a Ghost<Set<ThreadPtr>> with no predicate constraining it to be a subset of `thread_dom` — a container could claim ownership of thread pointers that don't exist in the thread permissions map

