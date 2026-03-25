# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__container_tree__container_tree_check_is_ancestor.rs`
**Date:** 2026-03-24T07:27:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 4
- False positives: 0

## Summary

All four candidates are true positives. Three share the same root cause: the container tree invariants (`container_perms_wf` and `container_tree_wf`) focus exclusively on structural tree properties (parent-child relationships, depth, subtree sets, uppertree sequences) but completely ignore resource ownership fields (`owned_threads`, `owned_endpoints`, `owned_procs`). These Ghost/linked-list fields have no cross-container disjointness constraints, allowing the same thread, endpoint, or process to be claimed by multiple containers simultaneously. The fourth finding shows the root container's `parent` field is unconstrained — while non-root nodes are required to have `parent.is_Some()`, nothing forces the root to have `parent.is_None()`.

## True Positives (Spec Issues)

### owned_threads_shared_across_containers
- **Confidence:** medium
- **Reasoning:** Neither `container_perms_wf` nor `container_tree_wf` includes any constraint on `owned_threads` disjointness across containers. The `owned_threads` field is a `Ghost<Set<ThreadPtr>>` that is completely unconstrained by the tree invariants, allowing two distinct containers to claim ownership of the same thread — a real resource ownership gap.

### owned_endpoints_shared_across_containers
- **Confidence:** medium
- **Reasoning:** Same pattern as owned_threads — `owned_endpoints` is a `Ghost<Set<EndpointPtr>>` with no disjointness constraint anywhere in `container_perms_wf` or `container_tree_wf`. Two containers can simultaneously claim the same endpoint, undermining the ownership model described in the comments about endpoint kill semantics.

### owned_procs_shared_across_containers
- **Confidence:** medium
- **Reasoning:** The `owned_procs` field is a `StaticLinkedList<ProcPtr, CONTAINER_PROC_LIST_LEN>` with no cross-container disjointness constraint in the tree invariants. While `unique()` ensures no duplicates within a single container's list, nothing prevents the same `ProcPtr` from appearing in two different containers' `owned_procs`.

### root_container_parent_unconstrained
- **Confidence:** medium
- **Reasoning:** `container_root_wf` sets `depth == 0` for the root and requires non-root nodes to have `parent.is_Some()` and `depth != 0`, but never explicitly asserts `container_perms[root_container].value().parent.is_None()`. The root container can have a spurious `parent` pointer pointing anywhere, which could cause confusion in algorithms that traverse upward.

## All Candidates

### φ1: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body unconditionally ensures l == self.value_list_len without requiring wf() — the raw field is exposed even for malformed lists

### φ2: owned_threads_shared_across_containers
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Neither container_perms_wf nor container_tree_wf constrains owned_threads disjointness — two distinct containers can claim ownership of the same thread simultaneously
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Neither `container_perms_wf` nor `container_tree_wf` includes any constraint on `owned_threads` disjointness across containers. The `owned_threads` field is a `Ghost<Set<ThreadPtr>>` that is completely unconstrained by the tree invariants, allowing two distinct containers to claim ownership of the same thread — a real resource ownership gap.

### φ3: owned_endpoints_shared_across_containers
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** owned_endpoints is a Ghost<Set<EndpointPtr>> with no disjointness constraint in container_tree_wf — two containers can claim the same endpoint
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Same pattern as owned_threads — `owned_endpoints` is a `Ghost<Set<EndpointPtr>>` with no disjointness constraint anywhere in `container_perms_wf` or `container_tree_wf`. Two containers can simultaneously claim the same endpoint, undermining the ownership model described in the comments about endpoint kill semantics.

### φ4: owned_procs_shared_across_containers
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** owned_procs is a StaticLinkedList with no cross-container disjointness constraint — the same ProcPtr can appear in two containers' owned_procs lists simultaneously
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `owned_procs` field is a `StaticLinkedList<ProcPtr, CONTAINER_PROC_LIST_LEN>` with no cross-container disjointness constraint in the tree invariants. While `unique()` ensures no duplicates within a single container's list, nothing prevents the same `ProcPtr` from appearing in two different containers' `owned_procs`.

### φ5: root_container_parent_unconstrained
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** container_root_wf constrains depth==0 and non-root nodes have parent.is_Some(), but never asserts that root's parent.is_None() — the root container can have a spurious parent pointer
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `container_root_wf` sets `depth == 0` for the root and requires non-root nodes to have `parent.is_Some()` and `depth != 0`, but never explicitly asserts `container_perms[root_container].value().parent.is_None()`. The root container can have a spurious `parent` pointer pointing anywhere, which could cause confusion in algorithms that traverse upward.

