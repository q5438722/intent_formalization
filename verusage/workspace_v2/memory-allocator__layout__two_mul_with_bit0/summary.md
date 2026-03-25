# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__two_mul_with_bit0.rs`
**Date:** 2026-03-24T11:32:46Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The `two_mul_with_bit0` lemma is fully verified (not external_body) using `nonlinear_arith` and simple arithmetic — all tested properties are correct mathematical consequences of Euclidean division. The negative-argument cases, chaining, and the intermediate division-associativity step are all sound.

## All Candidates

### φ1: two_mul_bit0_negative_y
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The lemma accepts negative `y1` — for `y1 = -1`, Verus Euclidean division `x1 / (-1)` may differ from expected truncation division semantics
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The lemma is fully verified (not external_body) using `nonlinear_arith` and simple arithmetic. The property `(2*x)/(2*y) == x/y` holds for all non-zero `y` under Verus's Euclidean division, including negative values. This is a correct mathematical consequence.

### φ2: two_mul_bit0_negative_both
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Both arguments negative — Euclidean division for negative divisor is non-trivial and the lemma applies without restriction
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Same as above — the lemma is fully verified and the property holds for all non-zero `y1` under Euclidean division. Negative arguments are correctly handled by the verified proof.

### φ3: two_mul_bit0_zero_x
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** With `x1 == 0`, both sides are trivially 0 — degenerate case where the scaling property provides no useful information

### φ4: two_mul_bit0_chain
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The lemma can be chained to derive `(2^n * x) / (2^n * y) == x / y` for any power of 2 — this is correct but shows the lemma's power extends to arbitrary even scaling factors
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Chaining the verified lemma twice to derive `(4*x)/(4*y) == x/y` is a correct and expected mathematical consequence. The lemma is fully proved, so composing it is sound.

### φ5: nonlinear_arith_step_unsound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The intermediate step `(2*x)/(2*y) == ((2*x)/2)/y` uses `nonlinear_arith` to assert a division-associativity property — this relies on Verus's Euclidean division semantics and the Z3 nonlinear arithmetic solver's correctness
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The division-associativity step `(2*x)/(2*y) == ((2*x)/2)/y` is verified by Z3's `nonlinear_arith` solver, which is a trusted component of Verus. This is a correct property of Euclidean division when the intermediate divisor (2) is positive, not a soundness gap.

