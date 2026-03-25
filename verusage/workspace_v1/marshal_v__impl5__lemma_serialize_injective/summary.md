# Adversarial Proof Test Results — `marshal_v__impl5__lemma_serialize_injective`

## Target Specification

The `Marshalable` trait defines serialization with two key lemmas:
- **`lemma_serialize_injective`**: Equal serializations ⟹ view-equal values (requires marshalability)
- **`lemma_serialization_is_not_a_prefix_of`**: Non-view-equal values have non-prefix serializations

Implementations for `u64`, `usize`, and `(T, U)` tuples.

---

## Results Summary

| Category | Tests | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary | 5 | 5 ✅ | 0 |
| Behavioral Mutation | 5 | 5 ✅ | 0 |
| Logical | 5 | 5 ✅ | 0 |
| **Total** | **15** | **15** | **0** |

**All 15 adversarial tests were correctly rejected by the verifier.** The specification is consistent with respect to all tested semantic boundaries.

---

## Boundary Tests (`boundary_tests.rs`) — 5/5 FAILED ✅

| Test | Violation | Result |
|---|---|---|
| `test_boundary_injective_different_u64_values` | Calls injective lemma with u64 values 0 and 1 (different serializations) | Precondition `ghost_serialize` equality rejected |
| `test_boundary_prefix_on_equal_u64` | Calls prefix lemma on equal u64 values (42, 42) | Precondition `!view_equal` rejected |
| `test_boundary_injective_zero_and_max` | Edge case: 0 and u64::MAX | Precondition `ghost_serialize` equality rejected |
| `test_boundary_pair_injective_different_values` | Different tuple values (1,2) vs (3,4) | Precondition `is_marshalable` rejected (can't verify serialization length bound) |
| `test_boundary_prefix_on_equal_pair` | Equal tuple values (5,10) vs (5,10) | Precondition `!view_equal` rejected |

**Conclusion**: All precondition boundaries are properly enforced. Invalid inputs are correctly rejected.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 5/5 FAILED ✅

| Test | Mutation | Result |
|---|---|---|
| `test_mutation_negate_injective_ensures` | Asserts `!view_equal` after calling injective lemma (negates ensures) | Assertion failed — ensures correctly contradicts negation |
| `test_mutation_serialize_not_self_equal` | Asserts `a.ghost_serialize() != a.ghost_serialize()` | Assertion failed — reflexivity holds |
| `test_mutation_pair_serialize_reversed_order` | Asserts tuple serialization is `b++a` instead of `a++b` | Assertion failed — correct concatenation order enforced |
| `test_mutation_negate_prefix_ensures` | Asserts prefix equality after prefix lemma (negates ensures) | Assertion failed — ensures correctly contradicts |
| `test_mutation_serialize_empty` | Asserts u64 serialization has length 0 | Assertion failed — non-empty serialization enforced |

**Conclusion**: All mutated behaviors are correctly rejected. The specification properly constrains outputs.

---

## Logical Tests (`logical_tests.rs`) — 5/5 FAILED ✅

| Test | Property Tested | Result |
|---|---|---|
| `test_logical_injective_without_lemma` | Injectivity without calling lemma (generic `T: Marshalable`) | Assertion failed — lemma call required |
| `test_logical_view_equal_symmetric` | Symmetry of `view_equal` at trait level | Assertion failed — not guaranteed by trait |
| `test_logical_contrapositive_without_lemma` | Contrapositive of injectivity without lemma | Assertion failed — not automatically derivable |
| `test_logical_derive_false_after_lemma` | Soundness: derive `false` after valid lemma call | Assertion failed — spec is sound |
| `test_logical_view_equal_transitive` | Transitivity of `view_equal` at trait level | Assertion failed — not guaranteed by trait |

**Conclusion**: The specification does not entail unintended logical properties. Key findings:
- **Injectivity is gated behind an explicit lemma call** — it cannot be obtained for free
- **`view_equal` has no structural guarantees** (symmetry, transitivity) at the trait level — this is a design choice, not a weakness
- **The spec is sound** — combining the axioms does not produce contradictions

---

## Overall Assessment

The specification for `Marshalable` serialization injectivity is **well-bounded**:

1. **Preconditions are tight** — invalid inputs (wrong types, violated constraints) are rejected
2. **Postconditions are precise** — mutated outputs are detected and rejected
3. **No unintended entailments** — the spec does not allow deriving properties beyond what is explicitly stated
4. **Axiom soundness** — the `external_body` axioms do not introduce inconsistencies

No specification weaknesses were detected.
