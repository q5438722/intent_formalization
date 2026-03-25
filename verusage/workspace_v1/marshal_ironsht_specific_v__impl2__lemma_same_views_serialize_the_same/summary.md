# Adversarial Test Summary

**Target**: `marshal_ironsht_specific_v__impl2__lemma_same_views_serialize_the_same.rs`
**Spec under test**: `lemma_same_views_serialize_the_same` for `CKeyHashMap` (and related `Marshalable` impls)

---

## Results Overview

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (5 errors) |
| `behavioral_mutation_tests.rs` | 4 | ✅ Yes (4 errors) |
| `logical_tests.rs` | 5 | ✅ Yes (5 errors) |

**Total: 14 tests, 14 correctly rejected.**

---

## Boundary Tests (precondition violations)

All 5 tests called `lemma_same_views_serialize_the_same` with inputs violating the `view_equal` precondition. Verus correctly rejected each call with "precondition not satisfied."

| Test | Type | Failure Mode |
|------|------|-------------|
| `test_boundary_ckh_different_views` | CKeyHashMap | Different map views (`!(a@ === b@)`) |
| `test_boundary_u64_different` | u64 | Different integer values (`a != b`) |
| `test_boundary_vec_u8_different` | Vec\<u8\> | Different byte sequences (`a@ !== b@`) |
| `test_boundary_shtkey_different` | SHTKey | Different keys (`a.ukey != b.ukey`) |
| `test_boundary_ckeykv_different_key` | CKeyKV | Different key-value pair keys |

**Conclusion**: The spec correctly rejects invalid (non-view-equal) inputs across all types.

---

## Behavioral Mutation Tests (negated postconditions)

All 4 tests established `view_equal`, called the lemma, then asserted the **negation** of guaranteed postconditions. Verus correctly rejected each assertion.

| Test | Mutation | Failure Mode |
|------|----------|-------------|
| `test_mutation_ckh_negate_serialize` | CKeyHashMap | Assert serializations differ (negates ensures) |
| `test_mutation_ckh_negate_marshalable` | CKeyHashMap | Assert marshalable status differs (negates ensures) |
| `test_mutation_u64_negate_serialize` | u64 | Assert u64 serializations differ |
| `test_mutation_ckh_negate_first_byte` | CKeyHashMap | Assert first byte of serialization differs |

**Conclusion**: The spec correctly enforces both postconditions (serialization equality and marshalable equivalence).

---

## Logical Tests (properties NOT guaranteed)

All 5 tests asserted properties that the spec does NOT explicitly guarantee. Verus correctly refused to verify them.

| Test | Unwarranted Property | Failure Mode |
|------|---------------------|-------------|
| `test_logical_converse_ckh` | Same serialization → same view | Converse of the lemma is not provable |
| `test_logical_injectivity_ckh` | Different views → different serializations | Injectivity not guaranteed |
| `test_logical_opaque_constant` | Opaque constant value accessible | `ckeyhashmap_max_serialized_size` is `#[opaque]` |
| `test_logical_empty_map_marshalable` | Empty view → marshalable | `is_marshalable` depends on uninterpreted `spec_to_vec` |
| `test_logical_no_lemma_to_vec_len` | view_equal → to_vec length equality (without lemma) | Relationship requires `lemma_to_vec_view` |

**Conclusion**: The spec does not over-entail — it correctly refuses to derive properties beyond what is stated.

---

## Overall Assessment

The specification for `lemma_same_views_serialize_the_same` is **consistent** with respect to the tested properties:

1. **Boundary integrity**: Invalid inputs are rejected across all type implementations.
2. **Behavioral correctness**: Incorrect output mutations are rejected; postconditions are enforced.
3. **Logical tightness**: The spec does not entail unstated properties (converse, injectivity, opaque access, structural assumptions).

No specification weaknesses were detected — all 14 adversarial tests were correctly rejected.
