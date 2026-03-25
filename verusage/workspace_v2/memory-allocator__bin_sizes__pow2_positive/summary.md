# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__pow2_positive.rs`
**Date:** 2026-03-24T10:31:18Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 0

## All Candidates

### φ1: pow2_negative_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2` returns 1 for all negative exponents — mathematically 2^(-n) is a fraction, so callers passing negative exponents by mistake silently get 1

### φ2: pow2_non_injective_at_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** pow2(0) == pow2(-1) == 1 means the function is non-injective in the non-positive range — distinct exponents produce the same value, which could mask off-by-one errors

### φ3: pow2_all_negative_equal
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** All negative inputs to `pow2` are indistinguishable — the function collapses the entire negative domain to a single value, losing all information about the exponent

### φ4: pow2_not_strictly_monotone
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `pow2` is not strictly increasing for non-positive inputs — `a < b` yet `pow2(a) == pow2(b)`, violating the expected strict monotonicity of exponential functions

### φ5: pow2_zero_not_standard
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2(e) == pow2(0) == 1` for all e ≤ 0, meaning the base case swallows all non-positive inputs rather than just e=0 — a caller computing `pow2(e-1)` when e=1 gets the same result as `pow2(e-2)`, silently hiding errors

