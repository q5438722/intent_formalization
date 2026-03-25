# Adversarial Proof Test Summary

**Target**: `marshal_v__impl5__lemma_serialization_is_not_a_prefix_of.rs`
**Specification**: `Marshalable` trait with `lemma_serialization_is_not_a_prefix_of`, `lemma_same_views_serialize_the_same`, `lemma_view_equal_symmetric`

---

## Results Overview

| Category              | Total | Failed (expected) | Passed (unexpected) |
|-----------------------|-------|--------------------|----------------------|
| Boundary Tests        | 5     | 5 ✅               | 0                    |
| Behavioral Mutation   | 5     | 5 ✅               | 0                    |
| Logical Tests         | 5     | 4 ✅               | **1 ⚠️ (LT4)**      |
| **Total**             | **15**| **14**             | **1**                |

---

## Boundary Tests (`boundary_tests.rs`) — All 5 FAILED ✅

All precondition violations were properly rejected:

| Test | Description | Violated Precondition | Result |
|------|-------------|-----------------------|--------|
| BT1  | `lemma_serialization_is_not_a_prefix_of` with equal u64 (42, 42) | `!self.view_equal(other)` | FAILED ✅ |
| BT2  | `lemma_same_views_serialize_the_same` with different u64 (10, 20) | `self.view_equal(other)` | FAILED ✅ |
| BT3  | `lemma_same_views_serialize_the_same` on partially matching tuple ((1,2), (1,3)) | `self.view_equal(other)` | FAILED ✅ |
| BT4  | `lemma_serialization_is_not_a_prefix_of` with equal tuples ((7,8), (7,8)) | `!self.view_equal(other)` | FAILED ✅ |
| BT5  | `lemma_serialization_is_not_a_prefix_of` with zero u64 (0, 0) | `!self.view_equal(other)` | FAILED ✅ |

**Conclusion**: The spec correctly guards all lemmas with appropriate preconditions.

---

## Behavioral Mutation Tests (`behavioral_tests.rs`) — All 5 FAILED ✅

All mutated postconditions were properly rejected:

| Test | Description | Mutated Property | Result |
|------|-------------|------------------|--------|
| MT1  | Assert serialization IS a prefix (contradicts `not_a_prefix` postcondition) | Negated `!=` to `=~=` | FAILED ✅ |
| MT2  | Assert same-view values have DIFFERENT serializations | Negated `==` to `!==` | FAILED ✅ |
| MT3  | Assert `view_equal` is NOT symmetric | Negated symmetry guarantee | FAILED ✅ |
| MT4  | Assert same-view tuples have DIFFERENT marshalability | Negated marshalability equality | FAILED ✅ |
| MT5  | Assert tuple serialization IS a prefix (contradicts postcondition) | Negated `!=` to `=~=` | FAILED ✅ |

**Conclusion**: The spec correctly rejects all behavioral mutations. Postconditions are strong enough to exclude incorrect output relations.

---

## Logical Tests (`logical_tests.rs`) — 4 FAILED ✅, 1 PASSED ⚠️

| Test | Description | Property Tested | Result |
|------|-------------|-----------------|--------|
| LT1  | First byte always differs for non-equal u64s (256 vs 512) | Stronger inequality | FAILED ✅ |
| LT2  | Tuple serialization is commutative: (a,b) ≡ (b,a) | Structural assumption | FAILED ✅ |
| LT3  | All 8 bytes differ for non-equal u64s (0 vs 1) | Stronger inequality | FAILED ✅ |
| LT4  | `view_equal(x, x)` is true (reflexivity) | Reflexivity (not in spec) | **PASSED ⚠️** |
| LT5  | Same serialization length ⟹ same serialization content | Cross-function misuse | FAILED ✅ |

---

## ⚠️ Finding: Spec Weakness — Missing Reflexivity Axiom (LT4)

**Test LT4 passed unexpectedly**, revealing a specification gap:

- **What happened**: The test asserts `x.view_equal(&x)` for a `u64`. This property (reflexivity) is NOT guaranteed by the abstract trait specification — only **symmetry** is guaranteed via `lemma_view_equal_symmetric`.

- **Why it passed**: For concrete `u64`, `view_equal` is defined as `self@ === other@`. Since `===` is reflexive (`x@ === x@` is trivially true), Verus can prove reflexivity directly from the concrete definition.

- **Why this matters**: The abstract `Marshalable` trait specification does not include a reflexivity axiom for `view_equal`. This means:
  1. An implementor could hypothetically define `view_equal(x, x) = false`, violating the expected equivalence-relation semantics.
  2. Code that relies on `view_equal` being reflexive does so without formal justification from the spec.
  3. The spec guarantees symmetry but not reflexivity or transitivity — it does not enforce that `view_equal` is an equivalence relation.

- **Recommendation**: Add a `lemma_view_equal_reflexive` to the `Marshalable` trait:
  ```rust
  proof fn lemma_view_equal_reflexive(&self)
      ensures self.view_equal(self)
  ```
  Or strengthen the existing specification to require `view_equal` to be an equivalence relation.

---

## Summary

The specification is **mostly robust**:
- All 5 boundary tests correctly reject invalid inputs (precondition violations).
- All 5 behavioral mutation tests correctly reject incorrect output relations.
- 4 of 5 logical tests correctly reject unintended reasoning.

**One spec weakness identified**: The trait lacks a reflexivity guarantee for `view_equal`, which is provable only because of concrete type definitions — not from the abstract specification itself.
