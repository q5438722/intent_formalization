# Adversarial Test Summary: `lemma_serialization_is_not_a_prefix_of`

## Target Specification

The `Marshalable` trait defines a prefix-freeness property for serialization:

- **Requires**: `!self.view_equal(other)` ∧ `self.ghost_serialize().len() <= other.ghost_serialize().len()`
- **Ensures**: `self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len())`

Implementations exist for `u64`, `usize`, `Vec<u8>`, and `Vec<T>`.

---

## Test Results

### Boundary Tests (`boundary_tests.rs`) — 5/5 FAILED ✅

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_equal_u64_values` (val=42) | Precondition `!view_equal` violated | FAILED ✅ |
| 2 | `test_boundary_zero_u64_values` (val=0) | Precondition `!view_equal` violated (edge: zero) | FAILED ✅ |
| 3 | `test_boundary_equal_usize_values` (val=100) | Precondition `!view_equal` violated (usize type) | FAILED ✅ |
| 4 | `test_boundary_conclusion_for_equal_values` | Asserts ensures for equal values (false conclusion) | FAILED ✅ |
| 5 | `test_boundary_max_u64_equal` (val=MAX) | Precondition `!view_equal` violated (edge: u64::MAX) | FAILED ✅ |

**Interpretation**: The spec correctly rejects all invalid inputs. The `!view_equal` precondition prevents calling the lemma on equal values at edge cases (0, typical, MAX) and across types (u64, usize).

---

### Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 4/4 FAILED ✅

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_assert_same_serialization` | Assert `serialize(a) == serialize(b)` for a≠b | FAILED ✅ |
| 2 | `test_mutation_negate_ensures` | Assert serialization IS a prefix (negated ensures) | FAILED ✅ |
| 3 | `test_mutation_assert_false_from_axiom` | Assert `false` after calling lemma (consistency check) | FAILED ✅ |
| 4 | `test_mutation_assert_view_equal_for_different` | Assert `view_equal(5, 10)` (mutated relation) | FAILED ✅ |

**Interpretation**: The spec correctly rejects all incorrect behaviors. The ensures clause is strong enough to prevent asserting equality of different serializations, negating the prefix-freeness conclusion, or deriving inconsistency from the axiom.

---

### Logical Tests (`logical_tests.rs`) — 4/4 FAILED ✅

| # | Test | Unstated Property | Result |
|---|------|-------------------|--------|
| 1 | `test_logical_injectivity_without_lemma` | Same serialization → view_equal (converse) | FAILED ✅ |
| 2 | `test_logical_prefix_free_without_lemma` | Prefix-freeness without calling lemma | FAILED ✅ |
| 3 | `test_logical_all_bytes_differ` | All byte positions differ (stronger than needed) | FAILED ✅ |
| 4 | `test_logical_reverse_without_lemma` | Reverse-direction prefix-freeness without 2nd lemma call | FAILED ✅ |

**Interpretation**: The spec correctly avoids entailing unstated properties:
- **Injectivity** (serialization → view_equal) is NOT freely available — the spec only provides the forward direction.
- **Prefix-freeness** requires explicitly invoking the lemma; it is not ambient.
- **Pointwise inequality** (stronger than prefix-freeness) is correctly rejected.
- **Symmetry** of prefix-freeness for the reversed pair is not automatically derivable from a single lemma call.

---

## Overall Assessment

**All 13 adversarial tests FAILED as expected.** The specification:

1. **Correctly guards preconditions**: Invalid inputs (equal values, edge cases) are rejected by the `!view_equal` check.
2. **Correctly constrains behavior**: Mutated output relationships (same serialization, negated ensures) are rejected.
3. **Does not over-entail**: Properties not explicitly guaranteed (injectivity, pointwise difference, symmetry) are not freely derivable.
4. **Is consistent**: `assert(false)` after calling the lemma with valid inputs is rejected (no inconsistency detected).

### Potential Spec Observations

- The lemma's preconditions do **not** require `is_marshalable()`. This is currently benign for `u64` (always marshalable) but could be a concern for `usize` or `Vec<T>` if called on non-marshalable values.
- The `u64` and `usize` implementations use `#[verifier::external_body]` (axiomatized, not proven). While no inconsistency was detected, these are trusted assumptions.
