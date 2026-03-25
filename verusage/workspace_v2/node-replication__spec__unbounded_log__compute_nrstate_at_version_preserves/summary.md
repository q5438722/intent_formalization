# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec__unbounded_log/spec__unbounded_log__compute_nrstate_at_version_preserves.rs`
**Date:** 2026-03-24T11:58:13Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. The version-zero base case, irrelevance of out-of-range log entries, and determinism from identical entry sequences are all correct and intended properties of `compute_nrstate_at_version`. The `node_id` field being unused in state computation is by design — it serves routing/ownership purposes elsewhere, not state transition logic.

## All Candidates

### φ1: compute_nrstate_version_zero_is_init
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Version 0 always returns `init_spec()` regardless of log contents — tests the base case where the log is completely ignored
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The base case `version == 0` returning `DT::init_spec()` is directly stated in the open spec definition. This is the intended initial state — no log entries should affect it.

### φ2: dispatch_mut_spec_trusted_no_constraints
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `dispatch_mut_spec` is a trusted trait method with no constraints — any implementation could define arbitrary state transitions, and the spec blindly trusts it

### φ3: preserves_allows_extra_keys
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Maps that agree on `[0, version)` produce the same state even if `b` has extra entries beyond `version` — the preservation lemma ignores entries at or above the version
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `compute_nrstate_at_version` only accesses log entries in `[0, version)`. Entries at or beyond `version` are irrelevant by construction. This is correct and expected — the preservation lemma's precondition only requires agreement on the relevant range.

### φ4: compute_nrstate_deterministic_same_ops
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** State computation depends only on the op sequence, not the log map identity — two different maps with identical entries produce the same state, meaning `node_id` is never used in state computation
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Two logs with identical entries (same `op` and `node_id`) at every index are equal by extensionality, satisfying the preservation lemma's `a[i] == b[i]` precondition. The `node_id` being unused in state computation is by design — `dispatch_mut_spec` only takes the op, not the node_id. This is a correct determinism property of a replicated state machine.

### φ5: node_id_irrelevant_to_state
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The `node_id` field of `LogEntry` is completely ignored by `compute_nrstate_at_version` — different nodes submitting the same op produce identical states, meaning the state computation cannot distinguish which node performed an operation

