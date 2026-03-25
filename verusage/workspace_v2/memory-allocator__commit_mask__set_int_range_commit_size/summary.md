# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/commit_mask/commit_mask__set_int_range_commit_size.rs`
**Date:** 2026-03-24T11:15:10Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive: `segment_start` is an `external_body` function with no postconditions, leaving the mapping from segment IDs to addresses completely unconstrained — different IDs could overlap, and addresses have no alignment guarantees. The other candidate is a false positive — the `bytes` behavior of mapping a single bit to a full COMMIT_SIZE region is the intended design, confirmed by the existing `set_int_range_commit_size` proof in the source.

## True Positives (Spec Issues)

### segment_start_external_body_unconstrained
- **Confidence:** medium
- **Reasoning:** `segment_start` is `external_body` with no postconditions whatsoever — it provides no injectivity guarantee (distinct segment IDs could map to the same address) and no alignment or range constraints. This is a real spec weakness, though the missing constraints may be established elsewhere in the full codebase.

## All Candidates

### φ1: bytes_contains_all_below_commit_size
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Any address in the first COMMIT_SIZE range is in `bytes` if bit 0 is set — the `bytes` spec uses integer division which maps the entire range `[segment_start, segment_start + COMMIT_SIZE)` to index 0, but this means a single bit controls a 65536-byte region with no finer granularity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the intended design — `bytes` maps commit mask bits to address ranges, where each bit covers a COMMIT_SIZE (65536-byte) region. The `set_int_range_commit_size` proof in the source file proves exactly this property, confirming it's the expected behavior.

### φ2: bytes_unbounded_below
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `bytes` uses `(addr - segment_start) / COMMIT_SIZE` without lower-bound check — for negative offsets, integer division could map to index 0 or a negative index that wraps, potentially including addresses below the segment

### φ3: bytes_infinite_set
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** For ANY integer `k`, `(segment_start + k*COMMIT_SIZE - segment_start) / COMMIT_SIZE == k` — when k maps to a contained index, `bytes` includes addresses arbitrarily far from the segment, making it an infinite set with no upper bound

### φ4: segment_start_external_body_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `segment_start` is `external_body` with no postconditions — different segment IDs are not constrained to have different start addresses, and the start address could be any integer
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `segment_start` is `external_body` with no postconditions whatsoever — it provides no injectivity guarantee (distinct segment IDs could map to the same address) and no alignment or range constraints. This is a real spec weakness, though the missing constraints may be established elsewhere in the full codebase.

### φ5: bytes_negative_offset_in_range
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Address `segment_start - COMMIT_SIZE` has offset `-COMMIT_SIZE`, and `(-COMMIT_SIZE) / COMMIT_SIZE == -1` — if `mask@.contains(-1)` is false this shouldn't be included, but if the view's domain accidentally includes -1 then bytes extends below the segment

