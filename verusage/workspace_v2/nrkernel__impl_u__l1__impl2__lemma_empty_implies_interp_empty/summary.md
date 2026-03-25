# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_empty_implies_interp_empty.rs`
**Date:** 2026-03-24T12:38:52Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `lemma_empty_implies_interp_aux_empty` is an `external_body` axiom trusting the structural induction that empty directories produce empty interpretations. The other four are false positives — downstream consequences of the same axiom, trivially true out-of-bounds cases, intended invariant properties, or tautological ensures clauses.

## True Positives (Spec Issues)

### empty_interp_aux_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_empty_implies_interp_aux_empty` is `external_body` with `unimplemented!()` body — the structural induction proving empty directories yield empty interpretations is trusted without proof. The `lemma_empty_implies_interp_empty` wrapper depends entirely on this axiom.

## All Candidates

### φ1: empty_interp_aux_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_empty_implies_interp_aux_empty` is `external_body` — the key structural induction that empty directories have empty interpretations is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_empty_implies_interp_aux_empty` is `external_body` with `unimplemented!()` body — the structural induction proving empty directories yield empty interpretations is trusted without proof. The `lemma_empty_implies_interp_empty` wrapper depends entirely on this axiom.

### φ2: empty_interp_dom_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `interp().dom()` emptiness depends on the external_body lemma — if that axiom were wrong, non-empty domains could leak from empty directories
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct downstream consequence of the same external_body already captured by φ1. The verified `lemma_empty_implies_interp_empty` wrapper adds no new trust gap.

### φ3: empty_interp_aux_any_start
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The external_body claims emptiness for ANY `i`, including out-of-bounds — the open spec base case handles `i >= len` but the external_body is not bounded
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For `i > entries.len()`, the open spec `interp_aux` directly returns `map![]` without needing the external_body. The out-of-bounds case is trivially true by definition.

### φ4: page_frame_size_matches_entry
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Page frame sizes must match entry size — tests the `pages_match_entry_size` invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the `pages_match_entry_size` clause in `inv()`. Correct and intended property.

### φ5: no_frame_alignment_constraint
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `inv()` has `frames_aligned` commented out — page frame base addresses have no alignment constraint whatsoever
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause `x == x` is a tautology — it proves nothing about missing alignment. The observation is valid but this PHI doesn't actually test it.

