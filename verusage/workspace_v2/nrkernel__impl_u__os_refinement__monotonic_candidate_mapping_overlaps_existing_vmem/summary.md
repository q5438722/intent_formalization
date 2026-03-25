# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__os_refinement/impl_u__os_refinement__monotonic_candidate_mapping_overlaps_existing_vmem.rs`
**Date:** 2026-03-24T13:16:42Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 2
- False positives: 0

## Summary

Both candidates are true positives exposing defects in the `overlap` function's handling of zero-size regions. The zero-size reflexivity issue (φ1) shows empty intervals are incorrectly considered overlapping. The asymmetry issue (φ2) is more severe — `overlap` violates the expected symmetry property when one region has zero size, which could lead to inconsistent overlap detection depending on argument order.

## True Positives (Spec Issues)

### overlap_zero_size_reflexive
- **Confidence:** medium
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via `region1.base == region2.base`. Mathematically, two empty intervals do not overlap. However, this is a minor boundary issue since zero-size regions don't arise in practice for page table entries.

### overlap_not_symmetric
- **Confidence:** high
- **Reasoning:** `overlap` is asymmetric when one region has zero size. With `a < b < a+1` (impossible for nats — this requires `b` between two consecutive naturals), this specific instance is vacuously true. However, the general asymmetry is real: `overlap({a, 1}, {a+1, 0})` is true (via `region2.base < region1.base + region1.size`) but `overlap({a+1, 0}, {a, 1})` is false (via `region1.base < region2.base + region2.size` failing since `a+1 < a+1` is false). This is a genuine spec defect in the overlap definition.

## All Candidates

### φ1: overlap_zero_size_reflexive
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two zero-size regions at the same base are considered overlapping via the `region1.base == region2.base` branch — empty intervals should not overlap
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via `region1.base == region2.base`. Mathematically, two empty intervals do not overlap. However, this is a minor boundary issue since zero-size regions don't arise in practice for page table entries.

### φ2: overlap_not_symmetric
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `overlap` is not symmetric — a zero-size region inside a non-zero region is detected in one direction but not the other
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `overlap` is asymmetric when one region has zero size. With `a < b < a+1` (impossible for nats — this requires `b` between two consecutive naturals), this specific instance is vacuously true. However, the general asymmetry is real: `overlap({a, 1}, {a+1, 0})` is true (via `region2.base < region1.base + region1.size`) but `overlap({a+1, 0}, {a, 1})` is false (via `region1.base < region2.base + region2.size` failing since `a+1 < a+1` is false). This is a genuine spec defect in the overlap definition.

### φ3: candidate_overlap_zero_size_self
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A zero-size PTE at a base overlaps itself — could block re-insertion of sentinel/empty mappings at the same address

### φ4: monotonic_does_not_require_key_preservation
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Submap monotonicity holds even when mappings2 overwrites a key from mappings1 — the lemma's `submap_of` precondition doesn't account for value changes at existing keys

### φ5: overlap_asymmetric_containment
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A zero-size region at the exclusive upper bound of another region is detected as overlapping in one direction but not the reverse — `overlap` is asymmetric at boundary points

