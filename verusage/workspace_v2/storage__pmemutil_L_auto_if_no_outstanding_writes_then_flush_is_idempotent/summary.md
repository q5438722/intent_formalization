# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_pmemutil/pmemutil_L_auto_if_no_outstanding_writes_then_flush_is_idempotent.rs`
**Date:** 2026-03-24T15:07:01Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: the flush idempotency lemma for `PersistentMemoryRegionsView` is `external_body`, trusting structural equality across mapped sequences without proof. Four false positives: flush_byte identity, byte-level flush idempotency, flush clearing outstanding writes, and empty regions vacuous truth all follow directly from the definitions.

## True Positives (Spec Issues)

### flush_idempotent_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_if_no_outstanding_writes_then_flush_is_idempotent` is `external_body` with `unimplemented!()`. While the property is correct (flush with no outstanding writes produces identical bytes and `None` outstanding writes), it requires reasoning about `ext_equal` on sequences after `map` — this structural equality is trusted rather than verified.

## All Candidates

### φ1: flush_idempotent_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_if_no_outstanding_writes_then_flush_is_idempotent` is `external_body` — flush idempotency when there are no outstanding writes is trusted without proof; if flush altered `state_at_last_flush` even with no outstanding writes, the equality would be unsound
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_if_no_outstanding_writes_then_flush_is_idempotent` is `external_body` with `unimplemented!()`. While the property is correct (flush with no outstanding writes produces identical bytes and `None` outstanding writes), it requires reasoning about `ext_equal` on sequences after `map` — this structural equality is trusted rather than verified.

### φ2: flush_byte_no_outstanding_is_state
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** When there's no outstanding write, flush_byte should return the last flushed state — if flush_byte returned something else, crash recovery would read corrupted data
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `flush_byte` matches on `outstanding_write`: `None => self.state_at_last_flush`. With `outstanding_write.is_none()`, it directly returns `state_at_last_flush`. Correct by definition.

### φ3: flush_double_is_flush_single
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Flushing twice should equal flushing once — if flush weren't idempotent at the byte level, repeated crash-recovery cycles could produce different states
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `b.flush()` produces `{ state_at_last_flush: b.flush_byte(), outstanding_write: None }`. Flushing again: `flush_byte` on `None` returns `state_at_last_flush` (= `b.flush_byte()`), and `outstanding_write` stays `None`. Identical result. Correct by definition.

### φ4: flush_clears_outstanding_write
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** After flushing, a byte should have no outstanding write — if outstanding writes persisted through flush, the write-ahead log invariant would be violated
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `flush` is defined to set `outstanding_write: None`. Trivially correct by construction.

### φ5: no_outstanding_writes_empty_regions
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An empty regions view should vacuously have no outstanding writes — if the empty case failed, initialization would be unable to establish the no-outstanding-writes invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `no_outstanding_writes` quantifies over `0 <= i < self.len()`. With `regions: seq![]`, `self.len() == 0`, so the quantifier is vacuously true.

