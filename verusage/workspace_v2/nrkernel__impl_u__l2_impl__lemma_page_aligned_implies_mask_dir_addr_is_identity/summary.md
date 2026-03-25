# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l2_impl/impl_u__l2_impl__lemma_page_aligned_implies_mask_dir_addr_is_identity.rs`
**Date:** 2026-03-24T13:08:47Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `MAX_PHYADDR_WIDTH` as `external_body` with a weaker axiom than its concrete implementation value creates a specification gap. The other three are false positives — a correctly verified bitvector property, a definitional equality, and a correct arithmetic consequence.

## True Positives (Spec Issues)

### max_phyaddr_width_range
- **Confidence:** medium
- **Reasoning:** `MAX_PHYADDR_WIDTH` is `external_body` with implementation value 52, but the axiom only guarantees `32 <= MAX_PHYADDR_WIDTH <= 52`. The verifier treats it as opaque, creating a gap where proofs cannot rely on the concrete value.

## All Candidates

### φ1: page_aligned_mask_dir_identity
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Page-aligned addresses within MAX_PHYADDR are identity under MASK_DIR_ADDR — verified consequence of the bitvector proof
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and desirable property fully verified by `by (bit_vector)` reasoning. Page-aligned addresses within the physical address space should be identity under the directory address mask — this is the intended design.

### φ2: mask_dir_addr_equals_mask_addr
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** MASK_DIR_ADDR is defined as MASK_ADDR — tests that the alias is correctly propagated
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `MASK_DIR_ADDR_SPEC` is defined as `MASK_ADDR`, so this is a definitional equality. Correct by construction.

### φ3: max_phyaddr_width_range
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `MAX_PHYADDR_WIDTH` is `external_body` with value 52 but axiom only constrains to 32..=52 — spec is weaker than implementation
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `MAX_PHYADDR_WIDTH` is `external_body` with implementation value 52, but the axiom only guarantees `32 <= MAX_PHYADDR_WIDTH <= 52`. The verifier treats it as opaque, creating a gap where proofs cannot rely on the concrete value.

### φ4: max_phyaddr_positive
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** MAX_PHYADDR must be positive since it's `(1 << width) - 1` with width >= 32 — tests the arithmetic chain
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `MAX_PHYADDR = (1 << width) - 1` with `width >= 32` gives at least `2^32 - 1 > 0`. This is a correct arithmetic consequence verified through bitvector reasoning, not a spec gap.

### φ5: zero_is_page_aligned_and_masked
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is trivially page-aligned and masked to itself — boundary case for the mask identity lemma

