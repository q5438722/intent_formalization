# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_nonempty_implies_interp_contains.rs`
**Date:** 2026-03-24T12:59:27Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `lemma_interp_of_entry_contains_mapping_implies_interp_contains_mapping` is an `external_body` axiom trusting that entry-level key-value pairs propagate into the full directory interpretation without proof. The other three are false positives — a redundant ensures clause from the same axiom, a correct invariant propagation property, and a tautologically satisfiable existential helper that works as designed.

## True Positives (Spec Issues)

### interp_of_entry_implies_interp_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_interp_of_entry_contains_mapping_implies_interp_contains_mapping` is `external_body` with `unimplemented!()` body — the property that every key-value pair in an entry's interpretation appears in the full `interp()` (via `union_prefer_right` chain) is trusted without proof.

## All Candidates

### φ1: interp_of_entry_implies_interp_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_interp_of_entry_contains_mapping_implies_interp_contains_mapping` is `external_body` — the containment from entry interpretation to full interpretation is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_interp_of_entry_contains_mapping_implies_interp_contains_mapping` is `external_body` with `unimplemented!()` body — the property that every key-value pair in an entry's interpretation appears in the full `interp()` (via `union_prefer_right` chain) is trusted without proof.

### φ2: interp_of_entry_key_implies_interp_key_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The key-containment ensures clause of the same external_body — if wrong, `union_prefer_right` could drop keys from earlier entries
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the second ensures clause of the same `external_body` axiom already captured by φ1. Both key and pair containment are part of a single trust assumption — flagging them separately adds no new information.

### φ3: inv_allows_empty_directories
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `inv()` does not include `no_empty_directories` — the commented-out `self.directories_are_nonempty()` means empty subdirectories satisfy `inv()`, potentially allowing vacuous directory trees
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct consequence of the open spec `directories_obey_invariant`. The fact that `inv()` implies subdirectory `inv()` is a desirable property — the question of whether empty directories should be excluded is a separate design choice (captured by the separate `no_empty_directories` predicate used in `lemma_nonempty_implies_interp_contains`).

### φ4: no_empty_directories_not_in_inv
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A directory with an all-Invalid subdirectory (empty directory) satisfies `inv()` — the `no_empty_directories` predicate is defined but not enforced in `inv()`

### φ5: contains_entry_size_at_index_atleast_any_layer
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `contains_entry_size_at_index_atleast` is trivially satisfied by witnessing `i == layer` — tests whether this existential provides any meaningful constraint beyond tautology
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is trivially correct by the existential witness `i == layer`. The `contains_entry_size_at_index_atleast` predicate is an intentional helper for tracking which entry sizes exist at or below a given layer — being tautologically satisfiable for the current layer is expected behavior, not a spec gap.

