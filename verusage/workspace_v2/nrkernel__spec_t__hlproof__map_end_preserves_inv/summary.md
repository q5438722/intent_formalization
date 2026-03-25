# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t__hlproof/spec_t__hlproof__map_end_preserves_inv.rs`
**Date:** 2026-03-24T13:20:50Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

Two true positives: the `insert_non_map_preserves_unique` external_body lemma is an unverified trust assumption in the invariant preservation proof, and the `overlap` function incorrectly considers zero-size regions at the same base as overlapping. One false positive: the invariant preservation dependency on external_body is a duplicate of the direct external_body finding.

## True Positives (Spec Issues)

### insert_non_map_preserves_unique_external_body
- **Confidence:** medium
- **Reasoning:** `insert_non_map_preserves_unique` is `external_body` with `unimplemented!()` body — its postcondition is trusted without proof. While the property itself is likely correct (inserting a non-Map shouldn't break Map uniqueness), this is an unverified trust assumption in the proof chain.

### overlap_zero_size_reflexive
- **Confidence:** medium
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via `region1.base == region2.base`. Mathematically, empty intervals do not overlap. This is a minor spec defect in the degenerate case.

## All Candidates

### φ1: insert_non_map_preserves_unique_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `insert_non_map_preserves_unique` is `external_body` with `unimplemented!()` — trusted without proof, any bug in its postcondition would silently propagate
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `insert_non_map_preserves_unique` is `external_body` with `unimplemented!()` body — its postcondition is trusted without proof. While the property itself is likely correct (inserting a non-Map shouldn't break Map uniqueness), this is an unverified trust assumption in the proof chain.

### φ2: map_end_err_no_pmem_overlap_check
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** On MapEnd error, mappings are unchanged but `step_MapEnd` only checks vmem overlap — a map that overlaps existing pmem could still succeed if it doesn't overlap vmem

### φ3: map_end_ok_no_pmem_overlap_guard
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `step_MapEnd` success path only guards on vmem overlap — a mapping with overlapping physical frames can be inserted if virtual addresses don't overlap, breaking `pmem_no_overlap`

### φ4: overlap_zero_size_reflexive
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two zero-size regions at the same base are considered overlapping via `region1.base == region2.base` — empty intervals should not overlap
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via `region1.base == region2.base`. Mathematically, empty intervals do not overlap. This is a minor spec defect in the degenerate case.

### φ5: inv_preserved_relies_on_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The proof that `inv` is preserved through `MapEnd` depends on `insert_non_map_preserves_unique` which is `external_body` — the entire invariant preservation chain rests on an unverified lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a duplicate of φ1 — it merely demonstrates a downstream consequence of the same `external_body` trust assumption. The issue is the unverified `insert_non_map_preserves_unique` lemma itself (already captured by φ1), not the fact that `map_end_preserves_inv` calls it.

