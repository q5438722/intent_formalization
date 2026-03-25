# Adversarial Proof Test Summary: `marshal_v__impl3__serialize`

## Target Specification
The `Marshalable` trait defines serialization with:
- `is_marshalable()`: domain predicate for valid inputs
- `ghost_serialize()`: spec-level serialization producing `Seq<u8>`
- `serialize()`: exec-level serialization with ensures: data grows, old data preserved, appended data = `ghost_serialize()`

Implementations tested: `u64`, `usize`, `Option<T>`

## Results

**All 12 adversarial tests FAILED verification as expected.** The specification correctly rejects all invalid inputs, incorrect behaviors, and unintended logical properties.

### Boundary Tests (4/4 rejected ✅)

| Test | Property Asserted | Why It Should Fail | Result |
|------|------------------|--------------------|--------|
| 1 | `!42u64.is_marshalable()` | u64 is unconditionally marshalable | ❌ REJECTED |
| 2 | `!None::<u64>.is_marshalable()` | None is unconditionally marshalable | ❌ REJECTED |
| 3 | `None::<u64>.ghost_serialize().len() == 0` | None serializes to `seq![0]` (length 1) | ❌ REJECTED |
| 4 | `!0usize.is_marshalable()` | 0 ≤ u64::MAX, so marshalable | ❌ REJECTED |

### Behavioral Mutation Tests (4/4 rejected ✅)

| Test | Property Asserted | Why It Should Fail | Result |
|------|------------------|--------------------|--------|
| 1 | `None.ghost_serialize()[0] == 1u8` | None tag byte is 0, not 1 | ❌ REJECTED |
| 2 | `Some(0u64).ghost_serialize()[0] == 0u8` | Some tag byte is 1, not 0 | ❌ REJECTED |
| 3 | `None.ghost_serialize() =~= seq![1u8]` | None serializes to `seq![0]`, not `seq![1]` | ❌ REJECTED |
| 4 | `None.ghost_serialize() =~= Some(0u64).ghost_serialize()` | Different variants produce different output | ❌ REJECTED |

### Logical Tests (4/4 rejected ✅)

| Test | Property Asserted | Why It Should Fail | Result |
|------|------------------|--------------------|--------|
| 1 | `0u64.ghost_serialize() =~= 1u64.ghost_serialize()` | Injectivity: distinct u64s serialize differently | ❌ REJECTED |
| 2 | `Some(0u64).ghost_serialize().len() == 1` | Some(x) = tag + inner, so len > 1 | ❌ REJECTED |
| 3 | `None.ghost_serialize().len() >= 2` | None serializes to exactly 1 byte | ❌ REJECTED |
| 4 | `0u64.ghost_serialize().len() == 1` | u64 LE serialization is 8 bytes, not 1 | ❌ REJECTED |

## Conclusion

The specification for `marshal_v__impl3__serialize` is **consistent** with respect to all tested adversarial properties:
- **Boundary completeness**: Invalid inputs (non-marshalable claims) are properly rejected
- **Behavioral correctness**: Mutated serialization outputs (wrong tags, wrong values) are rejected
- **Logical soundness**: Unintended properties (non-injectivity, wrong lengths, cross-variant confusion) are rejected

No spec weaknesses were found in this evaluation.
