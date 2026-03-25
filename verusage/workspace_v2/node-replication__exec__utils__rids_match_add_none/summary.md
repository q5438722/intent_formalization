# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/exec_utils/exec__utils__rids_match_add_none.rs`
**Date:** 2026-03-24T11:44:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. The source file contains a fully verified open spec function `rids_match` and a fully verified lemma `rids_match_add_none` with no external_body trust gaps. Empty range matching is the base case, and all-None ranges matching empty rids is the intended semantics of the recursive definition.

## All Candidates

### φ1: rids_match_empty_ranges
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Empty bools range matches empty rids range regardless of position — correct but shows rids_match is trivially satisfied at any aligned empty window
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty ranges matching is the base case of the recursive definition — `bools_end == bools_start` returns `rids_end == rids_start`, which is true when both are empty. This is correct by design.

### φ2: rids_match_all_none_empty_rids
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A bools range of all `None` matches an empty rids range — correct but shows `rids_match` ignores `None` entries entirely
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `None` entries in `bools` are explicitly skipped in the recursive definition (the `else` branch recurses without advancing `rids_end`). A range of all `None` matching an empty rids range is the intended behavior — `None` means "no request here."

### φ3: rids_match_single_some
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A single `Some(rid)` in bools matches a single matching rid — tests the base case of the recursive matching

### φ4: rids_match_add_none_extends
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Pushing `None` and extending the bools window by 1 still matches the same rids range — the `None` at the new position is silently skipped

### φ5: rids_match_no_rid_order_constraint
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `rids_match` pairs bools and rids right-to-left — `bools[1]` pairs with `rids[1]` and `bools[0]` with `rids[0]`; tests whether arbitrary distinct rids are accepted in order

