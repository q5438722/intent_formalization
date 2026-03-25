# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec__unbounded_log_refines_simplelog/spec__unbounded_log_refines_simplelog__state_at_version_refines.rs`
**Date:** 2026-03-24T12:02:28Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. The `interp_log` length equaling `gtail` is definitional from `Seq::new`. The version-0 refinement is the trivial base case. And ignoring entries beyond `gtail` is the intended semantics of interpreting only the committed log prefix. The file contains fully verified proofs with no external_body trust gaps (except the phantom `RefinementProof` struct which carries no logical content).

## All Candidates

### φ1: interp_log_length_equals_gtail
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `interp_log` uses `Seq::new(global_tail, ...)` so its length is always `gtail` — but `i_log` could contain entries beyond `gtail` that are silently dropped by the interpretation
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Seq::new(global_tail, f)` produces a sequence of length `global_tail` by definition. Entries beyond `gtail` being dropped is the intended semantics — `interp_log` interprets exactly the committed log prefix up to `gtail`.

### φ2: refinement_at_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Refinement at version 0 is trivially `init_spec() == init_spec()` — tests the vacuous base case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Both `simple_log` and `unbounded_log` versions of `compute_nrstate_at_version` return `DT::init_spec()` at version 0. This is the trivially correct base case of the refinement proof — intended by design.

### φ3: interp_log_ignores_extra_entries
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Entries at or beyond `gtail` are invisible to `interp_log` — the map-to-seq interpretation silently truncates, so appending entries doesn't change the interpreted log
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `interp_log` constructs a `Seq` of length `gtail` indexing only `[0, gtail)`. Inserting at index `gtail` doesn't affect any index in that range. This is correct — the interpretation only covers the committed prefix.

### φ4: refinement_proof_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `RefinementProof` is `external_body` with only a `PhantomData` field — its existence is unconditionally provable, meaning the refinement witness carries no actual proof content

### φ5: state_at_version_node_id_irrelevant
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `node_id` is stored in `LogEntry` but completely ignored by `compute_nrstate_at_version` — `interp_log` strips it via `.op`, so the refinement erases node provenance entirely

