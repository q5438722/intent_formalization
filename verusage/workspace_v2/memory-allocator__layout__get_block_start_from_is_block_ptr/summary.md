# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__get_block_start_from_is_block_ptr.rs`
**Date:** 2026-03-24T11:25:49Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `slice_idx` is not constrained to be `>= page_id.idx`, allowing physically impossible configurations where a block claims to be in a slice that precedes its own page. Two false positives: the non-uniqueness of `BlockId` for a pointer is expected design (uniqueness is an allocator-level invariant), and the `page_id.idx` unboundedness concern is mitigated by the combined address range constraints in `is_block_ptr1`.

## True Positives (Spec Issues)

### slice_idx_below_page_idx
- **Confidence:** medium
- **Reasoning:** `is_block_ptr1` constrains `0 <= slice_idx <= SLICES_PER_SEGMENT` and checks the block is within the slice's address range, but never requires `slice_idx >= page_id.idx`. A block on page 10 in slice 2 is physically impossible — a page's slices must start at or after the page's own index. This is a missing constraint.

## All Candidates

### φ1: slice_idx_below_page_idx
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `is_block_ptr1` allows `slice_idx < page_id.idx` — a block on page 10 claiming to be in slice 2 is physically impossible since a page's slices start at the page index
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `is_block_ptr1` constrains `0 <= slice_idx <= SLICES_PER_SEGMENT` and checks the block is within the slice's address range, but never requires `slice_idx >= page_id.idx`. A block on page 10 in slice 2 is physically impossible — a page's slices must start at or after the page's own index. This is a missing constraint.

### φ2: block_end_past_slice
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The spec checks that the block start is within a slice but never checks `ptr + block_size` fits — large blocks can extend past the slice boundary

### φ3: two_block_ids_same_ptr
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Two distinct `BlockId`s with different `block_size`/`idx` can yield the same `block_start` address — the spec doesn't ensure `is_block_ptr` uniquely determines the `BlockId` from a pointer
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The property just shows `block_start(b1) == block_start(b2)` when both map to the same pointer, which is a trivial arithmetic consequence. Non-uniqueness of `BlockId` for a given pointer is expected — the spec is a predicate relating pointers to block IDs, not an injective mapping. Uniqueness would be established by higher-level invariants about the allocator state.

### φ4: page_idx_exceeds_slices_per_segment
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `is_block_ptr1` bounds `slice_idx <= SLICES_PER_SEGMENT` but places no upper bound on `page_id.idx` — a page with `idx > 512` would compute `page_start` beyond the segment
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures only states `slice_idx <= SLICES_PER_SEGMENT`, which is directly from `is_block_ptr1`. While `page_id.idx` is indeed unbounded in this predicate, the combination of `ptr == block_start(block_id)` and the segment range constraint `segment_start < ptr <= segment_start + SEGMENT_SIZE` indirectly bounds `page_id.idx` for any satisfiable `is_block_ptr`. The φ itself is trivially true and doesn't demonstrate a real gap.

### φ5: block_size_no_upper_bound
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `is_block_ptr1` requires `block_size >= size_of::<Node>()` (lower bound) but has no upper bound — a block_size exceeding SEGMENT_SIZE is allowed, which combined with the slice containment check means block_idx must be 0 but the block extends far past the segment

