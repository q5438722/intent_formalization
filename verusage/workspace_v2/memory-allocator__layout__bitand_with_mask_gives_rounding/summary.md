# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__bitand_with_mask_gives_rounding.rs`
**Date:** 2026-03-24T11:19:29Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Two core true positives: `two_mul_with_bit0` and `two_mul_with_bit1` are both `external_body` lemmas that the main `bitand_with_mask_gives_rounding` theorem depends on — they trust integer division scaling properties without proof. Two additional true positives highlight that these axioms accept negative arguments (never needed by the main theorem on `usize` values), making the trusted assumptions broader than necessary. One false positive: the main theorem itself is the intended result and is correct modulo the two trusted helpers.

## True Positives (Spec Issues)

### two_mul_with_bit0_external_body
- **Confidence:** medium
- **Reasoning:** `two_mul_with_bit0` is `external_body` trusting `(2*x)/(2*y) == x/y` without proof. While mathematically correct for Euclidean division (scaling numerator and denominator by the same positive factor preserves the quotient, and for negative factor the signs cancel), this is an unverified trust gap that the main theorem depends on.

### two_mul_with_bit1_external_body
- **Confidence:** medium
- **Reasoning:** `two_mul_with_bit1` is `external_body` trusting `(2*x+1)/(2*y) == x/y` without proof. This is the more subtle of the two lemmas — the odd remainder `+1` is absorbed because `2*y` divides evenly, but this is unverified and any error would propagate to the main theorem.

### two_mul_bit1_negative_y
- **Confidence:** medium
- **Reasoning:** The external_body lemma only requires `y1 != 0` but is instantiated with `y1 = -1`. For Verus Euclidean division, `(2*x+1)/(-2)` vs `x/(-1)` — this is a concrete negative case where the trusted axiom applies without restriction, and the Euclidean division semantics for negative divisors are non-trivial.

### two_mul_bit0_negative_both
- **Confidence:** medium
- **Reasoning:** The external_body axiom applies to all `y1 < 0, x1 < 0` without restriction. While `(2*x)/(2*y) == x/y` does hold for Euclidean division when scaling by a common factor, this negative-argument regime is never used by the main theorem (which operates on `usize` values) and represents an over-broad trust assumption.

## All Candidates

### φ1: two_mul_with_bit0_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `two_mul_with_bit0` is `external_body` — the integer division scaling property `(2*x)/(2*y) == x/y` is trusted without proof; for negative values, Verus integer division semantics (Euclidean vs truncation) could make this subtly wrong
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `two_mul_with_bit0` is `external_body` trusting `(2*x)/(2*y) == x/y` without proof. While mathematically correct for Euclidean division (scaling numerator and denominator by the same positive factor preserves the quotient, and for negative factor the signs cancel), this is an unverified trust gap that the main theorem depends on.

### φ2: two_mul_with_bit1_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `two_mul_with_bit1` is `external_body` — the odd-numerator division property `(2*x+1)/(2*y) == x/y` is trusted without proof; this is the key lemma enabling the main theorem and any error here propagates to `bitand_with_mask_gives_rounding`
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `two_mul_with_bit1` is `external_body` trusting `(2*x+1)/(2*y) == x/y` without proof. This is the more subtle of the two lemmas — the odd remainder `+1` is absorbed because `2*y` divides evenly, but this is unverified and any error would propagate to the main theorem.

### φ3: power_of_two_rounddown
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The main theorem asserts bitwise AND with mask equals arithmetic round-down for all power-of-two `y` — this depends on both external_body lemmas being correct; if either is wrong, the bitwise-arithmetic equivalence is unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the intended main theorem — bitwise AND with a power-of-two mask equals arithmetic round-down. The proof is mostly verified with bit_vector assertions; only the two external_body helper lemmas are trusted. The property itself is correct and desirable.

### φ4: two_mul_bit1_negative_y
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `two_mul_with_bit1` has no constraint that `y1 > 0` — for negative `y1`, Verus Euclidean division semantics differ from truncation division, and the property may not hold
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The external_body lemma only requires `y1 != 0` but is instantiated with `y1 = -1`. For Verus Euclidean division, `(2*x+1)/(-2)` vs `x/(-1)` — this is a concrete negative case where the trusted axiom applies without restriction, and the Euclidean division semantics for negative divisors are non-trivial.

### φ5: two_mul_bit0_negative_both
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `two_mul_with_bit0` is called with negative arguments — Verus uses Euclidean division where `(-4)/(-2)` may differ from truncation division; the axiom is trusted for all non-zero `y1` including negatives
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The external_body axiom applies to all `y1 < 0, x1 < 0` without restriction. While `(2*x)/(2*y) == x/y` does hold for Euclidean division when scaling by a common factor, this negative-argument regime is never used by the main theorem (which operates on `usize` values) and represents an over-broad trust assumption.

