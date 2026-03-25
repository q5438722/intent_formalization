# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/extras/extra__lemma_aligned_iff_eq_mul_div.rs`
**Date:** 2026-03-24T12:25:47Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The file contains a single fully verified lemma proving the equivalence between alignment and division roundtrip using nonlinear arithmetic. All tested properties are correct mathematical facts with no external_body or trust gaps.

## All Candidates

### φ1: aligned_iff_mul_div
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The biconditional between alignment and division roundtrip — direct consequence of the verified lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct restatement of the fully verified `lemma_aligned_iff_eq_mul_div`. Correct mathematical equivalence proved by nonlinear arithmetic.

### φ2: aligned_implies_div_exact
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Forward direction — when aligned, division is exact with no remainder
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Forward direction of the verified biconditional. Correct mathematical fact.

### φ3: mul_div_implies_aligned
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Reverse direction — exact division roundtrip implies alignment
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Reverse direction of the verified biconditional. Correct mathematical fact.

### φ4: aligned_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Zero is aligned to everything — boundary test of the `%` definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `0 % b == 0` is trivially true for `b > 0`. Correct by definition of modulo.

### φ5: aligned_multiple
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Any multiple of `b` is aligned to `b` — tests that multiples satisfy the alignment predicate
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `(a * b) % b == 0` is a standard arithmetic identity proved by nonlinear arithmetic. Correct.

