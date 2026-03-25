# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/extras/extra__aligned_transitive.rs`
**Date:** 2026-03-24T12:23:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `lemma_aligned_iff_eq_mul_div` is an `external_body` axiom trusting the equivalence between modular arithmetic and division roundtrip without proof. The other four are false positives — transitivity is a verified wrapper, zero/self-alignment are tautologies, and the roundtrip is a redundant instantiation of the same axiom.

## True Positives (Spec Issues)

### lemma_aligned_iff_eq_mul_div_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_aligned_iff_eq_mul_div` is `external_body` with `unimplemented!()` body — the biconditional between `a % b == 0` and `a == b * (a / b)` is a correct mathematical fact but trusted without proof. The `aligned_transitive` proof depends on this axiom.

## All Candidates

### φ1: lemma_aligned_iff_eq_mul_div_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_aligned_iff_eq_mul_div` is `external_body` — the equivalence between `a % b == 0` and `a == b * (a / b)` is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_aligned_iff_eq_mul_div` is `external_body` with `unimplemented!()` body — the biconditional between `a % b == 0` and `a == b * (a / b)` is a correct mathematical fact but trusted without proof. The `aligned_transitive` proof depends on this axiom.

### φ2: aligned_transitive_verified
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Transitivity of alignment depends on the external_body `lemma_aligned_iff_eq_mul_div` — the proof is verified but its foundation is unverified
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The transitivity proof itself is verified — the trust gap is in the underlying `lemma_aligned_iff_eq_mul_div` already captured by φ1. This wrapper adds no new information.

### φ3: aligned_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Zero is aligned to every nonzero size — tests the trivial base case of the alignment spec
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `0 % b == 0` is trivially true for any `b > 0`. This follows directly from the open spec definition of `aligned` without any external_body dependency.

### φ4: aligned_self
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Every nonzero value is self-aligned — tests the `a % a == 0` tautology
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `a % a == 0` is a mathematical tautology for `a > 0`. No external_body needed.

### φ5: aligned_mul_div_roundtrip
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** When aligned, `a` equals `b * (a / b)` exactly — the forward direction of the biconditional relies on the external_body axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is just the forward direction of the biconditional from the same external_body axiom already flagged by φ1. It adds no new trust gap.

