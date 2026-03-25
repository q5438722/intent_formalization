# Adversarial Proof Test Summary

**Target**: `same_view_same_marshalable(x: &CSingleMessage, y: &CSingleMessage)`
- **Requires**: `x@ == y@` (abstract views are equal)
- **Ensures**: `x.is_marshalable() == y.is_marshalable()` (marshalability status is the same)

---

## Results Overview

| Test File | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| `boundary_tests.rs` | 5 | 5 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 5 | 5 ✅ | 0 |
| `logical_tests.rs` | 6 | 6 ✅ | 0 |
| **Total** | **16** | **16 ✅** | **0** |

All 16 adversarial tests were correctly **rejected** by the verifier.

---

## (1) Boundary Tests — Precondition Violations

All 5 tests violated `requires x@ == y@` and were rejected with "precondition not satisfied":

| Test | Failure Mode | Result |
|---|---|---|
| `test_boundary_different_ack_seqno` | Ack(0) vs Ack(1) — different seqnos | FAILED ✅ |
| `test_boundary_ack_vs_invalid` | Ack vs InvalidMessage — different variants | FAILED ✅ |
| `test_boundary_max_vs_zero` | Ack(MAX) vs Ack(0) — edge-case values | FAILED ✅ |
| `test_boundary_off_by_one` | Ack(42) vs Ack(43) — off-by-one | FAILED ✅ |
| `test_boundary_assume_different_views` | Parametric: assume(x@ != y@) | FAILED ✅ |

**Conclusion**: The precondition `x@ == y@` correctly rejects all invalid inputs.

## (2) Behavioral Mutation Tests — Mutated Postconditions

All 5 tests started from valid inputs (x@ == y@) and asserted mutated (incorrect) postconditions:

| Test | Mutation | Result |
|---|---|---|
| `test_mutation_negate_postcondition` | Assert `is_marshalable() !=` (negated equality) | FAILED ✅ |
| `test_mutation_asymmetric_marshalability` | Assert x marshalable, y not | FAILED ✅ |
| `test_mutation_both_not_marshalable` | Assert both not marshalable (Ack should be) | FAILED ✅ |
| `test_mutation_invalid_not_marshalable` | Assert InvalidMessage not marshalable (it is) | FAILED ✅ |
| `test_mutation_different_serialization` | Assert ghost_serialize differs despite same view | FAILED ✅ |

**Conclusion**: The postcondition `x.is_marshalable() == y.is_marshalable()` correctly rejects all behavioral mutations. The open specs for `is_marshalable` on concrete variants (Ack, InvalidMessage) are also tight enough to reject false marshalability claims.

## (3) Logical Tests — Unentailed Properties

All 6 tests attempted to derive properties NOT guaranteed by the specification:

| Test | Unentailed Property | Result |
|---|---|---|
| `test_logical_reverse_implication` | marshalability equality → view equality | FAILED ✅ |
| `test_logical_universal_without_lemma` | ∀ x,y: x@==y@ → marshalable equality (no lemma) | FAILED ✅ |
| `test_logical_no_contradiction_from_axioms` | `external_body` axioms → `false` | FAILED ✅ |
| `test_logical_serialize_not_in_ensures` | same_view_same_marshalable → serialize equality | FAILED ✅ |
| `test_logical_structural_equality` | x@ == y@ → x === y (structural equality) | FAILED ✅ |
| `test_logical_view_equal_implies_serialize` | view_equal_spec + view_equal → serialize equality | FAILED ✅ |

**Conclusion**: The spec does not leak unintended properties:
- The reverse direction (marshalability → views) is correctly not derivable.
- The `external_body` axioms (`view_equal_spec`, `lemma_same_views_serialize_the_same`) are not contradictory.
- `ghost_serialize` equality is NOT derivable from `same_view_same_marshalable` alone (ensures only covers `is_marshalable`).
- Structural equality is not derivable from view equality (correct for types with potentially different representations).
- The universal quantified version cannot be proven without explicit lemma invocation per instance.

---

## Overall Assessment

The specification of `same_view_same_marshalable` is **consistent**: it correctly rejects invalid inputs, incorrect behaviors, and unintended logical inferences across all 16 adversarial queries. No spec weaknesses were detected.

**Notable design observations**:
- The ensures clause is deliberately narrow (`is_marshalable` equality only), even though the internal proof also establishes `ghost_serialize` equality. This is appropriate modular design — callers who need serialization equality should call `lemma_same_views_serialize_the_same` directly.
- The `external_body` axioms are consistent (cannot derive `false`).
