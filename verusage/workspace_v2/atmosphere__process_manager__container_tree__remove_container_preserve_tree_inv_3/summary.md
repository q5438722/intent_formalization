# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__container_tree__remove_container_preserve_tree_inv_3.rs`
**Date:** 2026-03-24T07:37:06Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 0

## All Candidates

### φ1: owned_threads_shared_across_containers
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** No disjointness constraint on owned_threads — two containers can claim the same thread simultaneously

### φ2: remove_container_no_resource_cleanup
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Removed container's owned_threads are silently dropped — remove_container_ensures never transfers or validates resource cleanup

### φ3: owned_endpoints_shared_across_containers
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** owned_endpoints Ghost field has no disjointness constraint — two containers can claim the same endpoint despite comments about scoped ownership

### φ4: root_container_parent_unconstrained
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** container_root_wf never asserts root's parent.is_None() — root container can have a spurious parent pointer

### φ5: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** StaticLinkedList::len's external_body unconditionally ensures l == self.value_list_len without requiring wf() — the raw field is exposed for malformed lists

