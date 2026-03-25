# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_interp_of_entry_key_between.rs`
**Date:** 2026-03-24T12:57:32Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive: `lemma_interp_of_entry_between` is the foundational `external_body` axiom trusting VA and frame size bounding within entry ranges without proof. The second candidate is a false positive — a verified wrapper that adds no new unverified assumption beyond the same axiom.

## True Positives (Spec Issues)

### interp_of_entry_between_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_interp_of_entry_between` is `external_body` with `unimplemented!()` body — the VA bounds and frame size bounds for all key-value pairs in an entry's interpretation are trusted without proof. This is the root axiom for the entire entry bounding chain.

## All Candidates

### φ1: interp_of_entry_between_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_interp_of_entry_between` is `external_body` — both VA bounds and frame size bounds within entry range are trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_interp_of_entry_between` is `external_body` with `unimplemented!()` body — the VA bounds and frame size bounds for all key-value pairs in an entry's interpretation are trusted without proof. This is the root axiom for the entire entry bounding chain.

### φ2: key_between_depends_on_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_interp_of_entry_key_between` is verified but delegates entirely to the external_body `lemma_interp_of_entry_between` — the VA bounding chain is rooted in an unproven axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a verified downstream consequence of the same external_body axiom already captured by φ1. `lemma_interp_of_entry_key_between` simply instantiates `lemma_interp_of_entry_between` with the looked-up PTE — no new trust assumption.

### φ3: frame_size_positive_from_entry
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The external_body ensures `entry_base(i) < va + pte.frame.size` and `entry_base(i) <= va`, implying `frame.size > 0` — but this positivity is trusted, not proved

### φ4: frame_fits_within_single_entry
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The external_body constrains `va + frame.size <= next_entry_base(i)` and `entry_base(i) <= va`, so `frame.size <= entry_size` — but this upper bound on frame size within an entry is unproven

### φ5: inv_no_frames_aligned
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The commented-out `self.frames_aligned()` means `inv()` accepts page frames with arbitrary unaligned physical base addresses like 3

