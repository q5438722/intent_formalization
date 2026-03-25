# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__mul_ordering.rs`
**Date:** 2026-03-24T10:30:07Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives identify the same underlying design choice: `pow2` silently returns 1 for all non-positive inputs, making it non-injective below 1 and potentially masking off-by-one errors. Two false positives confirm correct mathematical properties: `div2`'s unbounded `y` parameter is fine since the arithmetic identity holds for all positive integers, and commutativity of nat multiplication derived from `mul_assoc` is a true mathematical fact.

## True Positives (Spec Issues)

### pow2_negative_is_one
- **Confidence:** medium
- **Reasoning:** `pow2` returns 1 for all negative exponents, which is mathematically incorrect (2^(-n) = 1/2^n). Callers passing negative exponents by mistake silently get 1, and no precondition guards against this misuse.

### pow2_non_injective_at_zero
- **Confidence:** medium
- **Reasoning:** Direct consequence of `pow2_negative_is_one` — all non-positive inputs collapse to 1. This non-injectivity means `pow2(0) == pow2(-1)`, which could mask off-by-one errors in callers near the e=0 boundary.

## All Candidates

### φ1: pow2_negative_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2` returns 1 for all negative exponents — mathematically 2^(-n) is a fraction, so callers passing negative exponents by mistake silently get 1
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `pow2` returns 1 for all negative exponents, which is mathematically incorrect (2^(-n) = 1/2^n). Callers passing negative exponents by mistake silently get 1, and no precondition guards against this misuse.

### φ2: pow2_non_injective_at_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** All non-positive inputs collapse to 1, making `pow2` non-injective below 1 — could mask off-by-one errors near e=0 boundaries
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Direct consequence of `pow2_negative_is_one` — all non-positive inputs collapse to 1. This non-injectivity means `pow2(0) == pow2(-1)`, which could mask off-by-one errors in callers near the e=0 boundary.

### φ3: div2_no_upper_bound_on_x
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `div2` takes `x: u64` but `y: int` with no upper bound — `y * 2` can exceed u64 range, making the lemma applicable to divisions by values that don't fit in machine words
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The lemma `div2` operates on mathematical integers in the ensures clause — `x as int / (y * 2) == (x as int / y) / 2` is a correct arithmetic identity for all positive `y`. The `x: u64` parameter simply bounds the numerator; `y` being unbounded is not a soundness issue since the property holds for all positive integers.

### φ4: mul_assoc_smuggles_commutativity
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mul_assoc` ensures `(x*y)*z == y*(x*z)` which encodes commutativity — setting z=1 directly yields `x*y == y*x`, which may let proofs go through for unintended reasons
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Multiplication of natural numbers is commutative — this is a correct mathematical fact. The `mul_assoc` lemma proving `(x*y)*z == y*(x*z)` implicitly uses commutativity via `nonlinear_arith`, but deriving `x*y == y*x` from it is not a soundness gap; it's a true property of nat multiplication.

### φ5: shift_is_div_zero_shift_identity
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Right-shifting by 0 is identity — trivially true but if `shift_is_div` were unsound it would allow equating arbitrary bit-shift results with division, so this tests the base case soundness

