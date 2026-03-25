# Adversarial Proof Test Summary

**Target**: `delegation_map_v__impl4__all_keys_agree.rs`  
**Function**: `DelegationMap::all_keys_agree`  
**Date**: 2026-03-22

---

## Specification Under Test

`all_keys_agree` proves that if all low-map entries in index range `[lo, hi]` map to the same endpoint `id`, then the delegation map is `range_consistent` — meaning every key `k` with `keys[lo] ≤ k < keys[hi]` satisfies `dm@[k] == id@`.

---

## Results Summary

| # | File | Test Name | Type | Target Failure Mode | Result |
|---|------|-----------|------|---------------------|--------|
| 1 | boundary_tests.rs | `test_boundary_lo_gt_hi` | Boundary | `lo > hi` violates `lo ≤ hi` | **FAILED** ✅ |
| 2 | boundary_tests.rs | `test_boundary_hi_out_of_bounds` | Boundary | `hi ≥ keys.len()` violates bounds | **FAILED** ✅ |
| 3 | boundary_tests.rs | `test_boundary_not_valid` | Boundary | Missing `self.valid()` | **FAILED** ✅ |
| 4 | boundary_tests.rs | `test_boundary_missing_forall` | Boundary | Missing forall (keys → id) | **FAILED** ✅ |
| 5 | behavioral_mutation_tests.rs | `test_mutation_wrong_id` | Behavioral | Assert range_consistent with wrong id | **FAILED** ✅ |
| 6 | behavioral_mutation_tests.rs | `test_mutation_extended_range` | Behavioral | Assert range_consistent for `[lo, hi+1)` | **FAILED** ✅ |
| 7 | behavioral_mutation_tests.rs | `test_mutation_negated_postcondition` | Behavioral | Assert ¬range_consistent (contradicts post) | **FAILED** ✅ |
| 8 | logical_tests.rs | `test_logical_global_agreement` | Logical | Arbitrary key maps to id (stronger claim) | **FAILED** ✅ |
| 9 | logical_tests.rs | `test_logical_weaker_precondition` | Logical | Single-key precondition suffices (weaker pre) | **FAILED** ✅ |
| 10 | logical_tests.rs | `test_logical_single_key_no_extend` | Logical | Key agreement extends to next index | **FAILED** ✅ |

**Overall: 10/10 tests correctly rejected by the specification.**

---

## Analysis

### Boundary Tests (4/4 rejected)
The specification's preconditions are tight:
- **Index ordering** (`lo ≤ hi`) is enforced — reversed indices are rejected.
- **Bounds checking** (`hi < keys.len()`) is enforced — out-of-bounds access is rejected.
- **Validity invariant** (`self.valid()`) is required — unvalidated maps cannot invoke the lemma.
- **Key agreement condition** (the `forall` over `[lo, hi]`) is necessary — partial agreement is insufficient.

### Behavioral Mutation Tests (3/3 rejected)
The postcondition is semantically precise:
- **Wrong endpoint**: Substituting a different `id` is correctly rejected, confirming `range_consistent` binds to the specific endpoint.
- **Range extension**: The guarantee does not silently extend beyond `keys[hi]`.
- **Negation**: The postcondition is non-vacuous — asserting its negation causes contradiction, confirming it carries real semantic content.

### Logical Tests (3/3 rejected)
The specification does not entail unintended properties:
- **Global agreement**: The range-local guarantee does not generalize to arbitrary keys outside `[keys[lo], keys[hi])`.
- **Weaker precondition**: A single key mapping to `id` is insufficient to derive full range consistency — the complete forall over `[lo, hi]` is necessary.
- **Inductive extension**: Knowing `dm@[keys[lo]] == id@` does not imply `dm@[keys[lo+1]] == id@` — agreement does not propagate without explicit specification.

---

## Conclusion

The specification for `all_keys_agree` is **well-calibrated**: it correctly rejects all 10 adversarial queries across boundary, behavioral, and logical dimensions. No specification weaknesses were detected.
