# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__l2_impl/impl_u__l2_impl__lemma_bitvector_facts_simple.rs`
**Date:** 2026-03-24T13:08:07Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The file contains fully verified bitvector lemmas with `by (bit_vector)` proofs and simple constant definitions — no external_body axioms or trust gaps. All properties are trivial bitvector identities or definitional equalities.

## All Candidates

### φ1: bit0_equals_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Tests that `1usize << 0 == 1` — a basic bitvector identity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `1usize << 0 == 1` is a trivially correct bitvector identity, verified by `by (bit_vector)`. No spec gap.

### φ2: zero_masked_is_zero
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Zero ANDed with any mask is still zero — trivial bitvector fact
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `0 & anything == 0` is a fundamental bitvector fact, verified by `by (bit_vector)`. No spec gap.

### φ3: zero_and_one_is_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Zero ANDed with 1 is 0 — trivial bitvector identity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `0 & 1 == 0` is a trivially correct bitvector identity. No spec gap.

### φ4: mask_dirty_access_value
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Tests that mask constants are defined as expected — verifies the macro expansion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** These are definitional equalities — `MASK_DIRTY_ACCESS` is defined as `bit!(5) | bit!(6)` and `MASK_NEG_DIRTY_ACCESS` is defined as `!MASK_DIRTY_ACCESS`. Correct by definition.

### φ5: shift_zero_identity
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Left shift by zero is identity for 1 — basic bitvector constant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Duplicate of φ1 — `1usize << 0 == 1` is a trivially correct bitvector constant. No spec gap.

