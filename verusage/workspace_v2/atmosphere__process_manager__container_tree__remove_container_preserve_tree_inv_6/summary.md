# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__container_tree__remove_container_preserve_tree_inv_6.rs`
**Date:** 2026-03-24T07:43:03Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 5
- False positives: 0

## Summary

All five candidates are true positives. Three share the root cause that Ghost resource-ownership fields (`owned_threads`, `owned_endpoints`, `owned_procs`) are completely unconstrained by `container_tree_wf` and `container_perms_wf` — no disjointness, and `remove_container_ensures` silently drops them without transfer. The `no_child_imply_no_subtree` external_body creates a trusted but unproved bridge between children emptiness and subtree emptiness. The root container's parent field is never constrained to be None despite depth==0.

## True Positives (Spec Issues)

### owned_threads_shared_across_containers
- **Confidence:** medium
- **Reasoning:** Neither `container_perms_wf` nor `container_tree_wf` constrains `owned_threads` disjointness. The Ghost field is completely unconstrained by the tree invariants, allowing two distinct containers to claim the same thread.

### no_child_imply_no_subtree_external_body
- **Confidence:** medium
- **Reasoning:** This is an `external_body` proof claiming empty children implies empty subtree_set. While likely true given the tree structure, `subtree_set_wf` only constrains the forward direction (subtree member → ancestor relationship) and `container_subtree_set_exclusive` links subtree to uppertree, but neither provides an inductive completeness argument that would formally justify this without trusted code.

### remove_container_no_resource_cleanup
- **Confidence:** medium
- **Reasoning:** `remove_container_ensures` removes the container from the domain but has no postcondition about its `owned_threads`, `owned_endpoints`, or `owned_procs`. Resources owned by the removed container are silently lost with no spec-level transfer or cleanup accounting.

### root_container_parent_unconstrained
- **Confidence:** medium
- **Reasoning:** `container_root_wf` sets depth==0 and requires non-root nodes to have `parent.is_Some()`, but never asserts `root.parent.is_None()`. The root can carry a spurious parent pointer, which could cause confusion in upward traversal logic.

### owned_endpoints_shared_across_containers
- **Confidence:** medium
- **Reasoning:** Same pattern as `owned_threads` — `owned_endpoints` is a `Ghost<Set<EndpointPtr>>` with no cross-container disjointness constraint. The source comments explicitly discuss scoped endpoint ownership within subtrees, yet the spec never enforces it.

## All Candidates

### φ1: owned_threads_shared_across_containers
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** No disjointness constraint on owned_threads across containers — two containers can claim the same thread simultaneously
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Neither `container_perms_wf` nor `container_tree_wf` constrains `owned_threads` disjointness. The Ghost field is completely unconstrained by the tree invariants, allowing two distinct containers to claim the same thread.

### φ2: no_child_imply_no_subtree_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** no_child_imply_no_subtree is external_body — the claim that empty children implies empty subtree_set is trusted without proof, yet subtree_set_wf only constrains membership direction (subtree member implies ancestor in uppertree), not completeness
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** This is an `external_body` proof claiming empty children implies empty subtree_set. While likely true given the tree structure, `subtree_set_wf` only constrains the forward direction (subtree member → ancestor relationship) and `container_subtree_set_exclusive` links subtree to uppertree, but neither provides an inductive completeness argument that would formally justify this without trusted code.

### φ3: remove_container_no_resource_cleanup
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Removed container's owned_threads/endpoints/procs are silently dropped — remove_container_ensures never transfers or validates resource cleanup
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `remove_container_ensures` removes the container from the domain but has no postcondition about its `owned_threads`, `owned_endpoints`, or `owned_procs`. Resources owned by the removed container are silently lost with no spec-level transfer or cleanup accounting.

### φ4: root_container_parent_unconstrained
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** container_root_wf constrains depth==0 but never asserts root.parent.is_None() — root can have a spurious parent pointer
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `container_root_wf` sets depth==0 and requires non-root nodes to have `parent.is_Some()`, but never asserts `root.parent.is_None()`. The root can carry a spurious parent pointer, which could cause confusion in upward traversal logic.

### φ5: owned_endpoints_shared_across_containers
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** owned_endpoints Ghost field has no disjointness constraint despite source comments about scoped endpoint ownership within container subtrees
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Same pattern as `owned_threads` — `owned_endpoints` is a `Ghost<Set<EndpointPtr>>` with no cross-container disjointness constraint. The source comments explicitly discuss scoped endpoint ownership within subtrees, yet the spec never enforces it.

