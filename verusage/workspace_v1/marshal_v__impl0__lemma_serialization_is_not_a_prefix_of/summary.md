# Adversarial Test Summary: `lemma_serialization_is_not_a_prefix_of`

## Specification Under Test

The `Marshalable` trait's `lemma_serialization_is_not_a_prefix_of` lemma, which states:
- **Requires**: `!self.view_equal(other)` AND `self.ghost_serialize().len() <= other.ghost_serialize().len()`
- **Ensures**: `self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len() as int)`

Tested with concrete implementations for `u64` and `usize`.

---

## Results: All 9 tests FAILED verification ✓ (as expected)

### Boundary Tests (3/3 rejected)

| Test | Target | Result |
|------|--------|--------|
| `test_boundary_equal_u64_values` | Call lemma with equal u64 values (violates `!view_equal`) | ✓ Precondition rejected |
| `test_boundary_equal_usize_values` | Call lemma with equal usize values (violates `!view_equal`) | ✓ Precondition rejected |
| `test_boundary_postcondition_for_equal_values` | Assert postcondition (!=) for identical values | ✓ Assertion rejected |

### Behavioral Mutation Tests (3/3 rejected)

| Test | Target | Result |
|------|--------|--------|
| `test_mutation_equal_serialization` | Assert different u64 values (1, 2) have equal serializations | ✓ Assertion rejected |
| `test_mutation_is_prefix` | Assert serialization IS a prefix (opposite of postcondition) | ✓ Assertion rejected |
| `test_mutation_wrong_serialization_length` | Assert u64 serialization length is 4 (should be 8) | ✓ Assertion rejected |

### Logical Tests (3/3 rejected)

| Test | Target | Result |
|------|--------|--------|
| `test_logical_first_byte_always_differs` | Stronger: different values always differ at byte 0 (false: 1 and 257 share byte 0) | ✓ Assertion rejected |
| `test_logical_serialization_concat_commutative` | Structural: serialization concatenation is commutative | ✓ Assertion rejected |
| `test_logical_last_byte_always_differs` | Stronger: different values always differ at byte 7 (false: 1 and 2 share byte 7) | ✓ Assertion rejected |

---

## Conclusion

The specification is **consistent** — it correctly rejects all 9 adversarial properties:
- **Boundary**: Invalid inputs (equal values) are properly guarded by preconditions.
- **Behavioral**: Mutated output relations (equal serializations, wrong length, prefix assertion) are correctly rejected.
- **Logical**: Stronger-than-specified properties (per-byte distinctness, concatenation commutativity) are not entailed.

No spec weaknesses were detected in these tests.
