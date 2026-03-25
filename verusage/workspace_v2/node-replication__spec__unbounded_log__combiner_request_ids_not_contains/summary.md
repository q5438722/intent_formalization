# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec__unbounded_log/spec__unbounded_log__combiner_request_ids_not_contains.rs`
**Date:** 2026-03-24T11:59:03Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The freshness biconditional (φ1, φ2) follows from the fully verified `combiner_request_ids_not_contains` lemma. The single-combiner case (φ3) is a direct consequence of the freshness definition. The Ready combiner contributing nothing (φ5) follows from `queued_ops()` returning empty for Ready state. No spec gaps found in this file — all functions are open specs or fully verified proofs with no external_body.

## All Candidates

### φ1: fresh_rid_not_in_combiner_ids
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A fresh rid (not in any combiner's queued_ops) is absent from the aggregate set — tests one direction of the biconditional proved by `combiner_request_ids_not_contains`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is one direction of the biconditional proved by the fully verified `combiner_request_ids_not_contains` lemma. Fresh rid not being in the aggregate set is correct by design.

### φ2: not_in_ids_implies_fresh
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The reverse direction — absence from the aggregate set implies freshness across all nodes — tests that the fold-over-choose correctly captures all combiners
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The other direction of the same fully verified biconditional. The fold-over-choose correctly captures all combiners — this is the intended semantics.

### φ3: single_combiner_fresh_iff_not_in_queue
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** For a single-node combiner map, freshness reduces to the rid not being in that node's queue — tests the single-element case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For a singleton map `{nid -> cs}`, the universally quantified freshness condition reduces to checking just that one node's queue. This follows directly from the definition of `combiner_request_id_fresh`.

### φ4: combiner_ids_infinite_unconstrained
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When the domain is infinite, `combiner_request_ids` returns `arbitrary()` which could be any set — an infinite combiner map makes every rid appear to be contained (or not), depending on the arbitrary choice

### φ5: ready_combiner_contributes_nothing
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A Ready combiner with empty queued_ops adds nothing to the aggregate set — tests that adding a Ready node doesn't spuriously include rids
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `CombinerState::Ready.queued_ops()` returns `Seq::empty()`, so `seq_to_set` of it is the empty set. Adding an empty set to the aggregate changes nothing. Correct by design.

