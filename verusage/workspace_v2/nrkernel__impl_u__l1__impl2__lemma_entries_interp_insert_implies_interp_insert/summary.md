# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_entries_interp_insert_implies_interp_insert.rs`
**Date:** 2026-03-24T12:42:31Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `lemma_entries_interp_insert_implies_interp_aux_insert` is an `external_body` axiom trusting the inductive proof that single-entry map insertion propagates correctly through `interp_aux`, including implicit disjointness reasoning. The other four are false positives — a verified wrapper, a branch of the same axiom, and two direct consequences of the open-spec invariant.

## True Positives (Spec Issues)

### insert_implies_interp_aux_insert_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_entries_interp_insert_implies_interp_aux_insert` is `external_body` with `unimplemented!()` body — the inductive proof that a single-entry map insertion propagates correctly through `interp_aux` (including the disjointness reasoning for `idx > i`) is trusted without proof.

## All Candidates

### φ1: insert_implies_interp_aux_insert_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_entries_interp_insert_implies_interp_aux_insert` is `external_body` — the inductive proof that single-entry map insertion propagates to interp_aux is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_entries_interp_insert_implies_interp_aux_insert` is `external_body` with `unimplemented!()` body — the inductive proof that a single-entry map insertion propagates correctly through `interp_aux` (including the disjointness reasoning for `idx > i`) is trusted without proof.

### φ2: insert_propagates_to_interp
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Full interp insertion depends on the external_body axiom — if the inductive step were wrong, map insertions could silently corrupt other entries
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct downstream consequence of the same external_body already captured by φ1. The verified wrapper adds no new trust gap.

### φ3: insert_past_idx_identity
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The external_body claims interp_aux is unchanged past the insertion index — the `idx < i` branch returns identity without proving entry-by-entry equivalence
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the `idx < i` branch of the same external_body axiom already flagged by φ1. No additional trust gap beyond what φ1 captures.

### φ4: page_frame_size_matches
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Page frame sizes must match entry size — tests the `pages_match_entry_size` invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the `pages_match_entry_size` clause in `inv()`. Correct and intended invariant property.

### φ5: subdirectory_base_computed
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Subdirectory base address is computed from parent — tests the `directories_are_in_next_layer` invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the `directories_are_in_next_layer` clause in `inv()`. Correct hierarchical page table design.

