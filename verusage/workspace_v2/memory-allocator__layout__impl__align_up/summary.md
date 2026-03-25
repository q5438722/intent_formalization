# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__impl__align_up.rs`
**Date:** 2026-03-24T11:27:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives identify external_body trust gaps: `bitand_with_mask_gives_rounding` trusts the bitwise rounding equivalence for the power-of-two fast path, and `mul_mod_right` trusts a fundamental modular arithmetic property — both could be proved but are instead assumed. Three false positives test correct expected behaviors: alignment of already-aligned values, alignment of zero, and result fitting within `usize` bounds.

## True Positives (Spec Issues)

### bitand_rounding_external_body
- **Confidence:** medium
- **Reasoning:** `bitand_with_mask_gives_rounding` is `external_body` trusting the bitwise-arithmetic equivalence for power-of-two rounding without proof. This is the key lemma enabling the fast path in `align_up`, and any error would make the bitwise branch unsound.

### mul_mod_right_external_body
- **Confidence:** medium
- **Reasoning:** `mul_mod_right` is `external_body` trusting `(a * b) % b == 0` without proof. This fundamental arithmetic property is used to establish `res % y == 0` in `align_up`'s postcondition and should be provable via nonlinear_arith.

## All Candidates

### φ1: bitand_rounding_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `bitand_with_mask_gives_rounding` is `external_body` — the bitwise rounding equivalence for power-of-two alignment is trusted without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `bitand_with_mask_gives_rounding` is `external_body` trusting the bitwise-arithmetic equivalence for power-of-two rounding without proof. This is the key lemma enabling the fast path in `align_up`, and any error would make the bitwise branch unsound.

### φ2: mul_mod_right_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mul_mod_right` is `external_body` — the modular arithmetic property is trusted without proof and used to establish `res % y == 0` in `align_up`
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `mul_mod_right` is `external_body` trusting `(a * b) % b == 0` without proof. This fundamental arithmetic property is used to establish `res % y == 0` in `align_up`'s postcondition and should be provable via nonlinear_arith.

### φ3: align_up_equals_x_when_aligned
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** When `x` is already aligned to `y`, `align_up` returns `x` unchanged — verifying this is expected behavior, not a gap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** When `x` is already a multiple of `y`, `align_up` returning `x` is the correct and expected behavior of ceiling alignment. This is basic arithmetic: `(128 + 63) / 64 * 64 == 128`.

### φ4: align_up_zero_input
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `align_up(0, y)` returns 0 — `(0 + 15) / 16 * 16 == 0`, so aligning zero up gives zero; this is correct but could be surprising if a caller expects a non-zero aligned result
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `align_up(0, 16) == 0` is correct — zero is already aligned to any value. `(0 + 15) / 16 * 16 == 0` is standard ceiling-alignment behavior.

### φ5: align_up_overflow_boundary
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The result is bounded by `x + y - 1 <= usize::MAX`, so the aligned-up value fits in `usize` — but the precondition `x + y - 1 <= usize::MAX` is the caller's burden, and if it's not checked at runtime, `x + mask` could overflow
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The result fitting in `usize` follows directly from the precondition `x + y - 1 <= usize::MAX` and the fact that the aligned result is at most `x + y - 1`. The precondition is appropriately placed — callers must ensure no overflow, which is standard practice.

