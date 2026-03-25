# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/impl_u__os_refinement/impl_u__os_refinement__lemma_inflight_vaddr_implies_hl_unmap_or_map.rs`
**Date:** 2026-03-24T13:12:20Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive: the `overlap` function considers two zero-size regions at the same base as overlapping, which is mathematically incorrect for empty intervals. One false positive: the zero-size candidate mapping overlap is a downstream consequence of the same issue and doesn't arise in practice since page table entries always have positive sizes.

## True Positives (Spec Issues)

### overlap_zero_size_equal_base
- **Confidence:** medium
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via the `region1.base == region2.base` branch. Mathematically, two empty intervals should not overlap — this is a spec weakness where the overlap definition doesn't properly handle the degenerate zero-size case.

## All Candidates

### φ1: ptmem_view_external_body_write_invisible
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `PTMem::view` is `external_body` with no postconditions — writes to page table memory may have no observable effect on the PTE interpretation

### φ2: rl3_interp_external_body_collapses
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `rl3::State::interp` is `external_body` closed spec with no postconditions — any two rl3 states could have identical rl2 interpretations

### φ3: overlap_zero_size_equal_base
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Two zero-size regions at the same base are considered overlapping — `overlap` returns true via the `region1.base == region2.base` branch even when both regions are empty
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `overlap` function returns true for two zero-size regions at the same base via the `region1.base == region2.base` branch. Mathematically, two empty intervals should not overlap — this is a spec weakness where the overlap definition doesn't properly handle the degenerate zero-size case.

### φ4: core_state_idle_vaddr_arbitrary
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `CoreState::Idle.vaddr()` returns `arbitrary()` — the vaddr accessor is callable on Idle state without triggering a recommends failure, returning an unspecified value

### φ5: candidate_mapping_overlaps_zero_size
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Zero-size mappings at the same base are detected as overlapping — `overlap` triggers on `region1.base == region2.base` regardless of size, potentially blocking insertion of empty/sentinel mappings
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct consequence of the same zero-size overlap issue already captured by φ3. In practice, zero-size PTEs don't arise in valid page table configurations — page sizes are always positive (4K, 2M, 1G). The overlap at equal bases is the expected behavior for the `candidate_mapping_overlaps_existing_vmem` check.

