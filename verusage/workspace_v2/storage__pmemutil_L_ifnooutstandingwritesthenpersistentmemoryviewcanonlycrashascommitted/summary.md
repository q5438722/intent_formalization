# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_pmemutil/pmemutil_L_ifnooutstandingwritesthenpersistentmemoryviewcanonlycrashascommitted.rs`
**Date:** 2026-03-24T15:11:09Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: the per-address crash determinism lemma for bytes without outstanding writes is `external_body`, trusting that both chunk crash branches yield `state_at_last_flush` when no write is pending — this requires non-trivial reasoning about the chunk-level disjunction. Four false positives confirm correct definitional properties: length preservation, vacuous truth for invalid chunks, map projection for committed, and flushed crash state for a uniform chunk.

## True Positives (Spec Issues)

### wherever_no_writes_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_wherever_no_outstanding_writes_persistent_memory_view_can_only_crash_as_committed` is `external_body` with `unimplemented!()`. It asserts that for any address without an outstanding write, any crash state must match `committed()` at that address. This requires reasoning that both chunk-level crash branches (ignoring writes vs after flush) produce `state_at_last_flush` when `outstanding_write` is `None` — trusted without verification.

## All Candidates

### φ1: wherever_no_writes_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_wherever_no_outstanding_writes_persistent_memory_view_can_only_crash_as_committed` is `external_body` — the per-address crash determinism for bytes without outstanding writes is trusted without proof; if a chunk with mixed outstanding/non-outstanding bytes could crash as flushed, the non-outstanding byte would change
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_wherever_no_outstanding_writes_persistent_memory_view_can_only_crash_as_committed` is `external_body` with `unimplemented!()`. It asserts that for any address without an outstanding write, any crash state must match `committed()` at that address. This requires reasoning that both chunk-level crash branches (ignoring writes vs after flush) produce `state_at_last_flush` when `outstanding_write` is `None` — trusted without verification.

### φ2: crash_as_preserves_length
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Any valid crash state must have the same length as the region — if crash states could have different lengths, all index-based reasoning would be invalid
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `can_crash_as` directly requires `bytes.len() == self.len()` as its first conjunct. Trivially correct by definition.

### φ3: chunk_vacuous_for_out_of_bounds
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Negative chunk indices are vacuously true because no valid address divides to a negative chunk — if the chunk predicate weren't vacuous for invalid chunks, `can_crash_as` would impose impossible constraints
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For chunk = -1, no address `addr` with `0 <= addr < self.len()` satisfies `addr / 8 == -1` (since non-negative integers divided by positive give non-negative results). The quantifier antecedent is always false, making both predicates vacuously true.

### φ4: committed_at_addr_equals_state_at_last_flush
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `committed()` should project `state_at_last_flush` for each byte — if the map produced different values, crash recovery would read corrupted data
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `committed()` is `self.state.map(|_, b| b.state_at_last_flush)`. By the vstd map indexing axiom, `committed()[addr] == state[addr].state_at_last_flush`. Correct by definition.

### φ5: outstanding_write_in_chunk_allows_flushed_crash
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A full chunk with outstanding writes should be able to crash as the flushed state — tests that the crash model allows the flushed branch per chunk; if it didn't, pending writes could never become visible after a crash
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** All 8 bytes have `outstanding_write == Some(val)`, so `flush_byte() == val` for each. `chunk_corresponds_after_flush(0, flushed)` holds since `flushed[addr] == val == flush_byte()`. For any other chunk, the predicate is vacuously true. Correct by the crash model definition.

