# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/log_logimpl/logimpl_tentatively_append_to_log.rs`
**Date:** 2026-03-24T15:05:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `lemma_tentatively_append` is an `external_body` proof trusting that non-wrapping log append preserves the complex `info_consistent_with_log_area` invariant without verification. Three false positives: nat arithmetic triviality for subregion bounds, map-preserves-length for write, and direct definitional unfolding for outstanding write recording.

## True Positives (Spec Issues)

### lemma_tentatively_append_external_body_invariant
- **Confidence:** high
- **Reasoning:** `lemma_tentatively_append` is `external_body` with `unimplemented!()` body. It asserts that writing bytes to the log area and updating info/state preserves `info_consistent_with_log_area` — a complex invariant involving circular buffer arithmetic, persistent memory byte state, and flush semantics. This entire proof is trusted without verification.

## All Candidates

### φ1: lemma_tentatively_append_external_body_invariant
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_tentatively_append` is `external_body` — the invariant preservation after non-wrapping append is trusted without proof; if the circular buffer arithmetic or persistent memory state tracking is wrong, log corruption goes undetected
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_tentatively_append` is `external_body` with `unimplemented!()` body. It asserts that writing bytes to the log area and updating info/state preserves `info_consistent_with_log_area` — a complex invariant involving circular buffer arithmetic, persistent memory byte state, and flush semantics. This entire proof is trusted without verification.

### φ2: lemma_tentatively_append_wrapping_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `lemma_tentatively_append_wrapping` is `external_body` — the wrapping case asserts no outstanding writes in the wrap-around target range without proof; if the invariant doesn't guarantee this for the beginning of the log area, a crash could corrupt already-flushed data

### φ3: subregion_start_len_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `start()` and `len()` are both `external_body` closed specs — any property about their relationship is trusted; if `start + len` could overflow or `len` could be inconsistent with the actual subregion, writes would target wrong addresses
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `start()` and `len()` return `nat` values (non-negative integers). The ensures `sub.start() + sub.len() >= sub.start()` is trivially true since `sub.len()` is a `nat` (≥ 0). This holds regardless of the external_body implementations — it's a consequence of natural number arithmetic.

### φ4: write_preserves_length
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writing to a persistent memory region should not change its length — if the map-based write accidentally altered the sequence length, all offset calculations would be invalidated
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `write` is defined via `self.state.map(...)` which preserves sequence length by the vstd `map` axiom. The resulting state sequence has the same length as the input, so `view.write(...).len() == view.len()`. Correct by construction.

### φ5: write_within_range_has_outstanding
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** After writing, positions within the range should have the new byte as an outstanding write — if `PersistentMemoryByte::write` didn't properly record the outstanding write, flush_byte would return stale data
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For `k` in range `[addr, addr + bytes.len())`, the map closure returns `pre_byte.write(bytes[k - addr])` which sets `outstanding_write: Some(bytes[k - addr])`. This follows directly from the definitions of `PersistentMemoryRegionView::write` and `PersistentMemoryByte::write`.

