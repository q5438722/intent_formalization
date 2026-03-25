# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__process_tree__new_proc_preserve_tree_inv.rs`
**Date:** 2026-03-24T08:01:14Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. The new process having an empty subtree set and zero children are exactly the correct properties for a freshly created leaf node in a process tree. These confirm that `new_proc_ensures` properly constrains the initial state of a new process rather than leaving it under-specified.

## All Candidates

### φ1: new_proc_no_owned_threads_constraint
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `new_proc_ensures` constrains tree-structural fields (parent, children, depth, subtree, uppertree) but may not constrain `owned_threads` of the new process, allowing it to start with arbitrary thread ownership

### φ2: new_proc_subtree_only_contains_self
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A newly created leaf process should have an empty subtree set (or only itself) — if `new_proc_ensures` doesn't constrain this, the new process could claim arbitrary processes in its subtree
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A newly created leaf process with no children should indeed have an empty subtree set. The `new_proc_ensures` spec explicitly constrains the new process's subtree_set to be empty (a leaf has no descendants in its subtree — the subtree_set tracks *descendants*, not the node itself). This is the expected and desirable property for a freshly created leaf node.

### φ3: new_proc_children_empty
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A freshly created process should have no children — if `new_proc_ensures` doesn't enforce empty children, the new process could be born with phantom child references to existing nodes, breaking tree structure
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A freshly created process must have zero children — it is born as a leaf. The `new_proc_ensures` spec constrains the new process's children list to have length 0 (visible from the `new_proc_perms[new_proc_ptr].value().children.len() == 0` clause in the omitted lines 353–422). This is a correct and desirable invariant, not a spec gap.

### φ4: new_proc_preserve_inv_1_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `new_proc_preserve_tree_inv_1` is external_body — it asserts tree invariant preservation without verified proof; if `new_proc_ensures` is too weak (missing constraints), this trusted lemma could mask a real invariant violation

### φ5: new_proc_parent_subtree_unchanged_others
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** When adding a new process under `proc_ptr`, only `proc_ptr` and its ancestors should gain the new process in their subtree sets — if `new_proc_ensures` doesn't update ancestor subtree sets, non-parent nodes correctly stay unchanged, but ancestors like `root_proc` would be missing the new process from their subtree

