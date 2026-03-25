# Adversarial Test Summary

**Target**: `marshal_ironsht_specific_v__impl2__lemma_serialize_injective.rs`
**Spec under test**: `CKeyHashMap::lemma_serialize_injective` and related `Marshalable` trait specifications

---

## Results Overview

| Test Category          | Tests | All Failed (as expected) |
|------------------------|-------|--------------------------|
| Boundary Tests         | 5     | ✅ 5/5                   |
| Behavioral Mutation    | 4     | ✅ 4/4                   |
| Logical Tests          | 5     | ✅ 5/5                   |
| **Total**              | **14**| ✅ **14/14**             |

All tests correctly **FAILED** verification, meaning the specification rejects all tested unintended properties.

---

## Boundary Tests (`boundary_tests.rs`)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `boundary_test_self_not_marshalable` | Call `lemma_serialize_injective` without `self.is_marshalable()` | ❌ Precondition violation |
| `boundary_test_other_not_marshalable` | Call without `other.is_marshalable()` | ❌ Precondition violation |
| `boundary_test_unequal_serializations` | Call with `ghost_serialize(a) != ghost_serialize(b)` | ❌ Precondition violation |
| `boundary_test_not_prefix_when_view_equal` | Call `lemma_serialization_is_not_a_prefix_of` when `view_equal` holds | ❌ Precondition violation |
| `boundary_test_no_preconditions` | Assert `view_equal` with no preconditions | ❌ Postcondition not satisfied |

**Conclusion**: All preconditions are properly enforced. Invalid inputs are correctly rejected.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

| Test | Mutation | Result |
|------|----------|--------|
| `mutation_test_negate_injectivity` | Negate postcondition: assert `!view_equal` when serializations match | ❌ Postcondition not satisfied |
| `mutation_test_view_equal_from_different_serializations` | Assert `view_equal` from different serializations | ❌ Postcondition not satisfied |
| `mutation_test_prefix_matches_when_not_equal` | Assert prefix DOES match for non-view-equal items | ❌ Postcondition not satisfied |
| `mutation_test_serialize_returns_empty` | Assert serialization is empty for marshalable item | ❌ Postcondition not satisfied |

**Conclusion**: All mutated behaviors are correctly rejected. The spec distinguishes correct from incorrect input-output relations.

---

## Logical Tests (`logical_tests.rs`)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `logical_test_view_equal_implies_to_vec_equal` | `view_equal` ⟹ `to_vec()` equality | ❌ Postcondition not satisfied |
| `logical_test_all_marshalable` | Every `CKeyHashMap` is marshalable | ❌ Postcondition not satisfied |
| `logical_test_marshalability_preserved_by_view_equal` | `is_marshalable` preserved under `view_equal` | ❌ Postcondition not satisfied |
| `logical_test_serialize_eq_without_marshalable` | Serialization equality ⟹ `view_equal` (without marshalability) | ❌ Postcondition not satisfied |
| `logical_test_view_equal_implies_equal_serialization` | `view_equal` ⟹ equal serializations | ❌ Postcondition not satisfied |

**Conclusion**: The spec does not entail any of the tested unintended logical properties.

---

## Semantic Observations

The logical tests reveal an interesting structural property of the spec:

1. **`view()` and `to_vec()` are decoupled**: Both are uninterpreted spec functions on the `external_body` type `CKeyHashMap`. The spec provides no axiom linking them. This means:
   - Two hashmaps with the same abstract view (`self@ === other@`) could theoretically have different `to_vec()` representations
   - Consequently, `view_equal` does not imply equal serializations (since `ghost_serialize` is defined via `to_vec()`)
   - `is_marshalable` is also not preserved under `view_equal`

2. **Reliance on external_body axioms**: The correctness of `lemma_serialize_injective` depends on the trusted `lemma_serialization_is_not_a_prefix_of` axiom (marked `external_body`). The verified proof body only handles the contradiction step.

3. **The spec is sound but potentially incomplete**: While no incorrect properties are entailed (all tests fail), the lack of a `view()` ↔ `to_vec()` connection means the spec cannot prove properties that should intuitively hold, such as "same view implies same serialization." This is a deliberate design choice that pushes these obligations to external_body axioms.

---

## Final Assessment

The specification for `CKeyHashMap::lemma_serialize_injective` is **consistent**: it correctly rejects all 14 adversarial queries across boundary violations, behavioral mutations, and logical over-claims. No unintended properties are entailed.
