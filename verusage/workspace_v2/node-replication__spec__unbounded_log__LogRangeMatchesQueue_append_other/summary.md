# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec__unbounded_log/spec__unbounded_log__LogRangeMatchesQueue_append_other.rs`
**Date:** 2026-03-24T11:48:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The source file contains fully verified open spec functions and lemmas with no external_body trust gaps (except the `Dispatch` trait which isn't exercised here). All tested properties — invariant preservation under remote append, empty range base case, local entry Placed state requirement, and Placed index matching — are correct direct consequences of the `LogRangeMatchesQueue` recursive definition.

## All Candidates

### φ1: append_other_preserves_queue
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Appending a remote entry preserves the existing queue match unchanged — tests that the append_other lemma correctly maintains the invariant when a different node's entry is added
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct instantiation of the fully verified `LogRangeMatchesQueue_append_other` lemma. The invariant preservation when appending a remote node's entry is correct and expected behavior.

### φ2: log_range_empty_implies_queue_end
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An empty log range forces the queue to be exhausted at `queueIndex` — tests the base case constraint
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The base case of `LogRangeMatchesQueue` directly states `logIndexLower == logIndexUpper ==> queueIndex == queue.len()`. This is the intended design — an empty log range means the queue is exhausted.

### φ3: log_range_local_entry_placed_state
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A single local log entry requires at least one queue element in Placed state — tests that the matching correctly links log entries to update states
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A single local log entry requiring a queue element in `Placed` state follows directly from the open spec definition's local case. This is the core intended semantics of `LogRangeMatchesQueue`.

### φ4: log_range_remote_skip
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A single remote log entry doesn't consume any queue element — tests that remote entries are skipped

### φ5: update_state_placed_idx_matches_log
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The Placed index in the update state must match the log index — tests that the spec correctly links the update's log position to the actual log entry
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The spec explicitly requires `updates[queue[queueIndex]].arrow_Placed_idx() == logIndexLower` in the local case. This is the fundamental linking invariant between updates and log positions — correct by design.

