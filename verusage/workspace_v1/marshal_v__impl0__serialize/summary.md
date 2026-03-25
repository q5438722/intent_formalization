# Adversarial Test Results — `marshal_v__impl0__serialize`

## Target Specification

The `Marshalable` trait defines `is_marshalable()` (precondition), `ghost_serialize()` (spec-level serialization), and `serialize()` (exec-level serialization with postconditions preserving old data and appending serialized bytes). Implementations exist for `u64`, `usize`, `Vec<u8>`, and `Vec<T>`.

---

## Results Summary

| # | Test Name | Type | Property Tested (φ) | Result |
|---|-----------|------|---------------------|--------|
| 1 | `boundary_u64_max_not_marshalable` | Boundary | u64::MAX is not marshalable | **REJECTED ✓** |
| 2 | `boundary_u64_zero_empty_serialize` | Boundary | 0u64 serializes to empty sequence | **REJECTED ✓** |
| 3 | `boundary_u64_serialize_too_short` | Boundary | u64 serialization < 8 bytes | **REJECTED ✓** |
| 4 | `boundary_usize_serialize_wrong_length` | Boundary | usize serialization ≠ 8 bytes | **REJECTED ✓** |
| 5 | `mutation_different_u64_same_serialize` | Mutation | 0u64 and 1u64 produce same serialization | **REJECTED ✓** |
| 6 | `mutation_u64_wrong_serialize_length` | Mutation | u64 serializes to 4 bytes (not 8) | **REJECTED ✓** |
| 7 | `mutation_wrong_roundtrip_value` | Mutation | round-trip of 42u64 yields 43 | **REJECTED ✓** |
| 8 | `logical_no_u64_is_marshalable` | Logical | ∀v:u64. ¬is_marshalable(v) | **REJECTED ✓** |
| 9 | `logical_cross_type_different_serialize` | Logical | u64 and usize serialize differently for same value | **REJECTED ✓** |
| 10 | `logical_serialize_unbounded_length` | Logical | ∃v:u64. serialize(v).len() > 8 | **REJECTED ✓** |

**Total: 10/10 tests REJECTED (verification failed as expected)**

---

## Analysis

### Boundary Tests (4/4 rejected)
The specification correctly constrains edge cases:
- `is_marshalable` for u64 is definitionally `true`, rejecting any claim to the contrary.
- `ghost_serialize` for u64 delegates to `spec_u64_to_le_bytes`, which is axiomatically 8 bytes (via `lemma_auto_spec_u64_to_from_le_bytes`), rejecting wrong-length claims.
- `usize` serialization correctly delegates to u64, preserving the 8-byte length property.

### Behavioral Mutation Tests (3/3 rejected)
The specification correctly rejects mutated behaviors:
- **Injectivity**: The round-trip axiom (`from_le_bytes(to_le_bytes(v)) == v`) makes serialization injective, so different u64 values cannot share the same serialization.
- **Length**: The 8-byte length is a hard axiom, rejecting any alternative.
- **Round-trip fidelity**: The axiom precisely fixes the round-trip value, rejecting off-by-one mutations.

### Logical Tests (3/3 rejected)
The specification correctly prevents unintended logical reasoning:
- The universal negation of marshalability is rejected because `is_marshalable` is `true` for all u64.
- Cross-type divergence is rejected because `usize::ghost_serialize` is defined as `(*self as u64).ghost_serialize()`, making them extensionally equal.
- Unbounded serialization length is rejected by the 8-byte axiom.

---

## Conclusion

The specification for `Marshalable` and its `u64`/`usize`/`Vec<u8>` implementations is **consistent** with respect to all tested adversarial properties. No invalid inputs, incorrect behaviors, or unintended logical inferences were admitted. The specification correctly rejects all 10 queries that fall outside its intended semantic space.
