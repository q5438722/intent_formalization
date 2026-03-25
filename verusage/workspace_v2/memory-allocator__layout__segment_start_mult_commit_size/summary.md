# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/memory-allocator/verified/layout/layout__segment_start_mult_commit_size.rs`
**Date:** 2026-03-24T11:31:05Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. The segment-start-at-zero concern is mitigated by higher-level invariants, non-negative starts are correct by design for a userspace allocator, and `COMMIT_SIZE == SLICE_SIZE` is an intentional mimalloc design choice. This file defines simple constants and a basic multiplicative address mapping with no external_body trust gaps or spec weaknesses.

## All Candidates

### φ1: segment_start_mult_segment_size
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Segment start is always SEGMENT_SIZE-aligned — a trivial consequence of `id * SEGMENT_SIZE`, but over-constrains placement to natural alignment

### φ2: segment_start_zero_for_id_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Segment ID 0 maps to address 0 (null) — address 0 is conventionally invalid, yet the spec allows it as a valid segment start
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Segment ID 0 mapping to address 0 is a correct arithmetic consequence of the definition. In practice, the allocator would never use segment ID 0 — higher-level invariants ensure segments are at valid non-null addresses. The spec function itself is just a mathematical mapping.

### φ3: segment_start_nonneg
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Segment starts are always non-negative since `id: nat` — no constraint prevents `id == 0` which yields address 0
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Non-negative segment starts follow directly from `id: nat` and `SEGMENT_SIZE > 0`. This is correct and expected for a userspace allocator operating on non-negative virtual addresses.

### φ4: distinct_ids_distinct_starts
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Different segment IDs always yield different start addresses — but `SegmentId`s with same `id` but different `provenance`/`uniq` are semantically distinct segments sharing the same address

### φ5: commit_size_equals_slice_size
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `COMMIT_SIZE` is defined as `SLICE_SIZE` — commit granularity is locked to slice granularity with no independent configuration, so changing one silently changes the other
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `COMMIT_SIZE = SLICE_SIZE` is the deliberate mimalloc design — commit granularity equals slice granularity by construction. This is an intentional constant alias, not a spec gap.

