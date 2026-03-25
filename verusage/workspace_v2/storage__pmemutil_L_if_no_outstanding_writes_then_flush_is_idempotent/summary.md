# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_pmemutil/pmemutil_L_if_no_outstanding_writes_then_flush_is_idempotent.rs`
**Date:** 2026-03-24T15:07:55Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: the single-region flush idempotency lemma is `external_body`, trusting extensional sequence equality after map without proof. Four false positives confirm correct definitional properties: flush commits pending values, map preserves length, flush clears all outstanding writes, and empty ranges are vacuously clean.

## True Positives (Spec Issues)

### region_flush_idempotent_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_if_no_outstanding_writes_to_region_then_flush_is_idempotent` is `external_body` with `unimplemented!()`. The property requires showing that `state.map(|_, b| b.flush())` produces a sequence extensionally equal to the original when all outstanding writes are `None` — this needs per-element reasoning plus `ext_equal` on sequences, which is trusted rather than verified.

## All Candidates

### φ1: region_flush_idempotent_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_if_no_outstanding_writes_to_region_then_flush_is_idempotent` is `external_body` — single-region flush idempotency is trusted without proof; the multi-region lemma's correctness entirely depends on this unverified assumption
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_if_no_outstanding_writes_to_region_then_flush_is_idempotent` is `external_body` with `unimplemented!()`. The property requires showing that `state.map(|_, b| b.flush())` produces a sequence extensionally equal to the original when all outstanding writes are `None` — this needs per-element reasoning plus `ext_equal` on sequences, which is trusted rather than verified.

### φ2: flush_outstanding_write_takes_new_value
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** After flushing a byte with an outstanding write, the old `state_at_last_flush` is permanently overwritten — tests that flush commits the pending value and loses the previous state
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `flush_byte` with `outstanding_write == Some(val)` returns `val`, so `flush().state_at_last_flush == val`. The second ensures is a tautology (`a != b || val == b`). Both follow directly from the definitions.

### φ3: flush_region_preserves_length
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Flushing a region should not change its size — if map altered the sequence length, indexing after flush would be invalid
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `flush` uses `self.state.map(...)` which preserves sequence length by the vstd `map` axiom. Correct by construction.

### φ4: no_outstanding_writes_after_flush
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** After flushing all bytes, every byte should have `outstanding_write == None` — if flush left outstanding writes, a second flush could change state, violating crash consistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Each `b.flush()` sets `outstanding_write: None`. After `map`, every element at index `k` has `outstanding_write.is_none()`. The quantifier in `no_outstanding_writes` is satisfied by SMT unfolding.

### φ5: no_outstanding_writes_in_empty_range
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An empty range should vacuously have no outstanding writes — if the range predicate were off-by-one (using `<=` instead of `<`), empty ranges would require checking a byte
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `no_outstanding_writes_in_range(i, i)` quantifies over `k` where `i <= k < i`, which is empty. Vacuously true.

