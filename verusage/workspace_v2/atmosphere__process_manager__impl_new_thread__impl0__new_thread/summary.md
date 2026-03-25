# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__impl_new_thread__impl0__new_thread.rs`
**Date:** 2026-03-24T08:02:38Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate is a false positive. The property that a running thread's owning container matches its CPU's owning container is directly entailed by the `threads_cpu_wf` predicate, which explicitly constrains `owning_container` equality between CPUs and their current threads. This is an intentional isolation invariant, not a spec weakness.

## All Candidates

### φ1: owned_threads_shared_across_containers
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The `owned_threads` Ghost field has no cross-container disjointness constraint — two distinct containers could claim ownership of the same thread simultaneously

### φ2: owned_endpoints_shared_across_containers
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The `owned_endpoints` Ghost field lacks any disjointness predicate across containers, allowing the same endpoint to appear in multiple containers' ownership sets

### φ3: thread_running_cpu_container_mismatch
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A running thread's owning_container should match its CPU's owning_container — if this isn't entailed, a thread could run on a CPU belonging to a different container, breaking isolation
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `threads_cpu_wf` spec explicitly ensures that when a CPU is active with a current thread, `self.cpu_list@[cpu_i].owning_container == self.thread_perms@[current_thread].value().owning_container`. Combined with the biconditional linking `running_cpu.is_Some()` to `RUNNING` state and the thread-CPU mutual reference, this container equality is an intentional and desirable invariant ensuring isolation — not a spec gap.

### φ4: no_root_process_allows_owned_threads
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A container with no root process and zero owned_procs should not own any threads — but `process_trees_wf` only constrains `owned_procs.len() == 0`, leaving `owned_threads` Ghost field unconstrained

### φ5: get_proc_container_dom_without_full_wf
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `get_proc` external_body ensures `self.wf() ==> container_dom.contains(ret.owning_container)` — with only `proc_perms_wf + process_fields_wf` (not full wf), the container_dom membership is NOT guaranteed, yet SMT might derive it from leaked constraints

