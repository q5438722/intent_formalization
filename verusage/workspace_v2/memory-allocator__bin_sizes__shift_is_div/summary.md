# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__shift_is_div.rs`
**Date:** 2026-03-24T10:54:00Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

Two true positives identify the same underlying issue from different angles: `pow2` collapses all non-positive inputs to 1, which is mathematically non-standard and creates a non-injective, non-monotone flat region. The `mul_assoc` naming concern is a false positive — the property is arithmetically correct and used appropriately despite the slightly misleading name.

## True Positives (Spec Issues)

### pow2_negative_is_one
- **Confidence:** medium
- **Reasoning:** `pow2` returning 1 for all negative exponents is mathematically non-standard (2^(-n) should be fractional). While the function is only used with non-negative arguments in this file, the definition silently masks errors if ever called with negative values.

### pow2_strict_monotone_gap
- **Confidence:** medium
- **Reasoning:** `pow2(0) == pow2(-1) == pow2(-100) == 1` means pow2 is not injective for non-positive inputs. The monotonicity property in `pow2_properties` carefully restricts to `0 <= e1 < e2`, but callers unaware of this could incorrectly assume pow2 is strictly monotone everywhere, leading to unsound reasoning near the non-positive boundary.

## All Candidates

### φ1: pow2_negative_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2` returns 1 for all negative exponents — mathematically 2^(-n) is a fraction, so callers accidentally passing negative values get silently wrong results
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `pow2` returning 1 for all negative exponents is mathematically non-standard (2^(-n) should be fractional). While the function is only used with non-negative arguments in this file, the definition silently masks errors if ever called with negative values.

### φ2: mul_assoc_smuggles_commutativity
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mul_assoc` proves `(x*y)*z == y*(x*z)` which is not pure associativity but a combination of associativity and commutativity — the name is misleading and may cause incorrect reasoning when callers expect standard `(x*y)*z == x*(y*z)`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The lemma `(x*y)*z == y*(x*z)` is a true arithmetic identity and is used correctly within this file (specifically in `pow2_adds` to rewrite `(pow2(e-1) * 2) * pow2(e2)` into `2 * (pow2(e-1) * pow2(e2))`). The name is slightly misleading but the property itself is mathematically correct and the spec is not unsound.

### φ3: shift_zero_by_63_is_zero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Shifting 0 by any amount gives 0 — trivially correct but tests that `shift_is_div` handles the maximum valid shift with a zero input without issues

### φ4: pow2_strict_monotone_gap
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `pow2` is NOT strictly monotone at the boundary: `pow2(0) == pow2(-1) == pow2(-100) == 1` — the strict monotonicity property in `pow2_properties` only covers `0 <= e1 < e2`, leaving a flat region for all non-positive inputs where different exponents map to the same value
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `pow2(0) == pow2(-1) == pow2(-100) == 1` means pow2 is not injective for non-positive inputs. The monotonicity property in `pow2_properties` carefully restricts to `0 <= e1 < e2`, but callers unaware of this could incorrectly assume pow2 is strictly monotone everywhere, leading to unsound reasoning near the non-positive boundary.

### φ5: div2_overflow_boundary
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `div2` takes `x: u64` but ensures a property about `x as int` — at u64::MAX boundary, the division chain works in int domain but the lemma's signature suggests it handles u64 overflow when it actually sidesteps it via int coercion

