# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__impl__align_down.rs`
**Date:** 2026-03-24T11:26:42Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Three true positives identify external_body trust gaps: `bitand_with_mask_gives_rounding` trusts the bitwise rounding equivalence for the fast power-of-two path, `mul_mod_right` trusts a fundamental modular arithmetic property used to establish the alignment postcondition, and `mul_mod_right`'s acceptance of negative divisors is an over-broad trust assumption beyond what `align_down` needs. One false positive: alignment to 1 returning `x` is correct expected behavior.

## True Positives (Spec Issues)

### bitand_rounding_external_body
- **Confidence:** medium
- **Reasoning:** `bitand_with_mask_gives_rounding` is `external_body` trusting the bitwise-arithmetic equivalence `x & !sub(y,1) == (x/y)*y` for power-of-two `y` without proof. This is the key lemma enabling the fast power-of-two path in `align_down`, and any error would make the bitwise branch unsound.

### mul_mod_right_external_body
- **Confidence:** medium
- **Reasoning:** `mul_mod_right` is `external_body` trusting `(a * b) % b == 0` without proof. While mathematically correct, this is a fundamental arithmetic property that should be provable via nonlinear_arith, and `align_down` depends on it to establish the `res % y == 0` postcondition.

### mul_mod_right_negative_divisor
- **Confidence:** medium
- **Reasoning:** `mul_mod_right` only requires `b != 0`, allowing negative divisors. While `align_down` only calls it with positive `y`, the axiom's over-broad precondition trusts the property for all negative `b` without verification — for Verus Euclidean division this happens to be correct, but it's an unnecessarily broad trust assumption.

## All Candidates

### φ1: bitand_rounding_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `bitand_with_mask_gives_rounding` is `external_body` — the bitwise-arithmetic equivalence for power-of-two rounding is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `bitand_with_mask_gives_rounding` is `external_body` trusting the bitwise-arithmetic equivalence `x & !sub(y,1) == (x/y)*y` for power-of-two `y` without proof. This is the key lemma enabling the fast power-of-two path in `align_down`, and any error would make the bitwise branch unsound.

### φ2: mul_mod_right_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mul_mod_right` is `external_body` — the property `(a * b) % b == 0` is trusted without proof; for negative `b`, Verus Euclidean division semantics must be considered
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `mul_mod_right` is `external_body` trusting `(a * b) % b == 0` without proof. While mathematically correct, this is a fundamental arithmetic property that should be provable via nonlinear_arith, and `align_down` depends on it to establish the `res % y == 0` postcondition.

### φ3: align_down_non_power_of_two
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `align_down` works for non-power-of-two `y` via the division path — despite the function name suggesting alignment, it's really just floor-rounding to any multiple, which may be surprising to callers expecting power-of-two alignment only

### φ4: align_down_y_equals_1
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** With `y == 1`, `align_down` always returns `x` unchanged — alignment to 1 is a no-op, and the power-of-two branch (`1 & 0 == 0`) uses `x & !0 == x`, which is correct but degenerate
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Aligning down to 1 returning `x` unchanged is correct and expected — every integer is a multiple of 1. This is a valid boundary case, not a spec gap.

### φ5: mul_mod_right_negative_divisor
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `mul_mod_right` accepts negative `b` — the axiom is trusted for all non-zero divisors including negatives, where Euclidean vs truncation division semantics could differ; over-broad trust assumption
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `mul_mod_right` only requires `b != 0`, allowing negative divisors. While `align_down` only calls it with positive `y`, the axiom's over-broad precondition trusts the property for all negative `b` without verification — for Verus Euclidean division this happens to be correct, but it's an unnecessarily broad trust assumption.

