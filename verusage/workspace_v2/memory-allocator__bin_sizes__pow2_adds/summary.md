# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__pow2_adds.rs`
**Date:** 2026-03-24T10:32:32Z
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
- **Why flagged:** All non-positive inputs collapse to 1, making `pow2` non-injective below 1 — could mask off-by-one errors near e=0 boundaries

### φ3: div2_y_overflow_unchecked
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `div2` takes `y: int` with no upper bound — `y * 2` can exceed any machine word, making the lemma applicable to divisions by values unrealizable as machine integers

### φ4: mul_assoc_smuggles_commutativity
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `mul_assoc` ensures `(x*y)*z == y*(x*z)` which with z=1 yields commutativity — may let proofs go through for unintended reasons

### φ5: shift_zero_always_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Right-shifting zero always yields zero — trivially true but tests that `shift_is_div` doesn't produce unsound nonzero results for zero input

