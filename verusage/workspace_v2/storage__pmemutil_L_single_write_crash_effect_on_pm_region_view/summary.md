# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_pmemutil/pmemutil_L_single_write_crash_effect_on_pm_region_view.rs`
**Date:** 2026-03-24T15:12:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: the per-address crash determinism for unwritten bytes depends on the `external_body` lemma that asserts bytes without outstanding writes must crash as committed — this is trusted without verification. Four false positives confirm correct definitional properties: write-then-flush-then-committed yields the written bytes, write preserves bytes outside the range, aligned writes span a single chunk, and both pre-write committed and post-flush committed are valid crash outcomes.

## True Positives (Spec Issues)

### single_write_crash_depends_on_external_body
- **Confidence:** medium
- **Reasoning:** This property relies on `lemma_wherever_no_outstanding_writes_persistent_memory_view_can_only_crash_as_committed`, which is `external_body` with `unimplemented!()`. The claim that unwritten bytes in a crash state must equal committed values requires proving that both chunk-level branches yield `state_at_last_flush` when `outstanding_write` is `None` — this reasoning is trusted, not verified.

## All Candidates

### φ1: single_write_crash_depends_on_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The proof that unwritten bytes crash as committed relies entirely on the `external_body` lemma — if the per-address crash determinism is wrong, unwritten regions could contain arbitrary data after a crash
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** This property relies on `lemma_wherever_no_outstanding_writes_persistent_memory_view_can_only_crash_as_committed`, which is `external_body` with `unimplemented!()`. The claim that unwritten bytes in a crash state must equal committed values requires proving that both chunk-level branches yield `state_at_last_flush` when `outstanding_write` is `None` — this reasoning is trusted, not verified.

### φ2: write_then_flush_committed_equals_bytes
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** After writing and flushing, the committed bytes in the written range should equal the written bytes — if flush didn't properly propagate outstanding writes to `state_at_last_flush`, crash recovery would see stale data
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For `k` in the write range, `write` sets `outstanding_write: Some(bytes_to_write[k - write_addr])`. Then `flush` sets `state_at_last_flush` to `flush_byte()` = that value, and `committed()` maps to `state_at_last_flush`. Correct by chaining the definitions.

### φ3: write_outside_range_unchanged
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writing should not affect bytes outside the written range — if the map-based write leaked into other positions, unrelated persistent data would be corrupted
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `write` map closure returns `pre_byte` unchanged when `!(addr <= pos < addr + bytes.len())`. For `k` outside the range, the byte is preserved. Correct by definition.

### φ4: aligned_write_single_chunk
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An aligned write of exactly chunk_size bytes should touch exactly one chunk — if addresses within the range mapped to different chunks, the two-outcome crash lemma would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With `write_addr` aligned to 8 and `k` in `[write_addr, write_addr + 8)`, integer division by 8 yields the same chunk. Standard arithmetic property of aligned ranges.

### φ5: crash_exactly_two_outcomes
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Both the old committed state and the fully-flushed state should be valid crash outcomes — if either were rejected, the crash model would be strictly stronger than intended, potentially making some correct programs unverifiable
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For committed: unwritten chunks satisfy both predicates (no outstanding writes), and the written chunk satisfies `chunk_corresponds_ignoring_outstanding_writes` since `state_at_last_flush` is preserved by write. For flushed: the written chunk satisfies `chunk_corresponds_after_flush`, and unwritten chunks satisfy both (flush_byte = state_at_last_flush when no outstanding write). Both are valid crash states by construction.

