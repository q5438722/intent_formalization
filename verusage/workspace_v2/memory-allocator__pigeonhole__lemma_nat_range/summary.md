# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/pigeonhole/pigeonhole__lemma_nat_range.rs`
**Date:** 2026-03-24T11:34:24Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The source file defines a standard half-open natural number range set with a fully verified finiteness/length lemma (no external_body). All tested properties — empty range at equal bounds, lower bound inclusion, upper bound exclusion, singleton length, and subset monotonicity — are correct mathematical consequences of the open spec definition.

## All Candidates

### φ1: empty_range_when_equal
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** When `lo == hi`, the range is empty — correct behavior but verifies the boundary case where the set contains no elements
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty set for `lo == hi` is the correct and expected behavior of a half-open range `[lo, hi)`. This is a basic property of the definition.

### φ2: range_contains_lo
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The range always contains its lower bound when non-empty — a correct property verifying set membership at the boundary
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A half-open range `[lo, hi)` containing `lo` when `lo < hi` is the standard expected behavior. This follows directly from the set comprehension definition.

### φ3: range_excludes_hi
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The range is half-open `[lo, hi)` and excludes `hi` — verifies the upper bound exclusion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Excluding `hi` from `[lo, hi)` is the defining property of a half-open range. This is correct by design.

### φ4: singleton_range_len_1
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A range of width 1 has exactly one element — tests the minimal non-empty case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A range of width 1 having exactly one element is a correct consequence of the verified `lemma_nat_range` which proves `len == hi - lo`.

### φ5: range_subset_monotone
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Narrowing the lower bound produces a subset — correct monotonicity property, but shows the spec allows arbitrary range nesting without additional constraints
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Subset monotonicity when widening the lower bound is a correct set-theoretic property. `[b, c) ⊆ [a, c)` when `a <= b` follows directly from the membership predicate.

