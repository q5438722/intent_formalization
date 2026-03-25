# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__os_refinement/impl_u__os_refinement__lemma_map_soundness_equality.rs`
**Date:** 2026-03-24T13:13:12Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

Two true positives: `step_Map_sound` omits the `candidate_mapping_overlaps_existing_vmem` check allowing virtual address aliasing, and `overlap` incorrectly considers zero-size regions at the same base as overlapping. One false positive: the pmem overlap symmetry test is just standard existential introduction.

## True Positives (Spec Issues)

### step_map_sound_no_existing_vmem_check
- **Confidence:** medium
- **Reasoning:** `step_Map_sound` checks inflight vmem overlap, existing pmem overlap, and inflight pmem overlap — but notably omits `candidate_mapping_overlaps_existing_vmem`. This means a map operation can proceed even when the new mapping's virtual address range overlaps an already-installed virtual memory mapping, potentially creating aliased virtual addresses.

### overlap_zero_size_equal_base
- **Confidence:** medium
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via the `region1.base == region2.base` branch. Mathematically, two empty intervals should not overlap — this is a spec weakness in the degenerate zero-size case.

## All Candidates

### φ1: ptmem_view_external_body_write_invisible
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `PTMem::view` is `external_body` with no postconditions — writes to page table memory may have no observable effect on the PTE interpretation

### φ2: rl3_interp_external_body_collapses
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `rl3::State::interp` is `external_body` closed spec with no postconditions — any two rl3 states could have identical rl2 interpretations

### φ3: step_map_sound_no_existing_vmem_check
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `step_Map_sound` checks inflight vmem overlap and existing/inflight pmem overlap but does NOT check `candidate_mapping_overlaps_existing_vmem` — a new mapping can overlap an existing virtual memory mapping
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `step_Map_sound` checks inflight vmem overlap, existing pmem overlap, and inflight pmem overlap — but notably omits `candidate_mapping_overlaps_existing_vmem`. This means a map operation can proceed even when the new mapping's virtual address range overlaps an already-installed virtual memory mapping, potentially creating aliased virtual addresses.

### φ4: overlap_zero_size_equal_base
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two zero-size regions at the same base are considered overlapping — `overlap` returns true via the `region1.base == region2.base` branch even when both regions are empty
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via the `region1.base == region2.base` branch. Mathematically, two empty intervals should not overlap — this is a spec weakness in the degenerate zero-size case.

### φ5: candidate_pmem_overlap_symmetric
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Tests that the existential witness in `candidate_mapping_overlaps_existing_pmem` is correctly triggered — a concrete overlapping base should satisfy the exists clause
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is correct by design — providing a concrete witness `b` that satisfies `mappings.contains_key(b) && overlap(pte.frame, mappings[b].frame)` trivially satisfies the existential in `candidate_mapping_overlaps_existing_pmem`. This is standard existential introduction, not a spec gap.

