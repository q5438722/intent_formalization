# Adversarial Proof Test Results: `sht_marshal_data_injective`

## Target Specification

```rust
pub proof fn sht_marshal_data_injective(a: &CSingleMessage, b: &CSingleMessage)
requires
    a.is_marshalable(),
    b.is_marshalable(),
    a.ghost_serialize() == b.ghost_serialize(),
ensures
    a@ == b@,
```

This lemma proves injectivity of serialization with respect to the abstract view: if two marshalable `CSingleMessage` values have identical serializations, their views must be equal.

---

## Results Summary

| # | File | Test Name | Type | Expected | Actual | Status |
|---|------|-----------|------|----------|--------|--------|
| 1 | boundary_tests.rs | `test_boundary_a_not_marshalable` | Boundary | FAIL | precondition not satisfied | ✅ |
| 2 | boundary_tests.rs | `test_boundary_b_not_marshalable` | Boundary | FAIL | precondition not satisfied | ✅ |
| 3 | boundary_tests.rs | `test_boundary_serializations_differ` | Boundary | FAIL | precondition not satisfied | ✅ |
| 4 | boundary_tests.rs | `test_boundary_no_preconditions` | Boundary | FAIL | assertion failed | ✅ |
| 5 | behavioral_mutation_tests.rs | `test_mutation_negate_postcondition` | Behavioral | FAIL | assertion failed | ✅ |
| 6 | behavioral_mutation_tests.rs | `test_mutation_wrong_serialization_length` | Behavioral | FAIL | assertion failed | ✅ |
| 7 | behavioral_mutation_tests.rs | `test_mutation_empty_serialization` | Behavioral | FAIL | assertion failed | ✅ |
| 8 | logical_tests.rs | `test_logical_serialization_length_constant` | Logical | FAIL | assertion failed | ✅ |
| 9 | logical_tests.rs | `test_logical_all_marshalable` | Logical | FAIL | assertion failed | ✅ |
| 10 | logical_tests.rs | `test_logical_transitivity_abuse` | Logical | FAIL | assertion failed | ✅ |

**All 10 tests failed verification as expected. No spec weaknesses detected.**

---

## Analysis

### Boundary Tests (4/4 FAIL ✅)
The specification correctly rejects all precondition violations:
- **Tests 1–3**: Each removes one precondition (`is_marshalable` for a or b, or serialization equality), and Verus correctly rejects the function call.
- **Test 4**: Without any preconditions, view equality cannot be established for arbitrary messages.

### Behavioral Mutation Tests (3/3 FAIL ✅)
The specification correctly rejects mutated output relations:
- **Test 5**: Negating the postcondition (`a@ != b@`) is correctly rejected — the spec does entail view equality.
- **Tests 6–7**: Asserting wrong serialization length properties (exactly 1 byte, or 0 bytes) is correctly rejected — the spec doesn't constrain serialization length to specific values.

### Logical Tests (3/3 FAIL ✅)
The specification does not accidentally entail unintended properties:
- **Test 8**: Marshalable messages do NOT all have the same serialization length (different variants/payloads produce different lengths).
- **Test 9**: Not every `CSingleMessage` is marshalable (marshalability depends on field sizes and serialization bounds).
- **Test 10**: Injectivity is NOT transitive without establishing serialization equality between all pairs. The spec correctly limits its guarantee to the specific pair whose serializations match.

## Conclusion

The specification of `sht_marshal_data_injective` is **well-formed**: it correctly rejects invalid inputs (boundary), incorrect behaviors (mutations), and unintended logical inferences (logical properties). No spec weaknesses were identified.
