# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/regular/regular__leb128__impl2__lemma_spec_serialize_length.rs`
**Date:** 2026-03-24T15:34:59Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. They confirm correct properties of the LEB128 serialization spec: non-emptiness and last-byte termination follow from the recursive structure, zero encoding follows from unfolding the base case, and the 10-byte bound is proven by a verified lemma with bitvector reasoning. No spec gaps found.

## All Candidates

### φ1: serialize_nonempty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Serialization should always produce at least one byte — if empty output were possible, there would be no bytes for a parser to consume
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `spec_serialize_helper` always returns either `seq![lo]` (base case) or `seq![...] + recursive_result`. Both branches produce at least one element. Correct by the open spec structure.

### φ2: serialize_zero_is_single_zero
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Zero should serialize to a single 0x00 byte — if it produced multiple bytes or a different value, the canonical encoding of zero would be wrong
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For `v = 0`: `lo = 0 & 0x7f = 0`, `hi = 0 >> 7 = 0`, so base case returns `seq![0u8]`. Correct by unfolding the open spec with bitvector facts.

### φ3: serialize_128_is_two_bytes
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** 128 is the smallest two-byte LEB128 value (0x80, 0x01) — if the encoding boundary between one and two bytes were wrong, all multi-byte values would be misencoded

### φ4: serialize_length_max_10
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A u64 should serialize to at most 10 bytes (ceil(64/7)) — the proof relies on `reveal_with_fuel` and bitvector reasoning; if the fuel or shift chain were wrong, the bound would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The verified `lemma_spec_serialize_length` proves this using `reveal_with_fuel(10)` and a bitvector chain showing 10 right-shifts of 7 bits zeroes any u64. The proof is verified, not `external_body`. Correct.

### φ5: serialize_last_byte_below_0x80
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The last byte of a LEB128 encoding must have the high bit unset (terminator) — if the last byte had high bit set, a parser would expect more continuation bytes
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** In the base case (`hi == 0`), the last byte is `take_low_7_bits!(v)` which is `< 0x80` by bitvector. In the recursive case, the last byte comes from the recursive call. The inductive proof is verified. Correct.

