# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/exec_utils/exec__utils__rids_match_add_rid.rs`
**Date:** 2026-03-24T11:46:15Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. The source file contains fully verified functions with no external_body trust gaps. The empty window base case, single-element match, and window preservation under appending are all correct mathematical consequences of the open spec recursive definition and the verified `rids_match_add_rid` lemma.

## All Candidates

### φ1: rids_match_add_rid_extends_both
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Pushing `Some(rid)` and extending both windows by 1 preserves the match — tests that appending a matched pair extends the relation

### φ2: rids_match_empty_preserves_after_push
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Empty match is preserved after constructing sequences with one element each — the empty window `[0,0)` trivially matches `[0,0)` regardless of sequence contents
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An empty window `[0,0)` trivially matches `[0,0)` by the base case of the recursive definition, regardless of sequence contents. This is correct by design.

### φ3: rids_match_single_push
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A single `Some(rid)` pushed onto empty sequences matches a single rid — tests the minimal non-trivial extension
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A single `Some(rid)` matching a single `rid` is the minimal non-trivial case — it follows directly from the recursive definition and the verified `rids_match_add_rid` lemma. No spec gap.

### φ4: rids_match_duplicate_rids_allowed
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The same `rid` can appear multiple times in both sequences — `rids_match` imposes no uniqueness constraint on request IDs

### φ5: rids_match_add_rid_idempotent_window
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Pushing two rid pairs preserves the original window match — the original `[bools_start, bools_end)` window is unaffected by appending beyond it
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Appending elements beyond the current window does not affect the match within the window — `rids_match` only inspects indices within `[bools_start, bools_end)`, so pushing beyond `bools_end` is invisible. This is a correct compositional consequence of the verified lemma.

