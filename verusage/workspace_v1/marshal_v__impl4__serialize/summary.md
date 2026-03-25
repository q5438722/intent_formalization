# Adversarial Proof Test Results — `marshal_v__impl4__serialize`

## Target
`source-projects/ironkv/verified/marshal_v/marshal_v__impl4__serialize.rs`

Specifications tested: `Marshalable` trait (with impls for `u64`, `usize`, `Vec<u8>`, `Vec<T>`), and `lemma_seq_fold_left_append_right`.

---

## Summary

| Category              | Tests | All Failed (as expected) |
|-----------------------|-------|--------------------------|
| Boundary              | 3     | ✅ Yes                   |
| Behavioral Mutation   | 3     | ✅ Yes                   |
| Logical               | 3     | ✅ Yes                   |
| **Total**             | **9** | **✅ 9/9**               |

**Verdict**: The specification correctly rejects all 9 adversarial queries. No specification weaknesses were detected.

---

## Boundary Tests (`boundary_tests.rs`)

All tests target precondition violations and edge-case inputs.

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_empty_seq_precondition` | Calls `lemma_seq_fold_left_append_right` with empty seq, violating `requires s.len() > 0` | ✅ FAIL (precondition not satisfied) |
| 2 | `test_boundary_zero_usize_not_marshalable` | Asserts `!(0usize).is_marshalable()` — but 0 ≤ u64::MAX | ✅ FAIL (assertion failed) |
| 3 | `test_boundary_max_u64_wrong_serialize_length` | Asserts `spec_u64_to_le_bytes(u64::MAX).len() != 8` — length is always 8 | ✅ FAIL (assertion failed) |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

All tests mutate expected output values or relations.

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_mutation_u64_serialize_length_4` | Asserts u64 serialization length is 4 (correct: 8) | ✅ FAIL (assertion failed) |
| 2 | `test_mutation_u64_noninjective` | Asserts `0u64` and `1u64` produce identical serialization (injectivity holds via round-trip) | ✅ FAIL (assertion failed) |
| 3 | `test_mutation_usize_wrong_delegate` | Asserts usize 42 serializes like u64 **43** (correct delegate: u64 42) | ✅ FAIL (assertion failed) |

## Logical Tests (`logical_tests.rs`)

All tests probe unintended properties not guaranteed by the spec.

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_logical_fold_commutative` | Asserts `fold_left` over `[1,2]` ≡ `fold_left` over `[2,1]` (serialization is order-dependent) | ✅ FAIL (assertion failed) |
| 2 | `test_logical_fold_empty_ignores_prefix` | Asserts `fold_left(empty_seq, prefix, f) ≡ Seq::empty()` (correct: returns `prefix`) | ✅ FAIL (assertion failed) |
| 3 | `test_logical_usize_u64_different_serialize` | Asserts usize 42 and u64 42 serialize differently (spec delegates usize→u64, so they're identical) | ✅ FAIL (assertion failed) |

---

## Analysis

The specification is **well-scoped** across all three test dimensions:

1. **Boundary completeness**: Preconditions (`requires s.len() > 0`, `is_marshalable` constraints) correctly guard against invalid inputs. Edge values (0, MAX) are properly handled.

2. **Behavioral correctness**: The combination of `spec_u64_to_le_bytes` with `lemma_auto_spec_u64_to_from_le_bytes` provides sufficient constraints — serialization length (8 bytes), injectivity (via round-trip), and correct delegation (usize→u64) are all enforced.

3. **Logical tightness**: The spec does not accidentally entail order-independence (commutativity of fold), does not confuse the fold base case, and correctly captures the usize-u64 serialization equivalence.

No spec weaknesses (unintended entailments) were found.
