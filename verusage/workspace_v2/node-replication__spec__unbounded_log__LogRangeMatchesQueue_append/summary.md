# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec__unbounded_log/spec__unbounded_log__LogRangeMatchesQueue_append.rs`
**Date:** 2026-03-24T11:47:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. The empty log range forcing queue exhaustion is the intended base case of `LogRangeMatchesQueue`. The append lemma instantiation is a correct consequence of the fully verified proof. The Dispatch trait test only exercises trivial struct construction, not any actual trust gap from `#[verus::trusted]`.

## All Candidates

### φ1: log_range_empty_queue_end
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** When the log range is empty (`lower == upper`), the queue must also be exhausted — tests whether an empty log range at a non-zero queue index is rejected
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** When `logIndexLower == logIndexUpper`, the spec directly requires `queueIndex == queue.len()`. With `queueIndex == 0`, this gives `queue.len() == 0`. This is the intended base case of the recursive matching — an empty log range means all local entries have been matched.

### φ2: log_range_local_entry_consumes_queue
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A single local log entry must consume exactly one queue element in `Placed` state — tests the one-entry local case

### φ3: log_range_remote_entry_no_queue
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A single remote log entry should not consume any queue element — tests that remote entries are skipped entirely

### φ4: append_preserves_existing_entries
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Full instantiation of append lemma from index 0 — tests that the verified lemma correctly extends the log range by 1 with a new local entry and queue element
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct instantiation of the fully verified `LogRangeMatchesQueue_append` lemma at index 0. The lemma is proved by induction with no external_body — this is a correct and expected consequence.

### φ5: dispatch_trait_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `Dispatch` trait is `#[verus::trusted]` — all associated types (`WriteOperation`, `Response`, `View`) are trusted without verification, and any implementation could introduce unsoundness
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The φ only tests that a struct literal has the expected field values (`entry.node_id == node_id`), which is trivially true by construction. The `#[verus::trusted]` on `Dispatch` means implementations aren't verified, but this φ doesn't expose any actual unsoundness — it's just struct field access.

