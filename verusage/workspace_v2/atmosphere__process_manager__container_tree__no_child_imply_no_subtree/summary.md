# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__container_tree__no_child_imply_no_subtree.rs`
**Date:** 2026-03-24T07:34:23Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 4
- False positives: 0

## Summary

All four candidates are true positives. Three share the same root cause: the container tree well-formedness predicates (`container_perms_wf`, `container_tree_wf`) focus exclusively on structural tree properties (parent-child, depth, subtree/uppertree sequences) but completely ignore the resource ownership fields (`owned_threads`, `owned_endpoints`, `owned_procs`). These Ghost fields have no cross-container disjointness constraints, and tree structure is fully decoupled from resource ownership — a leaf with empty subtree can still own arbitrary resources. The fourth finding identifies a minor but real gap: the root container's `parent` field is never constrained to be `None`, allowing a spurious parent pointer despite the root being identified by `depth == 0`.

## True Positives (Spec Issues)

### owned_threads_shared_across_containers
- **Confidence:** medium
- **Reasoning:** Neither `container_perms_wf` nor `container_tree_wf` includes any disjointness constraint on `owned_threads` across containers. The `owned_threads` Ghost field is completely unconstrained by the tree invariants, allowing two distinct containers to claim the same thread — a real resource ownership gap.

### owned_endpoints_shared_across_containers
- **Confidence:** medium
- **Reasoning:** Same pattern — `owned_endpoints` is a `Ghost<Set<EndpointPtr>>` with no cross-container disjointness constraint in the tree or perms well-formedness predicates. The code comments explicitly discuss endpoint ownership scoping within subtrees, yet the spec never enforces it.

### no_child_no_subtree_but_owns_resources
- **Confidence:** medium
- **Reasoning:** This confirms that tree structural invariants and resource ownership are completely decoupled. A leaf container with empty children and empty subtree can still own threads, meaning `no_child_imply_no_subtree` provides no guarantee about resource cleanup. The spec lacks a predicate connecting tree structure to resource ownership.

### root_container_parent_unconstrained
- **Confidence:** medium
- **Reasoning:** `container_root_wf` constrains `depth == 0` and requires non-root nodes to have `parent.is_Some()`, but never asserts `container_perms[root_container].value().parent.is_None()`. The root can carry a spurious parent pointer that could confuse upward-traversal algorithms or invariant reasoning.

## All Candidates

### φ1: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body unconditionally ensures `l == self.value_list_len` without requiring wf() — the raw field is exposed for malformed lists

### φ2: owned_threads_shared_across_containers
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Neither container_perms_wf nor container_tree_wf constrains owned_threads disjointness — two distinct containers can claim ownership of the same thread
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Neither `container_perms_wf` nor `container_tree_wf` includes any disjointness constraint on `owned_threads` across containers. The `owned_threads` Ghost field is completely unconstrained by the tree invariants, allowing two distinct containers to claim the same thread — a real resource ownership gap.

### φ3: owned_endpoints_shared_across_containers
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** owned_endpoints is Ghost<Set<EndpointPtr>> with no disjointness constraint — two containers can claim the same endpoint despite comments about endpoint ownership scoping
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Same pattern — `owned_endpoints` is a `Ghost<Set<EndpointPtr>>` with no cross-container disjointness constraint in the tree or perms well-formedness predicates. The code comments explicitly discuss endpoint ownership scoping within subtrees, yet the spec never enforces it.

### φ4: no_child_no_subtree_but_owns_resources
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A leaf container (empty children, empty subtree) can still own threads — tree structural invariants are completely decoupled from resource ownership, so no_child_imply_no_subtree says nothing about resource cleanup
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** This confirms that tree structural invariants and resource ownership are completely decoupled. A leaf container with empty children and empty subtree can still own threads, meaning `no_child_imply_no_subtree` provides no guarantee about resource cleanup. The spec lacks a predicate connecting tree structure to resource ownership.

### φ5: root_container_parent_unconstrained
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** container_root_wf constrains depth==0 and non-root must have parent.is_Some(), but never asserts root's parent.is_None() — the root can have a spurious parent pointer
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `container_root_wf` constrains `depth == 0` and requires non-root nodes to have `parent.is_Some()`, but never asserts `container_perms[root_container].value().parent.is_None()`. The root can carry a spurious parent pointer that could confuse upward-traversal algorithms or invariant reasoning.

