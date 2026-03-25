# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t__hlproof/spec_t__hlproof__map_start_preserves_inv.rs`
**Date:** 2026-03-24T13:21:38Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: the `overlap` zero-size reflexivity issue. Two false positives: `step_Map_sound` intentionally omits existing vmem overlap (handled separately as an error path in `step_MapStart/MapEnd`), and `step_Map_enabled` correctly prevents zero-size frames through its page size constraints.

## True Positives (Spec Issues)

### overlap_zero_size_reflexive
- **Confidence:** medium
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via `region1.base == region2.base`. Mathematically, empty intervals do not overlap. This is a minor spec defect in the degenerate case, though it doesn't arise in practice since `step_Map_enabled` constrains sizes to valid page sizes.

## All Candidates

### φ1: step_map_sound_no_existing_vmem_check
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `step_Map_sound` checks inflight vmem, existing pmem, and inflight pmem overlap — but does NOT check `candidate_mapping_overlaps_existing_vmem`, allowing a sound map that overlaps existing virtual memory
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional by design. `step_Map_sound` handles physical memory and inflight overlap for soundness. Existing vmem overlap is handled separately in `step_MapStart` — when `candidate_mapping_overlaps_existing_vmem` is true, the map still proceeds but returns an error at `step_MapEnd`. The vmem overlap check is a correctness concern (returns Err), not a soundness concern.

### φ2: max_phyaddr_width_weaker_than_constant
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `MAX_PHYADDR_WIDTH` is `external_body` set to 52 but the axiom only constrains it to `[32, 52]` — the SMT solver can assume any value in that range

### φ3: overlap_zero_size_reflexive
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two zero-size regions at the same base are considered overlapping via `region1.base == region2.base` — empty intervals should not overlap
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via `region1.base == region2.base`. Mathematically, empty intervals do not overlap. This is a minor spec defect in the degenerate case, though it doesn't arise in practice since `step_Map_enabled` constrains sizes to valid page sizes.

### φ4: step_map_enabled_no_size_zero_guard
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `step_Map_enabled` constrains `pte.frame.size` to be one of `L3_ENTRY_SIZE`, `L2_ENTRY_SIZE`, or `L1_ENTRY_SIZE` — tests whether this implicitly prevents zero-size frames
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and desirable property. `step_Map_enabled` constrains `pte.frame.size` to one of `L3_ENTRY_SIZE` (4096), `L2_ENTRY_SIZE` (2M), or `L1_ENTRY_SIZE` (1G) — all strictly positive. The implicit zero-size prevention is correct by construction.

### φ5: step_mapstart_unsound_allows_arbitrary_state
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** When `step_Map_sound` fails, the system transitions to an unsound state — the entire state becomes untrusted rather than simply rejecting the operation, which is a permissive failure mode

