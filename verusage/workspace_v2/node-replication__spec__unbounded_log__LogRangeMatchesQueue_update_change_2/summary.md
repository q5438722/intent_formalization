# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec__unbounded_log/spec__unbounded_log__LogRangeMatchesQueue_update_change_2.rs`
**Date:** 2026-03-24T11:56:23Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The non-queue-rid droppability and state swapability (φ1, φ2, φ3) reflect the correct frame condition of `LogRangeMatchesQueue_update_change_2` — the spec only accesses `updates` through queue-contained rids, so non-queue entries are intentionally irrelevant. The empty range base case (φ5) is directly stated in the spec definition.

## All Candidates

### φ1: update_change_2_non_queue_rid_droppable
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Any update entry not in the queue can be silently removed — the frame condition only guards queue-contained rids, allowing arbitrary mutation of non-queue entries
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `LogRangeMatchesQueue` only accesses `updates` via `queue[queueIndex]` — entries not in the queue are never referenced. Removing them is correct and expected; this is the intended frame condition of `update_change_2`.

### φ2: update_change_2_empty_updates_all_remote
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** When the queue is empty (all remote entries), the updates map can be replaced with an entirely empty map — shows the match is completely independent of updates for all-remote ranges
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** When the queue is empty, all log entries in the range are remote. The spec never looks up `updates` for remote entries, so replacing updates with an empty map is correct. This is the expected behavior for all-remote ranges.

### φ3: update_change_2_swap_non_queue_state
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A non-queue rid's update state can be changed to any arbitrary state (Init, Applied, Done) without affecting the match — the frame condition is blind to non-queue entries
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Same reasoning as the droppable case — non-queue rids are never accessed by `LogRangeMatchesQueue`, so their state is irrelevant. Inserting an arbitrary state for a non-queue rid is correct.

### φ4: local_entry_queue_rid_in_updates
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Single local log entry forces exactly one queue element in Placed state linking to that log index

### φ5: empty_log_range_exhausts_queue
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Empty log range forces queue index to equal queue length — base case of the recursion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Directly follows from the base case of the recursive definition: `logIndexLower == logIndexUpper ==> queueIndex == queue.len()`. This is the intended termination condition.

