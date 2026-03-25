# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec__unbounded_log/spec__unbounded_log__concat_LogRangeNoNodeId_LogRangeMatchesQueue.rs`
**Date:** 2026-03-24T11:59:50Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The empty range base case, concat lemma instantiation, all-remote empty queue derivation, and out-of-range insertion frame property are all correct and expected consequences of the open spec definitions and fully verified lemmas. No external_body or trust gaps are present in this file.

## All Candidates

### φ1: no_node_id_empty_range
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An empty range `[idx, idx)` vacuously satisfies `LogRangeNoNodeId` — tests that the base case holds regardless of log contents
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `LogRangeNoNodeId` spec only has a condition when `start < end`. For `start == end`, the implication is vacuously true. This is the correct base case.

### φ2: concat_no_node_extends_match
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Prepending a no-node-id range to a matching range extends the match — tests that remote-only log entries can be freely prepended
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct instantiation of the fully verified `concat_LogRangeNoNodeId_LogRangeMatchesQueue` lemma at `queueIndex=0`. Prepending remote-only entries to a matching range is the intended use case of this lemma.

### φ3: all_remote_empty_queue
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A log range with no local entries matches an empty queue — tests that all-remote ranges require zero queue elements
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Combines the empty-range base case (`LogRangeMatchesQueue` with `[end, end)` and empty queue) with the concat lemma. An all-remote range matching an empty queue is correct by design — no local entries means no queue consumption.

### φ4: no_node_id_single_entry
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A single remote entry satisfies LogRangeNoNodeId — tests the one-element inductive case

### φ5: no_node_id_ignores_extra_keys
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Inserting a log entry at or beyond `end` doesn't affect `LogRangeNoNodeId` — tests that the predicate is insensitive to out-of-range mutations
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `LogRangeNoNodeId` only accesses log entries in `[start, end)`. Inserting at `extra_idx >= end` doesn't affect any accessed key. This is a correct frame property.

