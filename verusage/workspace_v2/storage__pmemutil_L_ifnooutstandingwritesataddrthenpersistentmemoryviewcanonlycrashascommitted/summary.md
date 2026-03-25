# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_pmemutil/pmemutil_L_ifnooutstandingwritesataddrthenpersistentmemoryviewcanonlycrashascommitted.rs`
**Date:** 2026-03-24T15:09:04Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. They confirm correct definitional properties: map preserves length, committed state is a valid crash state when no writes are outstanding, per-chunk crash atomicity allows mixed flushed/committed states across chunks, and flush_byte returns the outstanding write value.

## All Candidates

### φ1: committed_length_equals_state_length
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `committed()` uses `map` which should preserve length — if it didn't, indexing into `committed()` would be out of bounds
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `committed()` uses `self.state.map(...)` which preserves sequence length by the vstd `map` axiom. Correct by construction.

### φ2: can_crash_as_committed_when_no_writes
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** When no outstanding writes exist, the committed state should be a valid crash state — if `can_crash_as` rejected the committed view, no valid crash state would exist
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With no outstanding writes, `flush_byte()` returns `state_at_last_flush` for every byte. Both `chunk_corresponds_ignoring_outstanding_writes` and `chunk_corresponds_after_flush` hold for `committed()`, so `can_crash_as` is satisfied. Correct by definition.

### φ3: chunk_size_divides_addr_range
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Two addresses in the same chunk should be within chunk_size of each other — if the chunk calculation allowed distant addresses in the same chunk, crash atomicity would span too large a range

### φ4: crash_state_allows_mixed_chunks
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Different chunks can independently choose between flushed and committed states — one chunk crashes as committed while another crashes as flushed; if the crash model forced all-or-nothing across chunks, it would be too strong
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Chunk 0 (addresses 0–7) uses `chunk_corresponds_ignoring_outstanding_writes` (all 0u8 = state_at_last_flush), chunk 1 (addresses 8–15) uses `chunk_corresponds_after_flush` (42/99 = flush_byte). This is the intended per-chunk crash atomicity model.

### φ5: flush_byte_with_outstanding_ignores_old
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `flush_byte` with an outstanding write should return the new value, not the old — if it returned the old value, flushing would be a no-op and writes would never persist
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `flush_byte` matches `Some(new_val)` and returns `new_val`. Correct by the match arm definition.

