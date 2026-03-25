# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__two_mul_with_bit1.rs`
**Date:** 2026-03-24T11:33:34Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The `two_mul_with_bit1` lemma is fully verified (not external_body) using `nonlinear_arith` and basic arithmetic. All tested properties — negative argument boundary cases, the intermediate division-associativity step, and the odd-number floor division — are correct mathematical consequences of Euclidean division verified by Z3.

## All Candidates

### φ1: two_mul_bit1_negative_y
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The lemma accepts negative `y1` — for `y1 = -1`, `(2*x+1)/(-2)` equals `x/(-1)` under Euclidean division, which may differ from truncation division expectations
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The lemma is fully verified (not external_body) using `nonlinear_arith`. The property holds for all non-zero `y1` under Verus's Euclidean division, including negative values. This is a correct mathematical consequence.

### φ2: two_mul_bit1_negative_x
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** With `x1 = -1`, `(2*(-1)+1) = -1` and `-1 / (2*y1)` should equal `(-1) / y1` — for Euclidean division with positive `y1`, both are -1, but the equality is non-obvious
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The lemma is fully verified and `x1 = -1` is a valid instantiation. `(-1)/(2*y1) == (-1)/y1` holds under Euclidean division — this is a correct consequence of the proved lemma.

### φ3: div_assoc_odd_step
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The intermediate division-associativity step `(2k+1)/(2y) == ((2k+1)/2)/y` is asserted via `nonlinear_arith` — relies on Z3's handling of Euclidean division which could be incorrect for edge cases
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is an intermediate step of the verified proof, asserted via `nonlinear_arith`. Z3's `nonlinear_arith` is a trusted solver component in Verus. The division-associativity property holds for Euclidean division when the intermediate divisor (2) is positive.

### φ4: odd_div_2_floors
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `(2*x+1)/2 == x` under Euclidean division — for negative `x1`, `(2*(-3)+1)/2 = -5/2` under Euclidean division gives `-3`, which equals `x1`; this is correct but the floor-toward-negative-infinity behavior may surprise
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `(2*x+1)/2 == x` is a correct property of Euclidean division for all integers. This is the second assertion in the verified proof body and is trivially true — `2*x+1 = 2*x + 1` with remainder 1.

### φ5: two_mul_bit1_chain_to_4
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Chaining the lemma yields `(4x+1)/(4y) == x/y` — combining two applications, the `+1` remainder propagates through two levels of halving

