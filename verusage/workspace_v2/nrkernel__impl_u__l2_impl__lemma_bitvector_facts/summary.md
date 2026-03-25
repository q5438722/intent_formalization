# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l2_impl/impl_u__l2_impl__lemma_bitvector_facts.rs`
**Date:** 2026-03-24T13:02:53Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The file contains fully verified bitvector lemmas with `by (bit_vector)` proofs — no external_body axioms or trust gaps. All properties are correct mathematical consequences of the mask definitions and standard bitvector arithmetic.

## All Candidates

### φ1: mask_neg_dirty_access_identity_no_bits
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Tests that clearing already-clear dirty/access bits is identity — verified consequence of the bitvector lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and desirable property — if bits 5 and 6 are already clear, ANDing with the complement mask is identity. Verified by `by (bit_vector)` in the lemma body.

### φ2: bit_test_dichotomy
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Each bit test yields exactly 0 or bit(i) — tests whether the bitvector fact's biconditional implies this strict dichotomy
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Standard bitvector fact: `v & (1 << i)` is always either 0 or `1 << i`. This is a correct mathematical consequence of single-bit masking, not a spec gap.

### φ3: mask_neg_dirty_access_preserves_bit0
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Masking out dirty/access bits preserves bit 0 — tests that MASK_NEG_DIRTY_ACCESS doesn't accidentally clear the present bit
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Correct and desirable — MASK_NEG_DIRTY_ACCESS only clears bits 5 and 6, so bit 0 (present bit) is preserved. Verified by `by (bit_vector)`.

### φ4: mask_neg_dirty_access_clears_bit5
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** MASK_NEG_DIRTY_ACCESS should clear bit 5 (access bit) — tests that the mask definition correctly zeros the access flag
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Correct and desirable — the entire purpose of MASK_NEG_DIRTY_ACCESS is to clear bit 5 (access). This confirms the mask works as intended.

### φ5: mask_neg_dirty_access_clears_bit6
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** MASK_NEG_DIRTY_ACCESS should clear bit 6 (dirty bit) — tests that the mask definition correctly zeros the dirty flag
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Correct and desirable — the entire purpose of MASK_NEG_DIRTY_ACCESS is to clear bit 6 (dirty). This confirms the mask works as intended.

