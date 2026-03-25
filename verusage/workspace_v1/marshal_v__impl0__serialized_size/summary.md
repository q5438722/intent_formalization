# Adversarial Test Summary: `marshal_v__impl0__serialized_size`

## Target Specification

The `Marshalable` trait defines:
- `is_marshalable()`: precondition guard
- `ghost_serialize()`: spec-level serialization to `Seq<u8>`
- `serialized_size()`: exec function with `ensures res == ghost_serialize().len()`

Implementations: `u64` (always marshalable, 8-byte LE encoding) and `usize` (marshalable when ≤ `u64::MAX`, delegates to u64).

---

## Results Overview

| # | File | Test | Expected | Actual | Status |
|---|------|------|----------|--------|--------|
| 1 | boundary | `test_u64_has_non_marshalable_value` | FAIL | FAIL | ✅ |
| 2 | boundary | `test_non_marshalable_usize_serialize_len` | FAIL | FAIL | ✅ |
| 3 | boundary | `test_u64_max_wrong_serialize_length` | FAIL | FAIL | ✅ |
| 4 | behavioral | `test_u64_serialize_length_mutated_to_4` | FAIL | FAIL | ✅ |
| 5 | behavioral | `test_distinct_u64_same_serialization` | FAIL | FAIL | ✅ |
| 6 | behavioral | `test_u64_serialize_empty` | FAIL | FAIL | ✅ |
| 7 | logical | `test_u64_serialize_injective` | FAIL | FAIL | ✅ |
| 8 | logical | `test_u64_first_byte_always_zero` | FAIL | FAIL | ✅ |
| 9 | logical | `test_u64_last_byte_always_zero` | FAIL | FAIL | ✅ |

**All 9 adversarial tests correctly FAIL verification**, confirming the spec rejects these invalid properties.

---

## 🔍 Spec Weakness Finding

During initial testing, an additional property was tested:

```
assert(forall|x: usize| x.is_marshalable());
```

**This PASSED verification**, revealing that `is_marshalable` for `usize` (defined as `*self as int <= u64::MAX`) is **vacuously true** under Verus's architecture model. Since Verus assumes `usize` fits within 64 bits, every `usize` value automatically satisfies the guard.

**Implication**: The `is_marshalable` precondition on `usize` provides **no actual filtering** — it admits all values unconditionally. If the spec intends to restrict certain `usize` values, this guard is ineffective.

---

## Analysis by Category

### Boundary Tests (3/3 FAIL ✅)
- **Test 1**: Spec correctly makes all u64 values marshalable (no non-marshalable u64 exists).
- **Test 2**: Spec correctly prevents reasoning about `ghost_serialize` when `recommends` is violated for non-marshalable usize.
- **Test 3**: Spec correctly rejects wrong serialization length (4) for edge-case `u64::MAX`.

### Behavioral Mutation Tests (3/3 FAIL ✅)
- **Test 4**: Mutated length (4 instead of 8) correctly rejected.
- **Test 5**: Asserting `serialize(0) == serialize(1)` correctly rejected — different values produce different byte sequences.
- **Test 6**: Asserting empty serialization for u64 correctly rejected — u64 always produces 8 bytes.

### Logical Tests (3/3 FAIL ✅)
- **Test 7**: Injectivity (`serialize(a) == serialize(b) ⟹ a == b`) is **not automatically provable** from the spec. The spec only guarantees length, not injectivity. This is appropriate — injectivity requires the round-trip lemma.
- **Test 8**: First byte always zero — correctly rejected (counterexample: any u64 with nonzero low byte).
- **Test 9**: Last byte always zero — correctly rejected (counterexample: any u64 ≥ 2^56).

---

## Conclusion

The specification is **sound for the properties tested**: it correctly rejects all 9 adversarial properties. However, the `is_marshalable` guard for `usize` is vacuously true, which may represent an unintended spec weakness if architecture-independence was intended.
