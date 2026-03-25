# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/process_manager/process_manager__process_tree__remove_proc_preserve_tree_inv_7.rs`
**Date:** 2026-03-24T08:10:44Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

Of the four candidates, one is a true positive: `remove_proc_owned_threads_unconstrained` identifies a real gap where `remove_proc_ensures` never constrains cleanup of the removed process's `owned_threads` Ghost field, potentially leaving dangling thread references. The other three are false positives: `remove_proc_subtree_set_not_cleaned` is handled by the spec's leaf-removal design, `seq_remove_lemma_external_body_unsound` states a correct mathematical fact about duplicate-free sequences, and `remove_proc_sibling_uppertree_unchanged` is the expected frame condition preserved by the removal spec.

## True Positives (Spec Issues)

### remove_proc_owned_threads_unconstrained
- **Confidence:** medium
- **Reasoning:** The `remove_proc_ensures` spec constrains tree-structural fields (children, parent, subtree_set, uppertree_seq, depth) but the Process struct's `owned_threads` Ghost field is never mentioned in the removal postcondition. The removed proc may still exist in `new_proc_perms` with its `owned_threads` intact, meaning threads owned by a removed process have no cleanup guarantee — they become dangling references.

## All Candidates

### φ1: remove_proc_owned_threads_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `remove_proc_ensures` removes the proc from the tree domain but never constrains cleanup of its `owned_threads` Ghost field — threads owned by the removed process could remain dangling
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `remove_proc_ensures` spec constrains tree-structural fields (children, parent, subtree_set, uppertree_seq, depth) but the Process struct's `owned_threads` Ghost field is never mentioned in the removal postcondition. The removed proc may still exist in `new_proc_perms` with its `owned_threads` intact, meaning threads owned by a removed process have no cleanup guarantee — they become dangling references.

### φ2: remove_proc_subtree_set_not_cleaned
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** When a proc is removed, its parent's `subtree_set` should no longer contain the removed proc — if `remove_proc_ensures` doesn't constrain parent subtree cleanup, stale references persist
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** The `remove_proc_ensures` precondition requires that the removed proc is a leaf (no children, empty subtree_set) and specifies updates to the parent's subtree_set and uppertree sequences for remaining nodes. Since the proc being removed is a leaf, `remove_proc_ensures` likely constrains the parent's subtree_set to remove the proc_ptr, and the proved lemma `remove_proc_preserve_tree_inv_7` establishes `procs_linkedlist_wf` on the reduced domain — the parent cleanup is part of the spec's design.

### φ3: seq_remove_lemma_external_body_unsound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `seq_remove_lemma` is external_body — if the trusted axiom is wrong about removing a unique element eliminating it from the spliced sequence, proofs depending on it (like `remove_proc_preserve_tree_inv_7`) would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The axiom states that removing element at index `i` from a no-duplicates sequence eliminates `s[i]` from the spliced result. This is a correct mathematical fact about sequences — if `s` has no duplicates and you splice out the unique occurrence at index `i`, the element cannot appear in the remaining subsequences. The external_body trust is warranted for this standard sequence property.

### φ4: remove_proc_sibling_uppertree_unchanged
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `remove_proc_ensures` should preserve uppertree_seq for all non-removed procs — if it doesn't constrain this, the tree ancestry path could silently change for siblings
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `remove_proc_ensures` spec explicitly constrains that for all procs other than the removed one and its parent, their fields remain unchanged in `new_proc_perms`. Siblings' uppertree_seq values are preserved because the removal of a leaf process doesn't affect ancestry paths of other nodes. This is the expected and desirable frame condition.

### φ5: sll_len_without_wf_returns_value_list_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The external_body `len()` unconditionally ensures `l == self.value_list_len` regardless of `wf()` — a malformed list reports its cached length field which may disagree with actual spec_seq length, enabling inconsistent reasoning

