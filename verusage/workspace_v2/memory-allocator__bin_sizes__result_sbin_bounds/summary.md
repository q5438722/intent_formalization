# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__result_sbin_bounds.rs`
**Date:** 2026-03-24T10:51:02Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `pow2` returning 1 for negative exponents is a non-standard design choice that could mask bugs if called with unexpected arguments. The other two are false positives — one is a trivial tautology that proves nothing about the spec, and the other is standard vacuous truth for empty ranges in a recursive range-checker.

## True Positives (Spec Issues)

### pow2_negative_is_one
- **Confidence:** medium
- **Reasoning:** `pow2` returning 1 for all negative exponents is mathematically non-standard (2^(-n) should be fractional). While `pow2` is only used with non-negative arguments in this file's core logic, the definition silently masks errors if ever called with negative values.

## All Candidates

### φ1: sbin_bounds_negative_size_unchecked
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `property_sbin_bounds` casts `size as u64` without a non-negativity or range check — for size=0, `u64_leading_zeros(0)` is 64, making `b = 63 - 64` underflow as u8, so the property may fail or produce nonsensical results at boundary
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `P || !P` is a tautology that holds for any proposition. This proves nothing about the spec — it doesn't expose any issue with the boundary behavior of `property_sbin_bounds` at size=0.

### φ2: pow2_negative_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2` returns 1 for all negative exponents — mathematically 2^(-n) is a fraction, so callers accidentally passing negative values get silently wrong results
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `pow2` returning 1 for all negative exponents is mathematically non-standard (2^(-n) should be fractional). While `pow2` is only used with non-negative arguments in this file's core logic, the definition silently masks errors if ever called with negative values.

### φ3: sbin_bounds_size_8_fails
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For small sizes like 4, `u64_leading_zeros(4) == 61`, so `b = 63 - 61 = 2`, and `(2 << 2) | ((4 >> 0) & 0x03) = 8 | 0 = 8 >= 4` should hold — but if it doesn't, the property excludes valid small sizes from the sbin computation

### φ4: check_sbin_bounds_vacuous_empty_range
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `check_sbin_bounds` is vacuously true when `size_start >= size_end` — callers who accidentally swap arguments get a trivially true result, masking missing coverage
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Vacuous truth for empty ranges (`size_start >= size_end`) is standard and correct for range-checking predicates — the universal quantification `forall |size| start <= size < end` is also vacuously true for empty ranges. This is consistent, expected base-case behavior for the recursive checker.

### φ5: shift_is_div_zero_identity
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Right-shifting by 0 is trivially the identity — but `shift_is_div` is used to justify bit-manipulation in `property_sbin_bounds`; if the shift amount in the sbin formula can be 0 (when `b == 2`), the shift `b - 2 = 0` extracts no useful bits, potentially misclassifying sizes into wrong bins

