# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/bin_sizes/bin_sizes__pow2_subtracts.rs`
**Date:** 2026-03-24T10:39:22Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 0

## All Candidates

### φ1: pow2_adds_external_body_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2_adds` is `external_body` — tests that the trusted axiom is consistent at the zero boundary where `pow2(0) = 1` and `1 * 1 = 1 = pow2(0)`

### φ2: pow2_positive_external_body_negative
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `pow2_positive` is `external_body` and trusted for all integers including negative — combined with `pow2(e < 0) == 1` by definition, this is trivially true but the axiom could be used to derive facts about negative-exponent pow2 in contexts where it shouldn't apply

### φ3: pow2_adds_inconsistency_check
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `pow2_adds` is `external_body` with `unimplemented!()` — if its ensures clause were inconsistent with the actual `pow2` definition, it could introduce unsoundness; this tests alignment between the trusted axiom and the recursive definition

### φ4: pow2_negative_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `pow2` returns 1 for all negative exponents — mathematically 2^(-n) is a fraction, so callers passing negative exponents by mistake silently get 1 instead of an error

### φ5: div2_no_upper_bound_on_x
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `div2` takes `x: u64` but the ensures operates on `x as int` — the lemma applies to `u64::MAX` divisions where `y * 2` can exceed u64 range, potentially enabling reasoning about non-realizable machine divisions

