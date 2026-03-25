# Adversarial Proof Test Results

**Target**: `marshal_v__impl4__lemma_view_equal_symmetric.rs`
**Specification under test**: `lemma_view_equal_symmetric` — ensures `self.view_equal(other) == other.view_equal(self)` (symmetry of `view_equal`)

---

## Summary

| Test File | Tests | All Failed (as expected) |
|-----------|-------|--------------------------|
| `boundary_tests.rs` | 4 | ✅ Yes |
| `behavioral_mutation_tests.rs` | 4 | ✅ Yes |
| `logical_tests.rs` | 4 | ✅ Yes |

**Total: 12/12 adversarial tests correctly rejected by the specification.**

---

## Boundary Tests (4/4 FAILED ✅)

Tests that probe edge cases and invalid input regions:

| # | Test | Property Asserted (φ) | Verdict |
|---|------|-----------------------|---------|
| 1 | `test_boundary_different_usize_are_equal` | `view_equal(0, 1)` is true | ✅ Rejected |
| 2 | `test_boundary_same_usize_not_equal` | `view_equal(0, 0)` is false | ✅ Rejected |
| 3 | `test_boundary_empty_vs_nonempty_vec` | empty vec equals non-empty vec | ✅ Rejected |
| 4 | `test_boundary_max_vs_zero_usize` | `view_equal(MAX, 0)` is true | ✅ Rejected |

**Conclusion**: The concrete `view_equal` definitions correctly distinguish values at all boundaries.

---

## Behavioral Mutation Tests (4/4 FAILED ✅)

Tests that invoke the symmetry lemma then assert mutated (incorrect) results:

| # | Test | Mutation Applied | Verdict |
|---|------|-----------------|---------|
| 1 | `test_mutation_symmetry_no_false_equality` | Symmetry call + assert equal for a≠b | ✅ Rejected |
| 2 | `test_mutation_negate_true_equality` | Symmetry call + negate self-equality | ✅ Rejected |
| 3 | `test_mutation_symmetry_reverse_negated` | Given a=b, assert ¬b.view_equal(a) | ✅ Rejected |
| 4 | `test_mutation_vec_length_mismatch_after_symmetry` | Symmetry call + assert equal for diff-length vecs | ✅ Rejected |

**Conclusion**: The symmetry lemma does not leak extra proving power; it cannot be used to derive false equalities or negate true ones.

---

## Logical Tests (4/4 FAILED ✅)

Tests that probe properties NOT entailed by symmetry at the abstract trait level (generic `T: Marshalable`):

| # | Test | Unentailed Property | Verdict |
|---|------|---------------------|---------|
| 1 | `test_logical_reflexivity_not_entailed` | Reflexivity: `a.view_equal(a)` | ✅ Rejected |
| 2 | `test_logical_transitivity_not_entailed` | Transitivity: `a=b ∧ b=c ⟹ a=c` | ✅ Rejected |
| 3 | `test_logical_anti_reflexivity_not_entailed` | Anti-reflexivity: `¬a.view_equal(a)` | ✅ Rejected |
| 4 | `test_logical_cross_argument_no_leakage` | Cross-argument: `a=b ⟹ a=c` | ✅ Rejected |

**Conclusion**: The trait-level specification correctly limits reasoning to symmetry only. It does not inadvertently entail reflexivity, transitivity, anti-reflexivity, or cross-argument inference — confirming the spec is tight at the trait abstraction level.

---

## Overall Assessment

The specification for `lemma_view_equal_symmetric` is **consistent** with respect to all 12 adversarial queries:

- **Boundary correctness**: Concrete implementations correctly reject invalid equalities at edge values.
- **Mutation resistance**: The symmetry lemma cannot be weaponized to derive false behavioral claims.
- **Logical tightness**: At the trait level, symmetry does not over-approximate — it entails exactly symmetry and nothing more.

**No spec weaknesses detected.** The specification neither admits invalid inputs, incorrect behaviors, nor unintended logical inferences.
