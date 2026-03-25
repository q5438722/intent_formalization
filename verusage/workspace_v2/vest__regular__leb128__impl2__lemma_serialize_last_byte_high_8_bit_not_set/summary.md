# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/vest/verified/regular/regular__leb128__impl2__lemma_serialize_last_byte_high_8_bit_not_set.rs`
**Date:** 2026-03-24T15:31:45Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives. Serialize non-emptiness follows from the structure of `spec_serialize_helper` which always produces at least one byte in both branches. The 127 single-byte encoding follows directly from unfolding the spec with `127 >> 7 == 0`. No spec gaps found in the serialization logic.

## All Candidates

### φ1: serialize_zero_is_single_zero_byte
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Serializing zero should produce a single zero byte — if it produced anything else, the LEB128 encoding of zero would be non-canonical or wrong

### φ2: serialize_nonempty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Serialization should always produce at least one byte — if it could produce an empty sequence, parsing would have no bytes to consume, violating productivity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `spec_serialize_helper` always returns either `seq![lo]` (base case) or `seq![set_high_8_bit!(lo)] + recursive_result`. Both branches produce at least one element. This is correct by the open spec definition.

### φ3: serialize_high_bits_set_except_last
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** All bytes except the last should have the high bit set — if any intermediate byte lacked the high bit, the parser would terminate early and produce an incorrect value

### φ4: serialize_max_length_bounded
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A u64 value should serialize to at most 10 bytes (ceil(64/7)) — if the serialization could exceed this, it would indicate the recursive encoding is producing spurious continuation bytes

### φ5: serialize_127_is_single_byte
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** 127 (0x7F) is the largest single-byte LEB128 value — if it required two bytes, the encoding boundary between single and multi-byte would be wrong
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For `v = 127u64`: `hi = 127 >> 7 == 0`, so the base case applies, returning `seq![take_low_7_bits!(127)]` = `seq![127u8]`. Correct by unfolding the open spec definition with bitvector arithmetic.

