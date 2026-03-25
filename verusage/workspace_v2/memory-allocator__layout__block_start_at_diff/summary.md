# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__block_start_at_diff.rs`
**Date:** 2026-03-24T11:24:52Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The source file defines pure address computation functions (`page_start`, `block_start_at`, `start_offset`) and a basic arithmetic lemma (`block_start_at_diff`). These are low-level helpers that correctly compute addresses without enforcing bounds — bounds enforcement is delegated to higher-level predicates like `is_block_ptr1`. The degenerate cases (zero block size, negative index differences, out-of-range page indices, cross-segment differences) are all arithmetically correct on mathematical integers and don't represent spec gaps.

## All Candidates

### φ1: block_start_at_diff_underflow
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `block_start_at_diff` has no precondition requiring `block_idx2 >= block_idx1` — when `block_idx2 < block_idx1`, the difference `(block_idx2 - block_idx1)` is negative, and the result is still provable but the "diff" semantics are misleading
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The lemma operates on mathematical integers (`int`), where `block_idx2 - block_idx1` being negative is perfectly well-defined. The equation `addr2 = addr1 + (idx2 - idx1) * size` is correct regardless of ordering. This is a general arithmetic identity, not a spec gap.

### φ2: block_start_at_diff_zero_size
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** With `block_size == 0`, all block indices map to the same address — `block_start_at_diff` allows this degenerate case where distinct blocks are at identical addresses
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With `block_size == 0`, the equation `0 * anything == 0` makes all block addresses equal, which is arithmetically correct. The degenerate case is excluded elsewhere — `is_block_ptr1` requires `block_size >= size_of::<Node>()`. This helper lemma doesn't need to enforce that constraint.

### φ3: page_start_exceeds_segment
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `page_start` places no upper bound on `page_id.idx` — idx=1000 yields an offset of ~65MB, far beyond SEGMENT_SIZE (33MB), placing the page outside the segment
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `page_start` is a pure address computation function — it correctly computes `segment_start + SLICE_SIZE * idx` for any `idx`. Bounds checking is the responsibility of callers (e.g., `is_block_ptr1` constrains addresses to within the segment). A helper function not enforcing bounds is normal design.

### φ4: start_offset_small_block_384
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Small blocks (8 ≤ size ≤ 1024) get a 384-byte start offset (3 * MAX_ALIGN_GUARANTEE = 3 * 128) — the first block starts 384 bytes into the page, wasting space; if MAX_ALIGN_GUARANTEE changes, this silently shifts all block addresses
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The 384-byte offset (3 × 128 = 3 × MAX_ALIGN_GUARANTEE) for small blocks is the intended mimalloc design from `_mi_segment_page_start_from_slice`. This ensures alignment guarantees for small allocations. It's the correct and expected behavior.

### φ5: block_start_at_diff_arbitrary_pages
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `block_start_at` allows computing cross-segment address differences with no constraint that blocks are in the same segment — the diff is well-defined on `int` but meaningless across unrelated segments
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is just arithmetic — subtracting two `block_start_at` calls with the same `block_size` and `idx` cancels the `start_offset` and `idx * block_size` terms, leaving `page_start` differences. This is a trivially true algebraic identity with no spec implications.

