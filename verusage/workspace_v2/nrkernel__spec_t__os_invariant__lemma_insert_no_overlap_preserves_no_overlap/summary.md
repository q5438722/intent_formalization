# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_os_invariant/spec_t__os_invariant__lemma_insert_no_overlap_preserves_no_overlap.rs`
**Date:** 2026-03-24T14:44:57Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. Zero-size regions for failed unmaps, vacuous truth of no-overlap for all-Idle states, and uniqueness skipping Idle cores are all intentional design choices that correctly handle the base/initialization case and error paths.

## All Candidates

### φ1: inflight_vmem_region_size_zero_on_failed_unmap
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A failed unmap (Err result) produces a zero-size inflight region — this means failed unmaps never block other operations via overlap, even though they're still in a non-Idle state occupying the core
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `inflight_vmem_region` for `UnmapOpDone { result: Err(()) }` computes `size = if result is Ok { ... } else { 0 }`, yielding 0. This is intentional — a failed unmap didn't find a mapping, so there's no region to protect. Correct by design.

### φ2: pte_size_unmap_waiting_depends_on_pt
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `UnmapWaiting` and early `UnmapExecuting` compute pte_size from the current pt map — if pt changes between when the unmap started and when size is queried, the inflight region size changes, potentially allowing overlapping operations to slip through

### φ3: no_overlap_vmem_allows_idle_duplicates
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `no_overlap_vmem_values` vacuously holds when all cores are Idle — tests that the invariant doesn't accidentally require something of Idle states; if it did, initial states couldn't satisfy it
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `no_overlap_vmem_values` quantifies over non-Idle states only (`!state1.is_idle() && !state2.is_idle()`). When all cores are Idle, the antecedent is never satisfied, making the invariant vacuously true. This is correct — the invariant should hold trivially at initialization.

### φ4: unique_corestates_ignores_idle
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `unique_CoreStates` skips Idle states — two cores can both be Idle without violating uniqueness; tests that the `!map[core].is_idle()` guard works correctly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `unique_CoreStates` guards with `!map[core].is_idle()`, so Idle cores skip the uniqueness check entirely. Multiple cores being Idle is expected and correct — uniqueness only matters for active operations.

### φ5: candidate_overlaps_inflight_zero_size_unmap_err
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A failed UnmapOpDone has zero-size inflight region — it never blocks any candidate mapping at a different base, effectively making the core invisible to overlap checks while still being non-Idle and occupying the core state

