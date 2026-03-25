# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__block_size_ge_word.rs`
**Date:** 2026-03-24T11:18:30Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives identify real spec weaknesses: the block end address is never checked against slice/segment boundaries (only the start is), `slice_idx` has no relationship constraint to `page_id.idx` allowing physically impossible slice assignments, and `page_id.idx` is unbounded allowing page starts beyond the segment end. Two false positives: the block-within-segment property is exactly what the spec intends, and the large segment ID concern is already mitigated by `is_block_ptr1`'s `< usize::MAX` check.

## True Positives (Spec Issues)

### block_end_exceeds_slice
- **Confidence:** medium
- **Reasoning:** The ensures is just `true` so it trivially holds, but the observation is valid: `is_block_ptr1` only constrains the block's start address within the slice range, not `ptr + block_size`. A block near the end of a slice could extend past the slice boundary without violating the spec. This is a real spec weakness — the block containment invariant is incomplete.

### slice_idx_not_linked_to_page_idx
- **Confidence:** medium
- **Reasoning:** `is_block_ptr1` constrains `slice_idx` to `[0, SLICES_PER_SEGMENT]` and checks the block is within the slice, but never requires `slice_idx >= page_id.idx`. A block claiming to be on page 5 could have `slice_idx == 0`, which is physically impossible since a page's slices must start at or after the page index. This is a missing constraint.

### page_idx_unbounded
- **Confidence:** medium
- **Reasoning:** `page_start` computes `segment_start + SLICE_SIZE * idx` with no bound on `idx`. While `is_block_ptr1` constrains `slice_idx <= SLICES_PER_SEGMENT`, it never bounds `page_id.idx`. A page with `idx=1000` would place the page start beyond the segment, and the spec doesn't prevent this directly — the indirect constraint via `ptr == block_start` and the segment range check may catch some cases but doesn't systematically enforce `page_id.idx < SLICES_PER_SEGMENT`.

## All Candidates

### φ1: block_ptr_within_segment
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Block pointers are constrained to within the segment — but the spec only checks the start address of the block, not that the entire block (start + block_size) fits within the segment or slice
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is exactly what `is_block_ptr1` specifies — the block start pointer must be within (segment_start, segment_start + SEGMENT_SIZE]. The concern about block end not being checked is a separate issue, but this property itself is correct and expected.

### φ2: block_end_exceeds_slice
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The spec constrains the block's start address to be within a slice but never checks that `ptr + block_size` fits within the slice — a large block_size could extend past the slice boundary
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The ensures is just `true` so it trivially holds, but the observation is valid: `is_block_ptr1` only constrains the block's start address within the slice range, not `ptr + block_size`. A block near the end of a slice could extend past the slice boundary without violating the spec. This is a real spec weakness — the block containment invariant is incomplete.

### φ3: slice_idx_not_linked_to_page_idx
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `slice_idx` is only bounded `0 <= slice_idx <= SLICES_PER_SEGMENT` but has no constraint relating it to `page_id.idx` — a block could claim to be in slice 0 while belonging to page 5
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `is_block_ptr1` constrains `slice_idx` to `[0, SLICES_PER_SEGMENT]` and checks the block is within the slice, but never requires `slice_idx >= page_id.idx`. A block claiming to be on page 5 could have `slice_idx == 0`, which is physically impossible since a page's slices must start at or after the page index. This is a missing constraint.

### φ4: segment_id_nat_allows_overflow
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `segment_id.id` is `nat` with no upper bound — `segment_start` computes `id * SEGMENT_SIZE` which for large IDs yields addresses far exceeding `usize::MAX`, though `is_block_ptr1` does check `< usize::MAX`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** While `segment_id.id` is unbounded `nat`, this φ only tests `block_start` computation which is pure math on `int` (no overflow). The actual concern about exceeding `usize::MAX` is already handled by `is_block_ptr1`'s constraint `segment_start(...) + SEGMENT_SIZE < usize::MAX`, which would reject such large IDs.

### φ5: page_idx_unbounded
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `page_id.idx` is `nat` with no upper bound in `page_start` — idx=1000 exceeds `SLICES_PER_SEGMENT=512`, placing the page start beyond the segment end; `is_block_ptr1` constrains `slice_idx` but not `page_id.idx`
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `page_start` computes `segment_start + SLICE_SIZE * idx` with no bound on `idx`. While `is_block_ptr1` constrains `slice_idx <= SLICES_PER_SEGMENT`, it never bounds `page_id.idx`. A page with `idx=1000` would place the page start beyond the segment, and the spec doesn't prevent this directly — the indirect constraint via `ptr == block_start` and the segment range check may catch some cases but doesn't systematically enforce `page_id.idx < SLICES_PER_SEGMENT`.

