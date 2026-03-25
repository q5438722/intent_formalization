# Test Execution Summary

**Target**: `marshal_ironsht_specific_v__impl2__lemma_serialization_is_not_a_prefix_of.rs`
**Spec under test**: `Marshalable::lemma_serialization_is_not_a_prefix_of` for `CKeyHashMap`

```
requires: !self.view_equal(other) ∧ self.ghost_serialize().len() <= other.ghost_serialize().len()
ensures:  self.ghost_serialize() != other.ghost_serialize().subrange(0, self.ghost_serialize().len())
```

---

## Results: ALL 9 TESTS FAILED ✓ (spec correctly rejects all undesirable properties)

### Boundary Tests (3/3 FAILED)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_view_equal` | Call lemma with `a@ === b@` (view-equal maps) | ✓ FAILED: precondition `!self.view_equal(other)` not satisfied |
| `test_boundary_wrong_length_order` | Call lemma with `a.serialize.len > b.serialize.len` | ✓ FAILED: precondition `self.len() <= other.len()` not satisfied |
| `test_boundary_self_identity` | Call lemma with `a` against itself | ✓ FAILED: precondition `!self.view_equal(other)` not satisfied |

**Conclusion**: The spec properly guards its preconditions — invalid inputs are rejected.

### Behavioral Mutation Tests (3/3 FAILED)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_mutation_assert_is_prefix` | After calling lemma, assert serialization IS a prefix | ✓ FAILED: assertion contradicts postcondition |
| `test_mutation_assert_equal_serialization` | Assert equal serialization for non-view-equal same-length maps | ✓ FAILED: assertion contradicts postcondition |
| `test_mutation_stronger_pointwise_differ` | Assert serializations differ at EVERY byte position | ✓ FAILED: overly strong property not entailed |

**Conclusion**: The spec correctly rejects mutated output relations — both negated postconditions and over-strengthened variants.

### Logical Tests (3/3 FAILED)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_logical_determinism` | Assert view-equal maps produce same serialization (without helper lemma) | ✓ FAILED: not provable without `lemma_to_vec_view` + reasoning about `spec_to_vec` |
| `test_logical_reverse_nonprefix` | Assert non-prefix in reverse direction without calling lemma symmetrically | ✓ FAILED: spec only guarantees non-prefix for `self.len <= other.len` |
| `test_logical_unbounded_serialization_length` | Assert serialization length bounded without `is_marshalable` | ✓ FAILED: bound only applies when `is_marshalable` holds, and `ckeyhashmap_max_serialized_size` is opaque |

**Conclusion**: The spec does not entail unintended logical properties — determinism requires explicit lemma calls, directionality is enforced, and size bounds require `is_marshalable`.

---

## Overall Assessment

The specification for `CKeyHashMap::lemma_serialization_is_not_a_prefix_of` is **consistent** across all three query dimensions:

1. **Boundary consistency**: Preconditions properly guard against invalid inputs (view-equal maps, wrong length ordering, self-referential calls).
2. **Behavioral consistency**: Postconditions correctly constrain output relations — both negations and strengthened variants are rejected.
3. **Logical consistency**: The spec does not leak unintended entailments — properties like determinism, directional symmetry, and unconditional size bounds require explicit reasoning not derivable from the spec alone.

No specification weaknesses were detected.
