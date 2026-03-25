# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/regular/regular__leb128__impl2__lemma_spec_parse_low_7_bits.rs`
**Date:** 2026-03-24T15:32:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. Empty-input failure and single-byte parsing follow directly from the open spec definition. The consumed-length bound is provable by structural induction on the recursive spec without relying on any trusted axioms. No spec gaps found.

## All Candidates

### φ1: parse_empty_is_none
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Parsing an empty sequence should fail — if it succeeded, the parser would produce values from no input
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `spec_parse` definition explicitly checks `s.len() != 0` and returns `None` for empty input. Correct by definition.

### φ2: parse_single_byte_value_range
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A single byte below 0x80 should parse to its low 7 bits — if the base case produced a different value, all multi-byte encodings would be wrong
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For `s = seq![b]` with `b < 0x80`, `is_high_8_bit_set` is false, so the spec returns `Some((1, take_low_7_bits!(b) as UInt))` = `Some((1, (b & 0x7f) as UInt))`. Correct by unfolding the open spec.

### φ3: parse_single_continuation_byte_fails
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A single continuation byte (high bit set) with no following data should fail to parse — if it succeeded, truncated encodings would silently produce incorrect values

### φ4: parse_rejects_non_canonical_zero_continuation
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The encoding 0x80 0x00 is a non-canonical representation of 0 (v2 == 0 fails the `0 < v2` check) — if accepted, the same value would have multiple encodings, breaking injectivity

### φ5: parse_consumed_at_most_input_length
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The number of bytes consumed should never exceed input length — if it did, the parser would claim to read past the end of the buffer
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The base case returns `n=1` for `s.len() >= 1`. The recursive case adds 1 to `n` from `s.drop_first()`, and by induction `n2 <= s.drop_first().len()`, so `n = n2 + 1 <= s.len()`. The proof itself is verified and correct by structural induction on the open spec.

