# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__impl_base__impl0__block_running_thread.rs`
**Date:** 2026-03-24T07:44:26Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

The single candidate is a true positive. The `owned_threads` Ghost field on `Container` lacks any cross-container disjointness constraint in the well-formedness predicates, meaning the spec permits two distinct containers to simultaneously claim ownership of the same thread — a real resource ownership gap.

## True Positives (Spec Issues)

### owned_threads_shared_across_containers
- **Confidence:** medium
- **Reasoning:** Neither `container_perms_wf` nor `container_tree_wf` nor any other component of `wf()` constrains `owned_threads` disjointness across containers. The `owned_threads` Ghost field is completely unconstrained by the tree invariants, allowing two distinct containers to claim the same thread.

## All Candidates

### φ1: get_proc_leaks_container_dom_without_wf
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** get_proc's external_body ensures `self.wf() ==> container_dom contains owning_container` but the ensures clause is guarded by wf() — yet the spec_get_proc inline spec has no such guarantee, creating a gap between the exec ensures and the spec recommends

### φ2: get_thread_leaks_container_scheduler_wf
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** get_thread's external_body ensures wf() for scheduler/owned_procs/children of the owning container — these should follow from wf() invariants, but being in external_body means they're trusted without proof and could mask missing invariant clauses

### φ3: owned_threads_shared_across_containers
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** No disjointness constraint on owned_threads across containers — two containers can claim the same thread simultaneously despite the resource ownership model requiring exclusive ownership
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Neither `container_perms_wf` nor `container_tree_wf` nor any other component of `wf()` constrains `owned_threads` disjointness across containers. The `owned_threads` Ghost field is completely unconstrained by the tree invariants, allowing two distinct containers to claim the same thread.

### φ4: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body unconditionally ensures l == self.value_list_len without requiring wf() — the raw internal field is exposed even for malformed lists, bypassing the well-formedness abstraction

### φ5: inactive_cpu_has_arbitrary_owning_container
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** container_cpu_wf requires ALL cpus (active or not) to have owning_container in container_dom — inactive CPUs should arguably have no meaningful owning_container, yet the spec forces a valid container binding even when the CPU is not running

