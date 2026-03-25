# Adversarial Test Summary: `lemma_set_map_insert`

## Specification Under Test

```rust
pub proof fn lemma_set_map_insert<A, B>(s: Set<A>, f: spec_fn(A) -> B, x: A)
    ensures s.insert(x).map(f) == s.map(f).insert(f(x))
```

**No preconditions.** Postcondition: mapping distributes over insert on sets.

---

## Results Overview

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 4 | ✅ Yes (4/4 errors) |
| `behavioral_mutation_tests.rs` | 4 | ✅ Yes (4/4 errors) |
| `logical_tests.rs` | 4 | ✅ Yes (4/4 errors) |

**Total: 12/12 tests correctly rejected by Verus.**

---

## Boundary Tests (4 tests)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_boundary_empty_set_mapped_is_empty` | `{}.insert(0).map(id) == {}` | ✅ FAIL |
| 2 | `test_boundary_empty_set_missing_mapped_element` | `1 ∉ {}.insert(0).map(x↦x+1)` | ✅ FAIL |
| 3 | `test_boundary_constant_fn_original_value_in_result` | `5 ∈ {}.insert(5).map(x↦42)` | ✅ FAIL |
| 4 | `test_boundary_redundant_insert_false_extra_element` | `3 ∈ {3}.insert(3).map(x↦2x)` | ✅ FAIL |

**Conclusion:** Edge cases (empty sets, constant functions, redundant inserts) are properly handled. The spec correctly rejects false membership claims at boundaries.

---

## Behavioral Mutation Tests (4 tests)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_omit_insert_rhs` | Removed `.insert(f(x))` from RHS | ✅ FAIL |
| 2 | `test_mutation_wrong_insert_value` | Used `x` instead of `f(x)` in insert | ✅ FAIL |
| 3 | `test_mutation_negated_postcondition` | Negated the equality (`!=`) | ✅ FAIL |
| 4 | `test_mutation_double_application` | Used `f(f(x))` instead of `f(x)` | ✅ FAIL |

**Conclusion:** All mutations of the postcondition are rejected. The spec is precise enough to distinguish the correct identity from structurally similar but incorrect variants.

---

## Logical Tests (4 tests)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_map_not_injective` | Map preserves elements (injectivity) | ✅ FAIL |
| 2 | `test_logical_phantom_element_in_map` | Arbitrary values appear in mapped set | ✅ FAIL |
| 3 | `test_logical_map_preserves_distinctness` | Non-injective map preserves cardinality | ✅ FAIL |
| 4 | `test_logical_cross_function_transfer` | Lemma result transfers between different functions | ✅ FAIL |

**Conclusion:** The spec does not entail unintended logical properties. Injectivity, phantom membership, cardinality preservation, and cross-function transfer are all correctly rejected.

---

## Overall Assessment

The specification for `lemma_set_map_insert` is **tight and well-formed**:

- **No missing preconditions**: The absence of `requires` is appropriate — the identity holds universally for all sets, functions, and elements.
- **Precise postcondition**: Mutations (omission, substitution, negation, double-application) are all rejected, confirming the postcondition is neither too strong nor too weak.
- **No unintended entailments**: The spec does not accidentally imply injectivity, cardinality preservation, or cross-function properties.

No specification weaknesses were detected.
