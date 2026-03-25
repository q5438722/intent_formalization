# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_pmemutil/pmemutil_L_ifnooutstandingwritesthenpersistentmemoryregionsviewcanonlycrashascommitted.rs`
**Date:** 2026-03-24T15:10:19Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: the single-region crash-state uniqueness lemma is `external_body`, trusting that no outstanding writes implies only `committed()` is a valid crash state without verification. Four false positives confirm correct definitional properties: committed is a valid crash state, the two chunk predicates coincide without outstanding writes, committed length matches region count, and empty regions crash as empty.

## True Positives (Spec Issues)

### crash_only_as_committed_external_body
- **Confidence:** medium
- **Reasoning:** `lemma_if_no_outstanding_writes_then_persistent_memory_view_can_only_crash_as_committed` is `external_body` with `unimplemented!()`. It asserts that with no outstanding writes, the only valid crash state equals `committed()`. This requires showing that for every chunk, both `chunk_corresponds_ignoring_outstanding_writes` and `chunk_corresponds_after_flush` produce the same bytes (= `state_at_last_flush`), forcing `s == committed()` — this reasoning is trusted, not verified.

## All Candidates

### φ1: crash_only_as_committed_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_if_no_outstanding_writes_then_persistent_memory_view_can_only_crash_as_committed` is `external_body` — the uniqueness of crash states when no writes are outstanding is trusted without proof; if `can_crash_as` admitted extra states, crash recovery could observe non-committed data
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_if_no_outstanding_writes_then_persistent_memory_view_can_only_crash_as_committed` is `external_body` with `unimplemented!()`. It asserts that with no outstanding writes, the only valid crash state equals `committed()`. This requires showing that for every chunk, both `chunk_corresponds_ignoring_outstanding_writes` and `chunk_corresponds_after_flush` produce the same bytes (= `state_at_last_flush`), forcing `s == committed()` — this reasoning is trusted, not verified.

### φ2: can_crash_as_allows_committed
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The committed state should always be a valid crash state when there are no outstanding writes — if `can_crash_as` rejected committed, no valid crash state would exist
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With no outstanding writes, `flush_byte()` returns `state_at_last_flush` for every byte. Both chunk predicates hold for `committed()`, and `committed().len() == view.len()` by map length preservation. Correct by definition.

### φ3: chunk_ignoring_and_after_flush_same_when_no_writes
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** With no outstanding writes, `flush_byte()` equals `state_at_last_flush` for every byte — the two chunk predicates should be equivalent; if they diverged, the crash model would be inconsistent
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With no outstanding writes, every byte has `outstanding_write == None`, so `flush_byte() == state_at_last_flush`. Both predicates check `bytes[addr] == state_at_last_flush` for the same addresses, making them equivalent. Correct by definition.

### φ4: regions_committed_length
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The committed sequence should have one entry per region — if `Seq::new` produced a different length, indexing into `committed()` would be misaligned with the regions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `committed()` is defined as `Seq::new(self.len(), ...)` which produces a sequence of length `self.len()` by the vstd `Seq::new` axiom. Trivially correct.

### φ5: can_crash_as_empty_regions
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Empty regions should crash as the empty sequence — if the empty case failed, initialization of the persistent memory system couldn't establish crash consistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `can_crash_as` checks `crash_regions.len() == self.len()` (both 0) and a vacuously true quantifier over regions. Correct by empty quantification.

