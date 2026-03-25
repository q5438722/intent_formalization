# Adversarial Test Results: `lemma_is_marshalable_CKeyHashMap`

## Target
`marshal_ironsht_specific_v__lemma_is_marshalable_CKeyHashMap.rs`

**Lemma under test:**
```
proof fn lemma_is_marshalable_CKeyHashMap(h: CKeyHashMap)
  requires valid_hashtable(h@)
  ensures  h.is_marshalable()
```

Where `valid_hashtable` requires `dom().len() < 62` and all values have `len() < 1024`.

---

## Results Summary

| # | Category | Test Name | Failure Mode | Result |
|---|----------|-----------|--------------|--------|
| 1 | Boundary | `test_boundary_exact_max_size` | `dom().len() == 62` (not `< 62`) | ✅ FAILED (precondition not satisfied) |
| 2 | Boundary | `test_boundary_value_too_long` | Value length `== 1024` (not `< 1024`) | ✅ FAILED (precondition not satisfied) |
| 3 | Boundary | `test_boundary_extreme_size` | `dom().len() > 1000` | ✅ FAILED (precondition not satisfied) |
| 4 | Mutation | `test_mutation_negate_postcondition` | Assert `!is_marshalable()` for valid input | ✅ FAILED (assertion failed) |
| 5 | Mutation | `test_mutation_exceed_serialize_bound` | Assert serialize len `> 0x100000` | ✅ FAILED (assertion failed) |
| 6 | Mutation | `test_mutation_unsorted_keys` | Assert keys NOT sorted | ✅ FAILED (assertion failed) |
| 7 | Logical  | `test_logical_converse` | `is_marshalable() ⟹ valid_hashtable()` | ✅ FAILED (assertion failed) |
| 8 | Logical  | `test_logical_tight_serialize_bound` | Serialize len `<= 100` (too tight) | ✅ FAILED (assertion failed) |
| 9 | Logical  | `test_logical_injectivity` | Different maps ⟹ different serializations | ✅ FAILED (assertion failed) |

**Verification output:** `1 verified, 9 errors` (the 1 verified is the original lemma itself)

---

## Analysis

### (1) Boundary Tests — All 3 FAILED ✅
The precondition `valid_hashtable(h@)` correctly guards the lemma:
- **Size boundary** (`< 62`): Exact boundary value 62 is rejected.
- **Value length boundary** (`< 1024`): Exact boundary value 1024 is rejected.
- **Extreme inputs**: Very large hashtables are rejected.

The preconditions are precise and cannot be bypassed at boundaries.

### (2) Behavioral Mutation Tests — All 3 FAILED ✅
The postcondition `h.is_marshalable()` is strong enough to reject:
- **Negated marshalability**: Cannot prove a valid hashtable is NOT marshalable.
- **Exceeded size bound**: Cannot prove serialized output exceeds `0x100000`.
- **Unsorted keys**: Cannot prove `to_vec()` output is unsorted (spec enforces sorted keys via `spec_sorted_keys`).

The specification correctly constrains the output behavior.

### (3) Logical Tests — All 3 FAILED ✅
The specification does not entail unintended properties:
- **Converse direction**: `is_marshalable()` does NOT imply `valid_hashtable()` — the spec correctly avoids over-constraining the relationship.
- **Tight bound**: The spec allows serialized sizes up to `0x100000` and does not collapse to a trivially small bound.
- **Injectivity**: Serialization injectivity is not guaranteed — this is appropriate since the spec focuses on marshalability, not bijective encoding.

---

## Conclusion

The specification for `lemma_is_marshalable_CKeyHashMap` is **consistent**: it rejects all 9 adversarial queries across boundary violations, behavioral mutations, and unwarranted logical inferences. No weaknesses were detected.
