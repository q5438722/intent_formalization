# Adversarial Proof Test Results — `delegation_map_v__impl3__erase`

## Target
`StrictlyOrderedMap::erase` and supporting functions (`StrictlyOrderedVec::erase`, `to_set`, `gap_means_empty`, `choose_gap_violator`).

## Summary
All 12 adversarial tests correctly **FAILED** verification, indicating the specification properly rejects invalid inputs, incorrect behaviors, and unintended logical reasoning.

---

## Boundary Tests (4/4 FAILED ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_to_set_on_invalid_vec` | Call `to_set` without `valid()` precondition | FAILED ✅ |
| `test_gap_means_empty_without_gap` | Call `gap_means_empty` without `gap(lo, hi)` | FAILED ✅ |
| `test_choose_gap_violator_with_gap` | Call `choose_gap_violator` when gap exists | FAILED ✅ |
| `test_gap_means_empty_k_outside_range` | Call `gap_means_empty` with `k` outside `[lo, hi)` | FAILED ✅ |

**Conclusion**: All preconditions are properly enforced. Invalid inputs are rejected.

---

## Behavioral Mutation Tests (4/4 FAILED ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_erased_key_still_present` | Assert erased key remains in map | FAILED ✅ |
| `test_preserved_key_removed` | Assert preserved key was removed | FAILED ✅ |
| `test_new_key_appears` | Assert new key appears after erase | FAILED ✅ |
| `test_value_changed_after_erase` | Assert preserved key's value changed | FAILED ✅ |

**Conclusion**: The erase postcondition correctly constrains key membership and value preservation. Incorrect behaviors are rejected.

---

## Logical Tests (4/4 FAILED ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_no_new_gaps_after_erase` | Assert new gaps must have existed before erase | FAILED ✅ |
| `test_to_set_empty_on_nonempty` | Assert `to_set` returns empty on non-empty vec | FAILED ✅ |
| `test_erase_identity_nonempty_range` | Assert erased key survives (identity after erase) | FAILED ✅ |
| `test_key_iterator_total_order_without_cmp_properties` | Assert trichotomy without `cmp_properties()` | FAILED ✅ |

**Conclusion**: The spec does not allow unintended reasoning. Gap creation is properly modeled. Ordering properties require explicit `cmp_properties()` invocation.

---

## Notable Finding

The source code's TODO comment about strengthening the `StrictlyOrderedVec::erase` spec to assert set disjointness is unnecessary — disjointness **IS** already derivable from the existing spec (via `valid()` → `no_duplicates` + sequence structure). An initial test for this property unexpectedly passed and was replaced.

---

## Overall Assessment
The specification for `delegation_map_v__impl3__erase` is **well-formed**: it rejects all tested boundary violations, behavioral mutations, and unintended logical inferences. No spec weaknesses were detected.
