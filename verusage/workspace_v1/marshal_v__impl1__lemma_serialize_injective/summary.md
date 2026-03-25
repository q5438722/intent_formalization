# Adversarial Test Results: `marshal_v__impl1__lemma_serialize_injective`

## Target Specification

The `Marshalable` trait defines serialization injectivity:
- **Preconditions**: `self.is_marshalable()`, `other.is_marshalable()`, `self.ghost_serialize() == other.ghost_serialize()`
- **Postcondition**: `self.view_equal(other)`

Implementations tested: `u64` (all values marshalable, `spec_u64_to_le_bytes`) and `usize` (marshalable if `≤ u64::MAX`).

---

## Results Summary

| Category | Test | Expected | Actual | Status |
|----------|------|----------|--------|--------|
| **Boundary** | `test_boundary_different_serializations` (0 vs 1) | FAIL | FAIL (precondition violated) | ✅ |
| **Boundary** | `test_boundary_assert_view_equal_no_proof` (100 vs 200) | FAIL | FAIL (assertion failed) | ✅ |
| **Boundary** | `test_boundary_max_vs_zero` (MAX vs 0) | FAIL | FAIL (precondition violated) | ✅ |
| **Boundary** | `test_boundary_adjacent_values` (255 vs 256) | FAIL | FAIL (precondition violated) | ✅ |
| **Boundary** | `test_boundary_usize_wrong_view_equal` (0usize vs 1usize) | FAIL | FAIL (assertion failed) | ✅ |
| **Mutation** | `test_mutation_distinct_values_view_equal` (1 vs 2) | FAIL | FAIL (assertion failed) | ✅ |
| **Mutation** | `test_mutation_same_value_different_serialize` (42 vs 42) | FAIL | FAIL (assertion failed) | ✅ |
| **Mutation** | `test_mutation_negate_ensures_clause` | FAIL | FAIL (assertion failed) | ✅ |
| **Mutation** | `test_mutation_wrong_serialize_length` (len==4) | FAIL | FAIL (assertion failed) | ✅ |
| **Mutation** | `test_mutation_usize_same_serialize` (0usize vs 1usize) | FAIL | FAIL (assertion failed) | ✅ |
| **Logical** | `test_logical_all_serialize_same` (0 vs 1) | FAIL | FAIL (assertion failed) | ✅ |
| **Logical** | `test_logical_view_equal_not_reflexive` | FAIL | FAIL (assertion failed) | ✅ |
| **Logical** | `test_logical_exists_collision` | FAIL | FAIL (assertion failed) | ✅ |
| **Logical** | `test_logical_symmetry_beyond_spec` | FAIL | **PASS** (verified) | ⚠️ FINDING |
| **Logical** | `test_logical_converse_of_lemma` | FAIL | **PASS** (verified) | ⚠️ FINDING |
| **Logical** | `test_logical_fixed_serialize_length` (len==8) | FAIL | FAIL (assertion failed) | ✅ |

**Totals**: 14/16 correctly rejected, **2 unexpected passes** (spec weaknesses)

---

## Findings

### ⚠️ Finding 1: Symmetry of `view_equal` derivable beyond spec guarantee

**Test**: `test_logical_symmetry_beyond_spec`
**Property**: After calling `a.lemma_serialize_injective(&b)`, which ensures `a.view_equal(&b)`, the test also asserts `b.view_equal(&a)` — and Verus verifies it.

**Analysis**: The trait's `lemma_serialize_injective` only ensures `self.view_equal(other)`, NOT `other.view_equal(self)`. The symmetry is derivable because `u64::view_equal` is defined as `self@ === other@`, and `===` is inherently symmetric. This is an **implicit semantic guarantee** not stated in the trait contract. A different `Marshalable` implementation could define `view_equal` asymmetrically and still satisfy the trait spec.

**Risk**: Clients may depend on symmetry of `view_equal` without the trait guaranteeing it.

### ⚠️ Finding 2: Converse of the lemma is provable

**Test**: `test_logical_converse_of_lemma`
**Property**: `a.view_equal(&b) ==> a.ghost_serialize() == b.ghost_serialize()` verifies for `u64`.

**Analysis**: The spec only states the forward direction: `serialize_equal ⇒ view_equal` (injectivity). The converse (`view_equal ⇒ serialize_equal`) is NOT part of the trait specification. However, for the `u64` implementation, both `view_equal` and `ghost_serialize` are `open spec fn`, so Verus can transparently reason: if `self@ === other@` (same int value), then `spec_u64_to_le_bytes(*self) == spec_u64_to_le_bytes(*other)`.

**Risk**: This over-specifies the `u64` implementation. Clients could depend on the converse property, which would break if `view_equal` or `ghost_serialize` were made `closed spec fn` or if the implementation changed.

### ✅ Notable Negative Finding: Serialization length NOT derivable

**Test**: `test_logical_fixed_serialize_length`
**Property**: `ghost_serialize(0u64).len() == 8` — this FAILS verification.

**Analysis**: Despite `spec_u64_to_le_bytes` producing 8-byte sequences, Verus cannot automatically derive the length property without explicit invocation of vstd lemmas (e.g., `lemma_auto_spec_u64_to_from_le_bytes`). This shows the spec does NOT inadvertently leak structural information about serialization format.

---

## Conclusion

The specification is **largely consistent**: it correctly rejects all invalid inputs (boundary tests), incorrect behaviors (mutation tests), and most unintended logical inferences. However, **two logical properties** are entailed that go beyond the trait-level specification:

1. **Symmetry of `view_equal`** — derivable from the `open spec fn` definition
2. **Converse of injectivity** — derivable from transparent spec functions

These represent **over-specification at the implementation level**: the `open spec fn` modifiers expose implementation details that allow reasoning beyond the trait contract. This could be mitigated by using `closed spec fn` if the intent is to restrict clients to only the trait-level guarantees.
