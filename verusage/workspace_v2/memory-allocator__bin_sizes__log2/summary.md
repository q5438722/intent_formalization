# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__log2.rs`
**Date:** 2026-03-24T10:23:49Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `pow2` returning 1 for all negative exponents is a non-standard design choice that silently maps an entire domain of invalid inputs to a plausible-looking value (1), potentially masking caller bugs. The remaining four are false positives confirming standard mathematical properties (2^0=1, floor-log2(1)=0, shift-by-zero identity, zero-division).

## True Positives (Spec Issues)

### pow2_negative_is_one
- **Confidence:** medium
- **Reasoning:** `pow2` returns 1 for all negative exponents, whereas mathematically 2^(-n) = 1/2^n (a fraction, not 1). Callers passing a negative exponent by mistake get `1` instead of a signal that something is wrong, silently masking the error.

## All Candidates

### φ1: pow2_zero_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If `pow2(0)` weren't 1, the entire tower of pow2 properties would be inconsistent with standard mathematical convention
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `pow2(0) == 1` is the standard mathematical definition of 2^0. Direct consequence of the spec.

### φ2: pow2_negative_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2` returns 1 for all negative exponents — this is a non-standard choice that could silently mask errors when callers accidentally pass negative values
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `pow2` returns 1 for all negative exponents, whereas mathematically 2^(-n) = 1/2^n (a fraction, not 1). Callers passing a negative exponent by mistake get `1` instead of a signal that something is wrong, silently masking the error.

### φ3: log2_one_is_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `log2(1)` should return 0, placing 1 in `[pow2(0), pow2(1))` = `[1, 2)` — if bounds were wrong, log2 wouldn't correctly identify powers of two
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `1 ∈ [pow2(0), pow2(1)) = [1, 2)` is the correct floor-log2 result. Standard mathematical property.

### φ4: shift_zero_is_identity
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Right-shifting by 0 should be the identity — if `shift_is_div` didn't handle the base case correctly, the recursive proof would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `x >> 0 == x / 1` is the correct base case — right-shifting by zero is the identity. Standard property.

### φ5: div_by_multiple_zero_numerator
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Dividing `0 * d` by `d` must yield 0 — if the lemma's precondition `0 <= b` didn't correctly handle the zero boundary, it could produce an incorrect result
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `(0 * d) / d == 0` is basic arithmetic. The lemma correctly handles `b = 0`.

