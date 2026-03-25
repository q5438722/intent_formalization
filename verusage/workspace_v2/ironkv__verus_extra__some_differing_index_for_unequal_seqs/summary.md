# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/verus_extra/verus_extra__some_differing_index_for_unequal_seqs.rs`
**Date:** 2026-03-24T10:17:17Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 0

## All Candidates

### φ1: differing_index_bounded
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the returned index could exceed the sequence length, it would allow out-of-bounds reasoning

### φ2: equal_seqs_no_differing_index
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If a sequence could differ from itself at any index, extensional equality would be broken

### φ3: swap_produces_different_seq
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Swapping two distinct elements should produce a different sequence — if it didn't, the function's ability to find the differing index would be undermined

### φ4: singleton_diff_at_zero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For singleton sequences with different elements, the only valid differing index is 0 — any other return value would be out of bounds

### φ5: differing_index_witnesses_inequality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If pointwise equality didn't imply sequence equality, the function's precondition `s1 != s2` combined with equal lengths would be satisfiable even when all elements match — breaking extensionality

