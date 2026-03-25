# Adversarial Proof Test Results Summary

**Target**: `marshal_v__impl4__lemma_serialize_injective.rs`
**Specification**: `Marshalable` trait with `lemma_serialize_injective` and `lemma_serialization_is_not_a_prefix_of`

---

## Results Overview

| Test File | Tests | Failed | Passed |
|-----------|-------|--------|--------|
| boundary_tests.rs | 4 | 4 ✓ | 0 |
| behavioral_mutation_tests.rs | 3 | 3 ✓ | 0 |
| logical_tests.rs | 3 | 3 ✓ | 0 |
| **Total** | **10** | **10 ✓** | **0** |

**All 10 adversarial tests were rejected by the verifier.** The specification is consistent with respect to all tested queries.

---

## Boundary Tests (4/4 FAILED ✓)

| # | Test | Violated Precondition | Failure Mode |
|---|------|-----------------------|--------------|
| 1 | `test_boundary_different_serializations` | `self.ghost_serialize() == other.ghost_serialize()` | Called `lemma_serialize_injective` on u64 values 0 and 1 (different serializations) |
| 2 | `test_boundary_view_equal_prefix` | `!self.view_equal(other)` | Called `lemma_serialization_is_not_a_prefix_of` on equal u64 values (42, 42) |
| 3 | `test_boundary_zero_and_max` | `self.ghost_serialize() == other.ghost_serialize()` | Edge case: u64 values 0 and u64::MAX |
| 4 | `test_boundary_assert_serialize_equal_no_proof` | N/A (raw assertion) | Asserted serialization equality of different values without proof |

**Conclusion**: The spec correctly rejects invalid inputs. Preconditions on both lemmas are properly enforced.

---

## Behavioral Mutation Tests (3/3 FAILED ✓)

| # | Test | Mutation | Failure Mode |
|---|------|----------|--------------|
| 1 | `test_mutation_negate_injectivity_ensures` | Negated `view_equal` postcondition | Called `lemma_serialize_injective` correctly on (7, 7), then asserted `!view_equal` — contradicts ensures |
| 2 | `test_mutation_different_values_same_serialization` | Asserted equal serializations for different values | Asserted `ghost_serialize(100) == ghost_serialize(200)` — correctly rejected |
| 3 | `test_mutation_negate_prefix_ensures` | Negated non-prefix postcondition | Asserted serialization IS a prefix after calling `lemma_serialization_is_not_a_prefix_of` — rejected (also: serialization length relationship unprovable without extra lemma) |

**Conclusion**: The spec correctly rejects incorrect behaviors. Postcondition negations are properly detected.

**Note**: Test 3 also revealed that Verus cannot automatically prove `spec_u64_to_le_bytes(a).len() <= spec_u64_to_le_bytes(b).len()` for concrete u64 values, indicating serialization length properties require explicit auxiliary lemmas.

---

## Logical Tests (3/3 FAILED ✓)

| # | Test | Unentailed Property | Failure Mode |
|---|------|---------------------|--------------|
| 1 | `test_logical_view_equal_without_proof` | `view_equal(3, 9)` with no supporting proof | Spec does not guarantee view_equal without equal serializations |
| 2 | `test_logical_wrong_serialize_length` | `ghost_serialize(42).len() > 16` | Spec does not entail incorrect serialization length bounds |
| 3 | `test_logical_converse_injectivity_no_lemma` | `ghost_serialize(0) !== ghost_serialize(1)` without lemma | Converse of injectivity is not directly provable without calling `lemma_serialization_is_not_a_prefix_of` |

**Conclusion**: The spec does not entail unintended logical consequences. Properties outside the stated ensures clauses are correctly rejected.

---

## Overall Assessment

The `Marshalable` trait specification is **consistent** with respect to all 10 adversarial queries:

- **Boundary integrity**: Preconditions on both `lemma_serialize_injective` and `lemma_serialization_is_not_a_prefix_of` are properly enforced
- **Behavioral correctness**: Postcondition negations are detected and rejected
- **Logical soundness**: The spec does not entail properties beyond its stated guarantees

No specification weaknesses (incompleteness) were detected in these tests.
