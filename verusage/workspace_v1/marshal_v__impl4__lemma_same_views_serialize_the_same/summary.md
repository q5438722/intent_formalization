# Test Summary: `marshal_v__impl4__lemma_same_views_serialize_the_same`

## Target Specification

The `Marshalable` trait defines `lemma_same_views_serialize_the_same`:
- **Requires**: `self.view_equal(other)`
- **Ensures**: `self.is_marshalable() == other.is_marshalable()` ∧ `self.ghost_serialize() == other.ghost_serialize()`

Implementations tested: `u64`, `usize`, `Vec<u8>`, `Vec<T>`, plus helper `lemma_fold_left_on_equiv_seqs`.

---

## Results Overview

| Category | Total | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary Tests | 5 | 5 ✅ | 0 |
| Behavioral Mutation Tests | 5 | 5 ✅ | 0 |
| Logical Tests | 5 | 4 ✅ | 1 ⚠️ |
| **Total** | **15** | **14** | **1** |

---

## Boundary Tests (5/5 FAIL ✅)

All precondition violations are correctly rejected:

| Test | Failure Mode | Result |
|---|---|---|
| `test_boundary_u64_distinct_values` | Non-view-equal u64 (0 vs 1) | FAIL ✅ |
| `test_boundary_usize_distinct_values` | Non-view-equal usize (0 vs 100) | FAIL ✅ |
| `test_boundary_u64_zero_vs_max` | Edge case: u64 0 vs MAX | FAIL ✅ |
| `test_boundary_vec_u8_different_contents` | Vec<u8> with different contents | FAIL ✅ |
| `test_boundary_fold_left_mismatched_lengths` | Sequences with different lengths | FAIL ✅ |

**Conclusion**: The `requires` clauses are effective at rejecting invalid inputs.

---

## Behavioral Mutation Tests (5/5 FAIL ✅)

All mutated postconditions are correctly rejected:

| Test | Mutation | Result |
|---|---|---|
| `test_mutation_u64_serialize_not_equal` | Assert serializations differ for equal u64 | FAIL ✅ |
| `test_mutation_usize_marshalability_differs` | Assert marshalability differs for equal usize | FAIL ✅ |
| `test_mutation_u64_serialize_wrong_length` | Assert u64 serialization has length 4 (not 8) | FAIL ✅ |
| `test_mutation_u64_marshalability_negated` | Assert u64 is NOT marshalable | FAIL ✅ |
| `test_mutation_fold_left_wrong_result` | Assert fold_left on equal seqs gives different results | FAIL ✅ |

**Conclusion**: The spec correctly rejects incorrect behavioral claims.

---

## Logical Tests (4/5 FAIL, 1 PASS ⚠️)

| Test | Property Tested | Result |
|---|---|---|
| `test_logical_serialize_equal_implies_view_equal` | Converse: equal serialization ⇒ view_equal | FAIL ✅ |
| `test_logical_all_u64_same_serialization` | All u64 values serialize identically | FAIL ✅ |
| `test_logical_usize_always_marshalable` | All usize values are marshalable | **PASS ⚠️** |
| `test_logical_non_view_equal_implies_different_serialize` | Non-view-equal ⇒ different serialization | FAIL ✅ |
| `test_logical_fold_left_order_independent` | fold_left is order-independent | FAIL ✅ |

### ⚠️ Spec Weakness Found

**`test_logical_usize_always_marshalable`** — PASSED unexpectedly.

The `is_marshalable` spec for `usize` includes the guard `*self as int <= u64::MAX`. On 64-bit architectures, `usize` is a 64-bit integer, so this condition is **always true** — the guard is vacuous. The spec appears restrictive but restricts nothing.

**Impact**: If the intent was to guard against platform-dependent overflow (e.g., on hypothetical architectures where `usize > u64::MAX`), the guard gives a false sense of safety. On all real architectures where Verus runs, the constraint is trivially satisfied.

### Additional Finding (from earlier iteration)

**`view_equal` transitivity for u64** — also provable from definitions. The original test `forall |a, b, c: u64| a.view_equal(&b) && b.view_equal(&c) ==> a.ghost_serialize() =~= c.ghost_serialize()` passed because `view_equal` for u64 is `===` (structural equality), which is inherently transitive. This is not a weakness per se, but shows the spec allows deriving transitivity chains directly from concrete definitions without the lemma.

---

## Summary

The specification is **largely sound**: 14 of 15 adversarial tests correctly fail verification. The one identified weakness — vacuous `usize` marshalability guard — is a completeness issue where the spec's protective constraint provides no actual restriction on standard architectures.
