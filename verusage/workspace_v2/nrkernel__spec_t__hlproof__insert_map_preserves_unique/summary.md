# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t__hlproof/spec_t__hlproof__insert_map_preserves_unique.rs`
**Date:** 2026-03-24T13:19:05Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: the `overlap` zero-size reflexivity issue. Two false positives: idle threads correctly produce no pmem overlap, and the uniqueness check's value-based comparison is intentionally complemented by the separate `candidate_mapping_overlaps_inflight_pmem` check that guards physical frame overlap at insertion time.

## True Positives (Spec Issues)

### overlap_zero_size_reflexive
- **Confidence:** medium
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via `region1.base == region2.base`. Mathematically, empty intervals do not overlap. This is a minor spec defect in the degenerate case.

## All Candidates

### φ1: overlap_zero_size_reflexive
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two zero-size regions at the same base are considered overlapping via `region1.base == region2.base` — empty intervals should not overlap
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via `region1.base == region2.base`. Mathematically, empty intervals do not overlap. This is a minor spec defect in the degenerate case.

### φ2: overlap_asymmetric_boundary
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `overlap` is asymmetric — a zero-size region at the exclusive upper bound of a non-empty region is detected as overlapping in one direction but not the reverse

### φ3: if_map_then_unique_ignores_vaddr
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `if_map_then_unique` compares entire `ThreadState` values including vaddr — two maps with different vaddrs but identical PTEs are considered unique, but two maps with same PTE and same vaddr are not, even though the uniqueness check should be about physical frame overlap

### φ4: inflight_pmem_overlap_idle_ignored
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `candidate_mapping_overlaps_inflight_pmem` ignores Idle threads — tests that the match arm returns false for Idle, which is expected but confirms no physical memory reservation for idle threads
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Idle threads have no associated mapping operation, so they correctly contribute no physical memory overlap. The match arm returning false for `Idle` is the intended and correct behavior.

### φ5: uniqueness_not_frame_overlap_based
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Two inflight maps with identical physical frames but different vaddrs pass the uniqueness check because `if_map_then_unique` compares `ThreadState` values (which include vaddr) rather than checking physical frame overlap — allows double-mapping the same physical frame
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `inflight_maps_unique` check is complemented by `candidate_mapping_overlaps_inflight_pmem` which separately checks physical frame overlap. The `insert_map_preserves_unique` lemma requires `!candidate_mapping_overlaps_inflight_pmem(thread_state.values(), pte)` as a precondition, so double-mapping the same physical frame is prevented by the pmem overlap check, not the uniqueness invariant. These are two separate concerns working together.

