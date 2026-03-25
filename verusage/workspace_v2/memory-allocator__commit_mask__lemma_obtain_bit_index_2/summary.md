# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/commit_mask/commit_mask__lemma_obtain_bit_index_2.rs`
**Date:** 2026-03-24T11:13:30Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `lemma_obtain_bit_index_1` is an `external_body` axiom trusting that any nonzero usize has a set bit below 64 without proof. The other two candidates are false positives — `not_all_ones_has_zero_bit` is a correct consequence derived by a fully verified proof, and `is_bit_set_definition_match` is a tautological reveal of the spec fn's own definition with a safe `b < 64` guard.

## True Positives (Spec Issues)

### obtain_bit_index_1_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_obtain_bit_index_1` is `external_body` — the property that any nonzero usize has a set bit below 64 is mathematically true, but it is trusted without proof. This is a real trust gap in the verification.

## All Candidates

### φ1: obtain_bit_index_1_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_obtain_bit_index_1` is `external_body` — the existence of a set bit in a nonzero usize is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_obtain_bit_index_1` is `external_body` — the property that any nonzero usize has a set bit below 64 is mathematically true, but it is trusted without proof. This is a real trust gap in the verification.

### φ2: not_all_ones_has_zero_bit
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Any non-`!0` usize has a clear bit below 64 — this depends on the external_body `lemma_obtain_bit_index_1` being correct; if that axiom is wrong, this derived property is also wrong
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `lemma_obtain_bit_index_2` is fully verified (not external_body) — it correctly derives the existence of a clear bit from `lemma_obtain_bit_index_1` applied to `!a`. The property itself is mathematically correct and the proof is sound modulo the existing external_body dependency.

### φ3: zero_has_zero_bit
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Zero is not `!0usize`, so `lemma_obtain_bit_index_2` applies — but the result depends on the external_body axiom correctly finding a set bit in `!0usize` (which is all ones)

### φ4: is_bit_set_definition_match
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The spec fn `is_bit_set` is defined as `a & (1usize << b) == (1usize << b)` but for `b >= 64` the shift wraps — the spec has no `b < 64` guard in its definition, so `is_bit_set(a, 64)` has wrapped semantics
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a tautology — `is_bit_set` is defined as `a & (1usize << b) == (1usize << b)`, so revealing it and asserting the same expression trivially holds. The `b < 64` guard in the requires makes this a safe, expected usage. The wrapping concern at `b >= 64` is not exercised here.

### φ5: is_bit_set_wrap_at_64
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `1usize << 64` wraps to `1usize << 0` on 64-bit, so `is_bit_set(a, 64) == is_bit_set(a, 0)` — the spec fn conflates bit 0 and "bit 64" due to missing bounds check

