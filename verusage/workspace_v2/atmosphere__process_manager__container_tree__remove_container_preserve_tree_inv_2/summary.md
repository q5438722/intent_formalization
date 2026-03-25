# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__container_tree__remove_container_preserve_tree_inv_2.rs`
**Date:** 2026-03-24T07:35:43Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 4
- False positives: 0

## Summary

All four candidates are true positives. Two relate to the `remove_container_ensures` spec: it removes a container from the domain but never constrains resource ownership fields (`owned_threads`, `owned_endpoints`, `owned_procs`) — neither ensuring disjointness in the new state nor requiring resource transfer/cleanup for the removed container. The `unique_implys_no_duplicates` external_body creates a trusted but unverified bridge between two representation layers of `StaticLinkedList`. The root container's parent field remains unconstrained by `container_root_wf`, allowing a spurious parent pointer on the root node.

## True Positives (Spec Issues)

### remove_container_preserves_owned_threads_overlap
- **Confidence:** medium
- **Reasoning:** Neither `container_tree_wf` nor `remove_container_ensures` constrains `owned_threads` disjointness. After container removal, the new_container_perms still allows two distinct containers to claim the same thread — the resource ownership gap persists through the removal operation.

### remove_container_no_resource_cleanup
- **Confidence:** medium
- **Reasoning:** `remove_container_ensures` removes `container_ptr` from the domain (`!new_container_perms.dom().contains(container_ptr)`) but has no postcondition about transferring or releasing its `owned_threads`, `owned_endpoints`, or `owned_procs`. Resources owned by the removed container are silently lost with no spec-level accounting.

### unique_implys_no_duplicates_external_body
- **Confidence:** medium
- **Reasoning:** This is an `external_body` proof bridging `unique()` (defined over `spec_seq` and `value_list_len`) to `Seq::no_duplicates()`. The `unique` spec uses `self.len()` which resolves to `spec_len` → `self@.len()`, but `len()`'s external_body unconditionally returns `value_list_len` — the connection between these representations is entirely trusted without proof.

### root_container_parent_unconstrained
- **Confidence:** medium
- **Reasoning:** `container_root_wf` sets `depth == 0` for root and requires non-root nodes to have `parent.is_Some()`, but never asserts `root.parent.is_None()`. The root can carry a spurious parent pointer, which could cause confusion in upward traversal or invariant reasoning.

## All Candidates

### φ1: remove_container_preserves_owned_threads_overlap
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** remove_container_ensures does not constrain owned_threads disjointness in new_container_perms — after removal, two containers can still claim the same thread
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Neither `container_tree_wf` nor `remove_container_ensures` constrains `owned_threads` disjointness. After container removal, the new_container_perms still allows two distinct containers to claim the same thread — the resource ownership gap persists through the removal operation.

### φ2: remove_container_no_resource_cleanup
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The removed container's owned_threads are silently dropped — remove_container_ensures removes the container from dom but never transfers or validates that owned resources (threads, endpoints, procs) are handled
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `remove_container_ensures` removes `container_ptr` from the domain (`!new_container_perms.dom().contains(container_ptr)`) but has no postcondition about transferring or releasing its `owned_threads`, `owned_endpoints`, or `owned_procs`. Resources owned by the removed container are silently lost with no spec-level accounting.

### φ3: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body unconditionally ensures l == self.value_list_len without requiring wf() — the raw field is exposed for malformed lists

### φ4: unique_implys_no_duplicates_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** unique_implys_no_duplicates is external_body — it asserts that unique() (defined over spec_seq and value_list_len) implies Seq::no_duplicates() without proof, creating a trusted bridge between two different representations
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** This is an `external_body` proof bridging `unique()` (defined over `spec_seq` and `value_list_len`) to `Seq::no_duplicates()`. The `unique` spec uses `self.len()` which resolves to `spec_len` → `self@.len()`, but `len()`'s external_body unconditionally returns `value_list_len` — the connection between these representations is entirely trusted without proof.

### φ5: root_container_parent_unconstrained
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** container_root_wf constrains depth==0 and non-root must have parent.is_Some(), but never asserts root's parent.is_None() — root can have a spurious parent pointer
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `container_root_wf` sets `depth == 0` for root and requires non-root nodes to have `parent.is_Some()`, but never asserts `root.parent.is_None()`. The root can carry a spurious parent pointer, which could cause confusion in upward traversal or invariant reasoning.

