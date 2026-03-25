# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l1/impl_u__l1__impl2__lemma_empty_implies_interp_aux_empty.rs`
**Date:** 2026-03-24T12:37:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four verified candidates are false positives. The tautological frame base test proves nothing meaningful. The other three test intended design properties: `union_prefer_right` semantics for `interp_aux`, subdirectory base address computation from `directories_are_in_next_layer`, and allowing empty subdirectories (an intentional relaxation of the invariant). No external_body or trust gaps exist in this file — all specs are open and all proofs are verified.

## All Candidates

### φ1: inv_no_base_vaddr_alignment
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `well_formed` has the alignment constraint on `base_vaddr` commented out — directories can have unaligned base addresses

### φ2: page_frame_base_unconstrained
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `inv()` constrains page frame *size* but not frame *base* — frame base is completely unconstrained, no alignment or identity mapping requirement
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause `x == x` is a tautology — it proves nothing about frame base being unconstrained. The observation about missing frame alignment is valid but this PHI doesn't actually test it; it would verify for any spec regardless.

### φ3: interp_aux_right_overrides_left
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `interp_aux` builds the map with earlier entries overriding later ones via `union_prefer_right` — key collisions silently shadow entries without the spec proving disjointness
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct unfolding of the open spec definition of `interp_aux`. The `union_prefer_right` semantics are the intended design — each entry maps to a distinct `entry_base(i)` key, so collisions don't occur in practice.

### φ4: subdirectory_base_not_aligned
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Subdirectory `base_vaddr` is computed from parent's `base_vaddr` which has no alignment constraint — misalignment propagates through the tree
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct consequence of `directories_are_in_next_layer`, which is part of `inv()`. The subdirectory base address formula is the correct and intended design for hierarchical page tables.

### φ5: empty_dir_nonempty_allowed
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `inv()` has `directories_are_nonempty` commented out — empty subdirectories are allowed, wasting page table entries without the spec preventing it
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Subdirectories satisfying `inv()` is exactly what `directories_obey_invariant` enforces. Empty subdirectories being allowed is a deliberate design choice (the commented-out check was intentionally removed), not a spec gap.

