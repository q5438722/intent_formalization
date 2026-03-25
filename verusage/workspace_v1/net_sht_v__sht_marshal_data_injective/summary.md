# Adversarial Proof Test Summary: `sht_marshal_data_injective`

## Target Specification

```rust
proof fn sht_marshal_data_injective(a: &CSingleMessage, b: &CSingleMessage)
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        a.ghost_serialize() == b.ghost_serialize(),
    ensures
        a@ == b@,
```

This function proves serialization injectivity for `CSingleMessage`: if two marshalable messages serialize to the same bytes, their abstract views must be equal.

---

## Test Results Overview

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary | 5 | 5 | 0 |
| Behavioral Mutation | 5 | 5 | 0 |
| Logical | 5 | 5 | 0 |
| **Total** | **15** | **15** | **0** |

**Verdict**: The specification is **consistent** — all 15 adversarial tests were correctly rejected.

---

## Boundary Tests (`boundary_tests.rs`)

Tests that violate preconditions to check if invalid inputs are rejected.

| Test | Violation | Result |
|---|---|---|
| B1: `test_boundary_a_not_marshalable` | `!a.is_marshalable()` (violates 1st precondition) | ✅ FAILED |
| B2: `test_boundary_b_not_marshalable` | `!b.is_marshalable()` (violates 2nd precondition) | ✅ FAILED |
| B3: `test_boundary_neither_marshalable` | Both args non-marshalable | ✅ FAILED |
| B4: `test_boundary_different_serializations` | `a.ghost_serialize() != b.ghost_serialize()` (violates 3rd precondition) | ✅ FAILED |
| B5: `test_boundary_all_preconditions_violated` | All three preconditions violated | ✅ FAILED |

**Analysis**: All preconditions are properly enforced. The function cannot be called with invalid inputs.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

Tests that start from valid inputs but mutate expected output relations.

| Test | Mutation | Result |
|---|---|---|
| M1: `test_mutation_negate_postcondition` | Assert `a@ != b@` (negate postcondition) | ✅ FAILED |
| M2: `test_mutation_ack_seqno_off_by_one` | Assert Ack seqno differs by 1 after proving equality | ✅ FAILED |
| M3: `test_mutation_serialize_length_zero` | Assert serialization length is 0 | ✅ FAILED |
| M4: `test_mutation_cross_variant_equality` | Assert InvalidMessage and Ack have same serialization | ✅ FAILED |
| M5: `test_mutation_different_acks_equal_view` | Assert Ack(42) and Ack(43) have same view | ✅ FAILED |

**Analysis**: The specification correctly rejects all mutated behavioral claims. Different concrete values produce different views, and the postcondition cannot be negated.

---

## Logical Tests (`logical_tests.rs`)

Tests for properties NOT explicitly guaranteed by the specification.

| Test | Property Tested | Result |
|---|---|---|
| L1: `test_logical_converse_injectivity` | `a@ != b@` ⟹ `serialize(a) != serialize(b)` (converse) | ✅ FAILED |
| L2: `test_logical_universal_marshalability` | All `CSingleMessage` values are marshalable | ✅ FAILED |
| L3: `test_logical_view_equal_implies_concrete_equal` | `serialize(a) == serialize(b)` ⟹ `a === b` (structural equality) | ✅ FAILED |
| L4: `test_logical_serialize_length_bounded` | Serialization length ≤ 100 | ✅ FAILED |
| L5: `test_logical_view_eq_implies_serialize_eq` | `a@ == b@` ⟹ `serialize(a) == serialize(b)` (reverse direction) | ✅ FAILED |

**Analysis**: The specification correctly does NOT entail any of these overly-strong properties:
- **L1/L5**: The spec only guarantees the forward direction of injectivity, not the converse.
- **L2**: Marshalability is a conditional property, not universal.
- **L3**: View equality (`a@ == b@`) is weaker than structural equality (`a === b`).
- **L4**: No upper bound on serialization length is specified.

---

## Conclusion

The `sht_marshal_data_injective` specification is **well-bounded**:
1. **Preconditions are enforced** — invalid inputs are rejected.
2. **Behavioral correctness is tight** — incorrect output relations are rejected.
3. **No unintended entailments** — the spec does not admit overly-strong logical consequences.

No specification weaknesses were discovered.
