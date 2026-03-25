# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/exec_utils/exec__utils__rids_match_pop.rs`
**Date:** 2026-03-24T11:45:26Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The source file contains a fully verified open spec function `rids_match` and a fully verified inversion lemma `rids_match_pop` with no external_body trust gaps. All tested properties — empty range base case, left-to-right Some/None popping, and rid count bound — are correct mathematical consequences of the recursive definition.

## All Candidates

### φ1: rids_match_empty_bools_empty_rids
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Empty bools range trivially matches empty rids range at any position — base case of the recursion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the base case of the recursive definition — `bools_end == bools_start` returns `rids_end == rids_start`, which is true when both ranges are empty. Correct by design.

### φ2: rids_match_pop_some_consumes_rid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A `Some` at `bools_start` forces `rids_start < rids_end` and value equality — tests that `rids_match_pop` correctly inverts the right-to-left definition into left-to-right access
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the core property proved by `rids_match_pop` — inverting the right-to-left recursive definition to allow left-to-right access. Both functions are fully verified, no spec gap.

### φ3: rids_match_pop_none_preserves
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A `None` at `bools_start` can be skipped without consuming a rid — tests that left-to-right popping of `None` preserves the match relation
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Skipping `None` entries without consuming a rid is the intended semantics, directly proved by the verified `rids_match_pop` lemma. No spec gap.

### φ4: rids_match_single_some_singleton
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A single `Some(rid)` matches a single `rid` — tests the minimal non-trivial case of the matching relation

### φ5: rids_match_implies_rid_count
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The number of matched rids never exceeds the number of bools entries — the `None` entries are skipped, so `rids_end - rids_start <= bools_end - bools_start`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The rid count being bounded by the bools count is a correct structural property — each `Some` consumes one rid and each `None` consumes zero. This is a straightforward inductive consequence of the open spec definition.

