# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/log_logimpl/logimpl_L_check_fast_way_to_compute_head_mod_log_area_len.rs`
**Date:** 2026-03-24T15:04:51Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. Two are trivial restatements of their own preconditions (aligned offset and identity). The boundary case is simple arithmetic from the preconditions. The small advancement case correctly applies the verified lemma's non-wrapping branch. No spec gaps found — the lemma is fully verified with a detailed modular arithmetic proof.

## All Candidates

### φ1: head_mod_zero_when_aligned
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** When head is perfectly aligned to log_area_len, head_log_area_offset should be 0 — tests that the modular arithmetic base case is consistent
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The precondition states `info.head_log_area_offset == info.head as int % info.log_area_len as int` and `info.head as int % info.log_area_len as int == 0`. By transitivity, `info.head_log_area_offset == 0`. This is a trivial logical consequence of the preconditions.

### φ2: no_advancement_preserves_offset
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Zero advancement (new_head == head) should yield the same offset — tests the trivial identity case of the modular arithmetic
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause `info.head as int % info.log_area_len as int == info.head_log_area_offset` is literally a restatement of the requires clause `info.head_log_area_offset == info.head as int % info.log_area_len as int`. Trivially correct.

### φ3: advancement_equals_log_area_len_wraps_to_same
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Advancing by exactly log_area_len should wrap back to the same offset — if the modular arithmetic doesn't handle full-cycle wrapping, the fast computation would diverge from the true modulus

### φ4: fast_path_matches_slow_path_at_boundary
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** When advancement exactly reaches the end of the log area, the fast path must take the wrapping branch — if the branch condition is off-by-one, the boundary case would compute an out-of-range offset
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The precondition `info.head_log_area_offset + info.log_length == info.log_area_len` means `info.log_length == info.log_area_len - info.head_log_area_offset`, so `amount >= info.log_area_len - info.head_log_area_offset` holds with equality. Simple arithmetic.

### φ5: lemma_small_advancement_no_wrap
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** For small advancements (no wrapping), the new offset should simply be the old offset plus the advancement — tests that the non-wrapping branch of the fast computation is a simple addition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This directly invokes the verified lemma `lemma_check_fast_way_to_compute_head_mod_log_area_len` and the precondition `advance < info.log_area_len - info.head_log_area_offset` selects the non-wrapping branch, yielding `advance + info.head_log_area_offset`. The lemma is fully verified (not external_body), so this is a correct consequence.

