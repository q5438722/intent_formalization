# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/delegation_map_v/delegation_map_v__impl3__mind_the_gap.rs`
**Date:** 2026-03-24T08:33:15Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. The irreflexivity of `lt_spec` follows correctly from the `cmp_properties` axioms. The vacuous gap transitivity through `end` and `end` being `geq` everything are both expected consequences of `end` serving as a maximal sentinel in the iterator ordering — standard design for past-the-end iterators in ordered collections.

## All Candidates

### φ1: gap_reflexive_on_end
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `gap(end, end)` is vacuously true since `end.lt_spec(ki)` requires `end.k.is_Some()` which is always false — this means any endpoint-bounded gap trivially holds regardless of map contents

### φ2: lt_spec_not_reflexive
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lt_spec` should be irreflexive for non-end iterators — if this verifies, it confirms `cmp_spec` irreflexivity, but if it fails, `gap` reasoning would be unsound since a key could be both a lower and upper bound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Irreflexivity of `lt_spec` for non-end iterators follows correctly from `cmp_properties` which ensures `a.cmp_spec(a).eq()` (reflexivity of equality), meaning `a.cmp_spec(a).lt()` is false. This is a desirable and expected property of a strict ordering.

### φ3: mind_the_gap_transitive_vacuous
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Transitivity of `gap` through `end` iterators — `gap(end, z)` is vacuously true for any `z`, so `gap(w, z)` would follow from `gap(w, end)` alone, making the second premise meaningless and the transitivity rule weaker than expected
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `gap(end, z)` is vacuously true because `end.lt_spec(ki)` requires `end.k.is_Some()` which is false — so no `ki` falls in the range. The transitivity result `gap(w, z)` follows from `gap(w, end)` alone, which is correct: if no keys exist above `w`, then no keys exist in any sub-interval. This is expected behavior of a gap predicate with a maximal sentinel.

### φ4: gap_narrowing_allows_trivial
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `gap(x, x)` is always vacuously true (no `ki` with `x < ki < x`), so the narrowing property holds trivially — callers using `gap` narrowing might incorrectly believe they've established a meaningful gap when they've only produced a vacuous one

### φ5: geq_spec_end_geq_everything
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `end` is `geq` every iterator since `end.lt_spec(ki)` is always false (requires `end.k.is_Some()`) — this means `gap` narrowing with `end` as a bound in `geq_spec` always succeeds, potentially hiding gaps that should not narrow to endpoint boundaries
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `end` representing the maximum sentinel (past-the-end iterator) should indeed be `geq` every other iterator. This is the standard design for a sentinel value in an ordered collection — `lt_spec` is defined so nothing is less than `end` from `end`'s perspective, making `geq_spec` (defined as `!lt_spec`) always true. This is correct and expected.

