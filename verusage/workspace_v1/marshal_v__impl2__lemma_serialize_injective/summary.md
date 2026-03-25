# Adversarial Proof Test Results — `marshal_v__impl2__lemma_serialize_injective`

## Target Specification

The `Marshalable` trait defines serialization injectivity via `lemma_serialize_injective`:
- **Requires**: `self.is_marshalable()`, `other.is_marshalable()`, `self.ghost_serialize() == other.ghost_serialize()`
- **Ensures**: `self.view_equal(other)`

Implementations: `u64`, `usize`, `Vec<u8>`, `Vec<T>`.

---

## Results Summary

| Category | Tests | All Failed (as expected) |
|----------|-------|--------------------------|
| Boundary | 5 | ✅ Yes |
| Behavioral Mutation | 4 | ✅ Yes |
| Logical | 5 | ✅ Yes |
| **Total** | **14** | **✅ All 14 rejected** |

---

## Boundary Tests (`boundary_tests.rs`) — 5/5 FAILED ✅

| # | Test | Violation | Result |
|---|------|-----------|--------|
| 1 | `test_boundary_distinct_u64_values` | `ghost_serialize(0) ≠ ghost_serialize(1)` | precondition not satisfied |
| 2 | `test_boundary_u64_max_vs_zero` | `ghost_serialize(0) ≠ ghost_serialize(u64::MAX)` | precondition not satisfied |
| 3 | `test_boundary_arbitrary_u64_no_serialize_eq` | Arbitrary u64s, serialize eq unproven | precondition not satisfied |
| 4 | `test_boundary_arbitrary_usize_not_marshalable` | Arbitrary usize, is_marshalable & serialize eq unproven | precondition not satisfied |
| 5 | `test_boundary_assert_view_equal_without_lemma` | Assert view_equal(42, 99) without lemma | assertion failed |

**Observation**: Test 4 (usize) only reports the `ghost_serialize` equality precondition failure, suggesting Verus may auto-prove `is_marshalable` for usize (i.e., `usize::MAX ≤ u64::MAX` holds in Verus's architecture model). This makes the `is_marshalable` check for `usize` effectively vacuous.

---

## Behavioral Mutation Tests (`mutation_tests.rs`) — 4/4 FAILED ✅

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_negate_postcondition` | Assert `!view_equal` after lemma | assertion failed |
| 2 | `test_mutation_wrong_inequality` | Assert `a@ > b@` after lemma | assertion failed |
| 3 | `test_mutation_off_by_one` | Assert `a@ == b@ + 1` after lemma | assertion failed |
| 4 | `test_mutation_assert_serialize_differ` | Assert `serialize(a) ≠ serialize(b)` despite equal precondition | assertion failed |

**Observation**: All mutations of the postcondition are correctly rejected. The spec precisely constrains the output relationship to `view_equal`.

---

## Logical Tests (`logical_tests.rs`) — 5/5 FAILED ✅

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_wrong_serialize_length` | `ghost_serialize(x).len() == 0` for u64 | assertion failed |
| 2 | `test_logical_all_u64_serialize_same` | All u64 values serialize identically | assertion failed |
| 3 | `test_logical_wrong_concrete_bytes` | `ghost_serialize(0) == [1,0,0,0,0,0,0,0]` | assertion failed |
| 4 | `test_logical_vec_serialize_is_identity` | Vec<u8> serialize is identity (no length prefix) | assertion failed |
| 5 | `test_logical_wrong_u64_serialize_length_one` | `ghost_serialize(x).len() == 1` for u64 | assertion failed |

**Observation**: The spec correctly rejects all fabricated structural/logical properties. The `open spec fn` definitions for `ghost_serialize` provide enough information for the verifier to distinguish correct from incorrect serialization properties.

Note: The "1 verified" in Verus output corresponds to `Vec<u8>::lemma_serialize_injective` (the source proof), not any test function.

---

## Conclusion

The specification for `lemma_serialize_injective` is **consistent** with respect to all 14 adversarial queries:

1. **Boundary completeness**: Invalid inputs (missing preconditions, distinct values) are properly rejected.
2. **Behavioral precision**: Incorrect output relations (negation, wrong inequality, off-by-one) are rejected.
3. **Logical soundness**: Unintended structural properties (wrong lengths, wrong bytes, identity serialization) are rejected.

No spec weaknesses were detected. The specification correctly entails only its intended properties and rejects all tested undesirable queries.
