# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__sub_distribute.rs`
**Date:** 2026-03-24T11:32:00Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The source file contains a single fully-verified lemma (`sub_distribute`) proving the distributive law of multiplication over subtraction via `nonlinear_arith`. All tested properties are correct mathematical consequences of this identity — commutativity composition, zero/self boundary cases, and sign derivation. No external_body trust gaps or spec weaknesses exist.

## All Candidates

### φ1: sub_distribute_commuted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Commutativity combined with sub_distribute derives `c*a - c*b == (a-b)*c` — a correct mathematical identity, testing that the lemma composes with basic arithmetic
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct mathematical identity — commutativity of multiplication composed with the distributive law. The lemma is fully verified by `nonlinear_arith`, so this is an expected consequence.

### φ2: sub_distribute_zero_c
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** With `c == 0`, the lemma yields `0 - 0 == 0` — trivially true but a degenerate case where the distributive law provides no useful information
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `a * 0 - b * 0 == 0` is trivially true arithmetic. The lemma correctly handles this degenerate case — no spec gap.

### φ3: sub_distribute_self
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** When `a == b`, the result is `0 == 0 * c == 0` — trivially true, testing the degenerate case where subtraction cancels
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `a * c - a * c == 0` is trivially true. The lemma correctly handles equal arguments — no spec gap.

### φ4: sub_distribute_negative
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The lemma enables deriving sign information — when `a < b` and `c > 0`, `a*c - b*c` is negative, which is correct but shows the lemma's power extends beyond simple factoring
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Deriving sign information from the factored form is a correct mathematical consequence. `(a - b) * c < 0` when `a < b` and `c > 0` is standard arithmetic, not a spec weakness.

### φ5: sub_distribute_factor_extraction
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Setting `a = n+1, b = n` yields `(n+1)*c - n*c == c` — this extracts a single factor, which is correct but shows the lemma can derive stepping relationships between consecutive multiples

