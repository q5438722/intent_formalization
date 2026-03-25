# Adversarial Proof Test Summary

**Target**: `marshal_v__impl4__lemma_serialization_is_not_a_prefix_of.rs`
**Specification under test**: `Marshalable` trait — `lemma_serialization_is_not_a_prefix_of`, `lemma_same_views_serialize_the_same`, `lemma_view_equal_symmetric`, `choose_smallest`, `some_differing_index_for_unequal_seqs`

---

## Results Overview

| Category | Tests | All Failed (Expected) |
|---|---|---|
| Boundary Tests | 4 | ✅ Yes (4/4) |
| Behavioral Mutation Tests | 3 | ✅ Yes (3/3) |
| Logical Tests | 3 | ✅ Yes (3/3) |
| **Total** | **10** | **✅ 10/10** |

---

## Boundary Tests (`boundary_tests.rs`)

All 4 tests correctly **failed** due to precondition violations:

| # | Test | Violated Precondition | Result |
|---|---|---|---|
| 1 | `test_boundary_equal_u64_not_prefix` | `!self.view_equal(other)` — equal u64s (42, 42) | ❌ FAILED (precondition) |
| 2 | `test_boundary_different_u64_same_views` | `self.view_equal(other)` — different u64s (1, 2) | ❌ FAILED (precondition) |
| 3 | `test_boundary_choose_smallest_impossible` | `exists \|i\| ... && p(i)` — always-false predicate | ❌ FAILED (precondition) |
| 4 | `test_boundary_equal_seqs_differing_index` | `s1 != s2` — identical sequences | ❌ FAILED (precondition) |

**Conclusion**: The specification correctly rejects all invalid inputs. Preconditions are tight.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

All 3 tests correctly **failed** — mutated postconditions were rejected:

| # | Test | Mutation | Result |
|---|---|---|---|
| 1 | `test_mutation_assert_is_prefix` | Assert serialization IS a prefix (negates `!=` postcondition) | ❌ FAILED (precondition + assertion) |
| 2 | `test_mutation_same_views_different_length` | Assert serialization lengths differ after `lemma_same_views_serialize_the_same` | ❌ FAILED (assertion) |
| 3 | `test_mutation_view_equal_asymmetric` | Assert `view_equal(a,b) != view_equal(b,a)` after symmetry lemma | ❌ FAILED (assertion) |

**Note on Test 1**: Verus could not prove the length precondition (`self.ghost_serialize().len() <= other.ghost_serialize().len()`) for the lemma call without the broadcast lemma `lemma_auto_spec_u64_to_from_le_bytes`. This caused a double failure (precondition + assertion). This is still correct behavior — the specification does not make serialization length facts freely available without the appropriate lemma.

**Conclusion**: The specification correctly rejects incorrect behavioral claims. Postconditions are strong enough to exclude mutated behaviors.

---

## Logical Tests (`logical_tests.rs`)

All 3 tests correctly **failed** — non-entailed properties were rejected:

| # | Test | Unentailed Property | Result |
|---|---|---|---|
| 1 | `test_logical_injectivity_without_lemma` | `∀ a b : u64. a ≠ b ⟹ serialize(a) ≠ serialize(b)` without calling any lemma | ❌ FAILED (assertion) |
| 2 | `test_logical_differing_index_is_first` | `some_differing_index` returns the **first** differing index (spec only guarantees **some**) | ❌ FAILED (assertion) |
| 3 | `test_logical_different_values_different_ser_lengths` | Non-equal u64s have different serialization lengths (false: all u64s serialize to 8 bytes) | ❌ FAILED (assertion) |

**Key Observations**:
- **Test 1** confirms that serialization injectivity for u64 is not freely available — it requires explicitly calling `lemma_serialization_is_not_a_prefix_of` or the byte conversion broadcast lemma. This is good: the spec does not leak implementation details.
- **Test 2** confirms that `some_differing_index_for_unequal_seqs` does not over-specify — it only guarantees existence of a differing index, not minimality. This keeps the spec appropriately abstract.
- **Test 3** confirms the spec does not claim different values produce different-length serializations (which would be false for fixed-width types like u64).

**Discarded test**: An initial test asserting `∀ x : usize. x.is_marshalable()` **passed** verification. This is because Verus assumes `usize::BITS ≤ 64`, making all usize values fit in u64. This is a valid Verus axiom, not a spec weakness.

---

## Overall Assessment

The specification for `Marshalable` and its associated lemmas is **well-constrained**:

1. **Preconditions are tight**: Invalid inputs are correctly rejected at every entry point.
2. **Postconditions are strong enough**: Mutated behavioral claims are rejected.
3. **Logical boundaries are sound**: The spec does not entail properties it shouldn't — serialization injectivity requires explicit lemma invocation, helper functions don't over-specify, and type-specific properties are not leaked.

No spec weaknesses (incompleteness issues) were found in this analysis.
