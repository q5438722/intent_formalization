# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__mul_assoc.rs`
**Date:** 2026-03-24T10:27:15Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives identify the same underlying issue: `pow2` silently returns 1 for all non-positive inputs, making the function non-injective below 1 and potentially masking off-by-one errors near e=0. Three false positives confirm correct mathematical properties: zero shifted by any amount is zero, natural number multiplication is commutative (correctly used by `mul_assoc`), and integer division correctly truncates remainders.

## True Positives (Spec Issues)

### pow2_negative_is_one
- **Confidence:** medium
- **Reasoning:** `pow2` returns 1 for all negative exponents, which is mathematically incorrect (2^(-n) = 1/2^n). While this is a deliberate design choice for a nat-returning function, callers passing negative exponents by mistake get `1` instead of any signal that the input is out of the intended domain — a precondition or different base case would be safer.

### pow2_zero_equals_pow2_minus1
- **Confidence:** medium
- **Reasoning:** This is a direct consequence of `pow2_negative_is_one` — the function is non-injective for e ≤ 0, collapsing all non-positive inputs to 1. This could mask off-by-one errors where a caller accidentally computes `pow2(e-1)` instead of `pow2(e)` when `e` is near 0.

## All Candidates

### φ1: pow2_negative_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2` returns 1 for all negative exponents, whereas mathematically 2^(-n) should be a fraction — callers passing negative exponents by mistake get silently wrong results
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `pow2` returns 1 for all negative exponents, which is mathematically incorrect (2^(-n) = 1/2^n). While this is a deliberate design choice for a nat-returning function, callers passing negative exponents by mistake get `1` instead of any signal that the input is out of the intended domain — a precondition or different base case would be safer.

### φ2: pow2_zero_equals_pow2_minus1
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** pow2(0) == pow2(-1) == 1 means the function is non-injective in the non-positive range, so distinct exponents produce the same value — could mask off-by-one errors in callers
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** This is a direct consequence of `pow2_negative_is_one` — the function is non-injective for e ≤ 0, collapsing all non-positive inputs to 1. This could mask off-by-one errors where a caller accidentally computes `pow2(e-1)` instead of `pow2(e)` when `e` is near 0.

### φ3: shift_zero_by_63
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Right-shifting zero always yields zero regardless of shift amount — if `shift_is_div` allowed a nonzero result for zero input, the shift-division equivalence would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Zero right-shifted by any amount is zero — this is a basic mathematical fact (0 / 2^n = 0). This confirms `shift_is_div` behaves correctly on boundary inputs.

### φ4: mul_assoc_commutativity_hidden
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mul_assoc` actually proves `(x*y)*z == y*(x*z)` which smuggles in commutativity — if this is unintentional, it could allow proofs to go through for wrong reasons
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `mul_assoc` proves `(x*y)*z == y*(x*z)`, which by `nonlinear_arith` also implicitly uses commutativity of natural number multiplication. Commutativity of multiplication is a correct mathematical fact, not a soundness gap — the lemma's name is slightly misleading but the property is valid.

### φ5: div2_loses_remainder
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Integer division truncates — `div2` lemma enables chained divisions that silently discard remainders at each step, potentially accumulating rounding errors in shift_is_div's inductive proof
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Integer division truncating remainders is the standard, correct semantics. The property `x/2*2 != x` when `x` is odd is a basic mathematical fact, not a spec gap. The `shift_is_div` proof correctly accounts for this truncation at each inductive step.

