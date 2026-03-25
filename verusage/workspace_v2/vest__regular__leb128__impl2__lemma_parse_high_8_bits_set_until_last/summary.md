# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/regular/regular__leb128__impl2__lemma_parse_high_8_bits_set_until_last.rs`
**Date:** 2026-03-24T15:30:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: both `lemma_parse_length` and `lemma_parse_productive` are independent `external_body` axioms with `unimplemented!()` bodies on the `UnsignedLEB128` implementation, trusting parse-length bounds and productivity without verification. Three false positives: empty-input failure and single-byte parsing follow directly from the open spec definition, and the max-value bound is a type-level tautology for `u64`.

## True Positives (Spec Issues)

### parse_length_external_body_trusted
- **Confidence:** high
- **Reasoning:** `lemma_parse_length` for `UnsignedLEB128` is `external_body` with `unimplemented!()`. The property that parsed length ≤ input length is non-trivial for this recursive parser and is entirely trusted. It is also used as a dependency in `lemma_parse_high_8_bits_set_until_last`.

### parse_productive_external_body_trusted
- **Confidence:** high
- **Reasoning:** `lemma_parse_productive` for `UnsignedLEB128` is a separate `external_body` with `unimplemented!()`. The productivity property (n > 0 on successful parse) is trusted without verification and is used in `lemma_parse_high_8_bits_set_until_last` to derive a contradiction.

## All Candidates

### φ1: parse_length_external_body_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_parse_length` is `external_body` with `unimplemented!()` — the bound that parsed length ≤ input length is trusted without verification for this recursive parser
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_parse_length` for `UnsignedLEB128` is `external_body` with `unimplemented!()`. The property that parsed length ≤ input length is non-trivial for this recursive parser and is entirely trusted. It is also used as a dependency in `lemma_parse_high_8_bits_set_until_last`.

### φ2: parse_productive_external_body_trusted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_parse_productive` is `external_body` with `unimplemented!()` — the productivity guarantee is trusted without proof for UnsignedLEB128 which hardcodes `is_productive() == true`
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `lemma_parse_productive` for `UnsignedLEB128` is a separate `external_body` with `unimplemented!()`. The productivity property (n > 0 on successful parse) is trusted without verification and is used in `lemma_parse_high_8_bits_set_until_last` to derive a contradiction.

### φ3: parse_empty_is_none
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Parsing an empty sequence should fail — if it somehow succeeded, the parser would produce values from no input, violating the productivity and length bounds
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `spec_parse` definition explicitly checks `s.len() != 0` and returns `None` for empty input. This follows directly from unfolding the open spec fn definition. Correct by definition.

### φ4: parse_single_byte_no_high_bit
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A single byte with high bit unset should parse to its low 7 bits in exactly 1 byte — if this base case were wrong, all multi-byte encodings built on top would produce incorrect values
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For `s = seq![b]` with `b < 0x80`, the spec unfolds: `s.len() != 0` is true, `is_high_8_bit_set` is false, so it returns `Some((1, (b & 0x7f) as UInt))`. Correct by definition.

### φ5: parse_max_value_overflow_check
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The parsed value must fit in u64 — the overflow check `v2 <= n_bit_max_unsigned!(8*8-7)` combined with `v2 << 7 | low_bits` should never exceed UInt::MAX, but the recursive accumulation depends on the trusted length/productive lemmas for termination reasoning
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `v` has type `UInt` which is `u64`, so `v <= UInt::MAX` is trivially true by the type's range. This is a tautology for any value of type `u64`.

