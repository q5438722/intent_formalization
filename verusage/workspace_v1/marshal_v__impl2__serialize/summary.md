# Test Results Summary: `marshal_v__impl2__serialize`

## Overview

15 adversarial proof tests were generated across three categories to probe the semantic boundary of the `Marshalable` trait specification (impls for `u64`, `usize`, `Vec<u8>`, `Vec<T: Marshalable>`).

**Result: All 15 tests correctly FAILED verification (0 verified, 15 errors).**

The specification is consistent — it rejects all tested undesirable properties.

---

## Boundary Tests (5/5 failed ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `boundary_u64_max_not_marshalable` | u64::MAX is NOT marshalable | FAIL ✅ — all u64 are marshalable |
| `boundary_usize_zero_serialize_empty` | usize(0) serializes to empty | FAIL ✅ — always 8 bytes |
| `boundary_u64_serialize_length_exceeds_8` | u64 serialization > 8 bytes | FAIL ✅ — always exactly 8 |
| `boundary_u64_serialize_length_under_8` | u64 serialization < 8 bytes | FAIL ✅ — always exactly 8 |
| `boundary_usize_zero_not_marshalable` | usize(0) is NOT marshalable | FAIL ✅ — 0 ≤ u64::MAX holds |

## Behavioral Mutation Tests (5/5 failed ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `mutation_different_u64_same_serialize` | 0u64 and 1u64 serialize identically | FAIL ✅ — injectivity holds |
| `mutation_u64_serialize_length_4` | u64 serialization is 4 bytes | FAIL ✅ — it is 8 bytes |
| `mutation_u64_roundtrip_wrong_value` | roundtrip(42) == 99 | FAIL ✅ — roundtrip(42) == 42 |
| `mutation_usize_u64_same_value_different_serialize` | usize(42) ≠ u64(42) serialization | FAIL ✅ — they are equal by spec |
| `mutation_vec_u8_serialize_ignores_prefix` | Vec<u8> serialization has no length prefix | FAIL ✅ — prefix is 8 bytes |

## Logical Tests (5/5 failed ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `logical_u64_serialize_constant` | all u64 values serialize identically | FAIL ✅ — serialization is injective |
| `logical_u64_serialize_unbounded` | some u64 serializes to > 8 bytes | FAIL ✅ — always exactly 8 |
| `logical_cross_type_different_value_same_serialize` | u64(0) and usize(1) serialize identically | FAIL ✅ — different values differ |
| `logical_vec_u8_serialize_length_equals_content` | serialized length == content length | FAIL ✅ — must include 8-byte prefix |
| `logical_u64_serialize_all_zeros` | u64(1) serializes to all-zero bytes | FAIL ✅ — LE encoding of 1 has non-zero byte |

## Conclusion

The specification for `marshal_v__impl2__serialize` correctly:
1. **Rejects invalid inputs** — precondition boundaries are well-guarded
2. **Rejects incorrect behaviors** — output mutations are caught by spec constraints
3. **Rejects unintended reasoning** — logical overreach (constancy, unbounded length, cross-type confusion) is not entailed

No spec weaknesses were detected in this round of testing.
