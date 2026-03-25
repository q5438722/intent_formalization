# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/regular/regular__leb128__impl2__lemma_spec_parse_non_zero.rs`
**Date:** 2026-03-24T15:33:25Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. Empty-input failure follows directly from the open spec definition. The `> 1` bound for continuation-byte parsing is proven by a verified lemma with bitvector reasoning — while the bound could be tightened to `>= 128`, the weaker bound is still correct and does not indicate a spec inconsistency.

## All Candidates

### φ1: parse_empty_is_none
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Parsing an empty sequence should fail — if it succeeded, the parser would produce values from no input
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `spec_parse` definition explicitly checks `s.len() != 0` and returns `None` for empty input. Correct by definition.

### φ2: parse_continuation_only_fails
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A single continuation byte with no following data should fail — if it succeeded, truncated LEB128 encodings would silently produce values

### φ3: parse_non_canonical_zero_rejected
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The encoding 0x80 0x00 represents 0 non-canonically (v2==0 fails `0 < v2`) — if accepted, the same value would have multiple encodings breaking injectivity

### φ4: parse_high_bit_result_greater_than_one
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** When the first byte has high bit set and parse succeeds, the result must be > 1 — this is proven by a verified lemma, but the bound `> 1` rather than `>= 128` seems surprisingly weak; if the lower bound were actually higher, the lemma would be hiding a tighter invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `> 1` bound follows from the verified `lemma_spec_parse_non_zero` which uses a bitvector proof that `rest << 7 | low_bits > 1` when `0 < rest`. The bound could be tighter (`>= 128`), but `> 1` is still a correct (if weak) property — a weak-but-true ensures is not a spec gap.

### φ5: parse_value_fits_consumed_bits
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The consumed byte count must be at least 1 — if n could be 0 or negative, the parser would not make progress, potentially causing infinite loops in sequential combinators

