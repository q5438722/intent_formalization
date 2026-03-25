# Test Summary: `marshal_v__impl0__lemma_serialize_injective`

## Target Specification

The `Marshalable` trait defines serialization injectivity:
- **Preconditions**: `self.is_marshalable()`, `other.is_marshalable()`, `self.ghost_serialize() == other.ghost_serialize()`
- **Postcondition**: `self.view_equal(other)`

Implementations: `u64` (proven) and `usize` (trusted via `external_body`).

---

## Results: All 15 tests FAILED verification ✅

The specification correctly rejects all adversarial queries.

### Boundary Tests (5/5 failed)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_violate_serialize_eq` | Precondition violated: `ghost_serialize(0) != ghost_serialize(1)` | ✅ FAILED |
| `test_boundary_zero_vs_max` | Edge case: `view_equal(0, MAX)` without proof | ✅ FAILED |
| `test_boundary_max_adjacent` | Precondition violated: `MAX` vs `MAX-1` different serializations | ✅ FAILED |
| `test_boundary_usize_distinct` | Precondition violated: usize 10 vs 20 different serializations | ✅ FAILED |
| `test_boundary_only_self_marshalable` | Precondition violated: `other.is_marshalable()` not met (generic) | ✅ FAILED |

### Behavioral Mutation Tests (5/5 failed)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_mutation_negate_postcondition` | Negated postcondition: same values but assert `!view_equal` | ✅ FAILED |
| `test_mutation_wrong_serialize_equality` | Mutated behavior: assert `serialize(0) == serialize(1)` | ✅ FAILED |
| `test_mutation_strengthen_postcondition` | Strengthened postcondition: `a@ == b@ + 1` after lemma | ✅ FAILED |
| `test_mutation_different_values_view_equal` | Mutated claim: `view_equal(100, 200)` | ✅ FAILED |
| `test_mutation_assert_unequal_after_lemma` | Mutated postcondition: `a@ != b@` after valid lemma call | ✅ FAILED |

### Logical Tests (5/5 failed)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_logical_converse_injectivity` | Converse not entailed: `view_equal ⇒ same serialization` (trait-level) | ✅ FAILED |
| `test_logical_contrapositive_without_lemma` | Contrapositive not derivable without calling lemma (trait-level) | ✅ FAILED |
| `test_logical_reflexivity` | Reflexivity of `view_equal` not guaranteed (trait-level) | ✅ FAILED |
| `test_logical_wrong_serialize_length` | Wrong structural assumption: `len == 4` instead of 8 for u64 | ✅ FAILED |
| `test_logical_transitivity` | Transitivity of `view_equal` not guaranteed (trait-level) | ✅ FAILED |

---

## Conclusion

The specification is **consistent** with respect to all tested semantic queries:
- **Preconditions are enforced**: Invalid inputs (different serializations, non-marshalable values) are correctly rejected.
- **Postconditions are precise**: Mutated or negated output claims are rejected.
- **Logical boundaries are sound**: The spec does not admit unintended reasoning (converse injectivity, reflexivity, transitivity of `view_equal` are not provable at the trait level).

**Note**: The `usize` implementation's `lemma_serialize_injective` uses `external_body` (trusted without proof). While the preconditions are still checked at call sites, the proof body is not mechanically verified.
