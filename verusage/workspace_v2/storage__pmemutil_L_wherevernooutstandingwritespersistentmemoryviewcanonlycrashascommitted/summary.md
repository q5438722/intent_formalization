# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_pmemutil/pmemutil_L_wherevernooutstandingwritespersistentmemoryviewcanonlycrashascommitted.rs`
**Date:** 2026-03-24T15:14:17Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

One clear true positive: the per-address crash determinism lemma is `external_body` and trusted without proof. One derivative true positive: the two-address same-chunk corollary inherits the same trust gap by invoking the external_body lemma twice. Three false positives confirm correct definitional properties: committed is a valid crash state, the flushed state is always a valid crash outcome, and mixed chunks with clean/dirty bytes correctly crash as flushed.

## True Positives (Spec Issues)

### per_addr_crash_determinism_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_if_no_outstanding_writes_at_addr_then_persistent_memory_view_can_only_crash_as_committed` is `external_body` with `unimplemented!()`. It asserts that a byte with no outstanding write must crash as committed regardless of which chunk branch (ignoring writes vs after flush) is taken. This requires reasoning that `flush_byte()` equals `state_at_last_flush` when `outstanding_write` is `None` and that the chunk disjunction forces this — trusted without verification.

### two_addrs_same_chunk_same_outcome
- **Confidence:** medium
- **Reasoning:** This is a direct corollary of invoking the `external_body` lemma twice. While the property itself is correct, both invocations rely on the unverified per-address crash determinism assumption. The composition inherits the same trust gap.

## All Candidates

### φ1: per_addr_crash_determinism_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_if_no_outstanding_writes_at_addr_then_persistent_memory_view_can_only_crash_as_committed` is `external_body` — the per-address crash determinism when no outstanding write exists is trusted without proof; if a chunk containing this byte crashes as flushed but the byte has no write, it should still equal committed, but this reasoning is unverified
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_if_no_outstanding_writes_at_addr_then_persistent_memory_view_can_only_crash_as_committed` is `external_body` with `unimplemented!()`. It asserts that a byte with no outstanding write must crash as committed regardless of which chunk branch (ignoring writes vs after flush) is taken. This requires reasoning that `flush_byte()` equals `state_at_last_flush` when `outstanding_write` is `None` and that the chunk disjunction forces this — trusted without verification.

### φ2: crash_as_admits_all_committed
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** When there are no outstanding writes, the committed state should be a valid crash outcome — if `can_crash_as` rejected committed, no valid crash state could exist
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With no outstanding writes, `flush_byte() == state_at_last_flush` for all bytes. Both chunk predicates hold for `committed()`, so `can_crash_as(committed())` is satisfied. Correct by definition.

### φ3: crash_as_admits_flushed_state
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The fully-flushed state should always be a valid crash outcome — every chunk satisfies `chunk_corresponds_after_flush`; if the flushed state weren't admitted, the crash model would reject a physically realizable outcome
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The flushed sequence has `s[addr] == view.state[addr].flush_byte()` for all addresses, so every chunk satisfies `chunk_corresponds_after_flush`. Correct by definition.

### φ4: two_addrs_same_chunk_same_outcome
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Two addresses in the same chunk with no outstanding writes must both crash as committed — this is a corollary of the per-address external_body lemma, testing that the trusted assumption composes correctly across addresses
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** This is a direct corollary of invoking the `external_body` lemma twice. While the property itself is correct, both invocations rely on the unverified per-address crash determinism assumption. The composition inherits the same trust gap.

### φ5: mixed_chunk_outstanding_write_addr_can_differ
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A chunk with mixed clean/dirty bytes can crash as flushed — the dirty byte takes its outstanding write value while the clean byte's flush_byte equals state_at_last_flush anyway; tests that the per-chunk model correctly handles heterogeneous outstanding writes within a chunk
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The chunk satisfies `chunk_corresponds_after_flush` since `b_clean.flush_byte() == 0` and `b_dirty.flush_byte() == 42`, matching the crash sequence exactly. The length matches (8 == 8). Correct by definition.

