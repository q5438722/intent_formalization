# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_entries_interp_equal_implies_interp_equal.rs`
**Date:** 2026-03-24T12:40:34Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `lemma_entries_interp_equal_implies_interp_aux_equal` is an `external_body` axiom trusting the inductive proof that pointwise-equal entry interpretations yield equal `interp_aux` results. The other four are false positives — a downstream wrapper of the same axiom, and three direct consequences of the verified open-spec invariant clauses.

## True Positives (Spec Issues)

### interp_aux_equal_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_entries_interp_equal_implies_interp_aux_equal` is `external_body` with `unimplemented!()` body — the inductive proof that pointwise-equal entry interpretations yield equal `interp_aux` results is trusted without proof. The `lemma_entries_interp_equal_implies_interp_equal` wrapper depends entirely on this axiom.

## All Candidates

### φ1: interp_aux_equal_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_entries_interp_equal_implies_interp_aux_equal` is `external_body` — the inductive proof that equal entry interpretations yield equal interp_aux is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_entries_interp_equal_implies_interp_aux_equal` is `external_body` with `unimplemented!()` body — the inductive proof that pointwise-equal entry interpretations yield equal `interp_aux` results is trusted without proof. The `lemma_entries_interp_equal_implies_interp_equal` wrapper depends entirely on this axiom.

### φ2: interp_equal_from_entries
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Two directories with same entry interpretations have the same full interp — depends on the external_body axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct downstream consequence of the same external_body already captured by φ1. The verified wrapper adds no new trust gap.

### φ3: page_frame_size_matches
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Page frame sizes must match entry size — tests the `pages_match_entry_size` invariant clause
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the `pages_match_entry_size` clause in `inv()`. Correct and intended invariant property.

### φ4: subdirectory_layer_increment
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Subdirectories are always exactly one layer deeper — tests the `directories_are_in_next_layer` invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the `directories_are_in_next_layer` clause in `inv()`. Correct hierarchical page table design.

### φ5: subdirectory_arch_shared
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** All subdirectories share the same arch — tests the `directories_match_arch` invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the `directories_match_arch` clause in `inv()`. Correct — all nodes in the tree share the same architecture.

