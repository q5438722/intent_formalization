# Test Summary: `a_submap_of_a_finite_map_is_finite`

## Specification Under Test

```rust
pub proof fn a_submap_of_a_finite_map_is_finite<K, V>(m1: Map<K, V>, m2: Map<K, V>)
    requires
        m1.submap_of(m2),
        m2.dom().finite(),
    ensures
        m1.dom().finite(),
```

## Results Overview

| Category               | Tests | All Failed (as expected) |
|------------------------|-------|--------------------------|
| Boundary Tests         | 3     | ✅ Yes (3/3)             |
| Behavioral Mutation    | 3     | ✅ Yes (3/3)             |
| Logical Tests          | 3     | ✅ Yes (3/3)             |
| **Total**              | **9** | **✅ 9/9**               |

## Boundary Tests (precondition violations)

| # | Test | Violated Precondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_boundary_not_submap` | `m1.submap_of(m2)` — m1 has key not in m2 | ✅ FAILED |
| 2 | `test_boundary_m2_not_finite` | `m2.dom().finite()` — m2's domain is infinite | ✅ FAILED |
| 3 | `test_boundary_both_violated` | Both preconditions — neither submap nor finite | ✅ FAILED |

**Conclusion**: The preconditions correctly reject all invalid inputs.

## Behavioral Mutation Tests (incorrect output assertions)

| # | Test | Mutated Property | Result |
|---|------|-----------------|--------|
| 1 | `test_mutation_negate_finite` | `!m1.dom().finite()` — negation of postcondition | ✅ FAILED |
| 2 | `test_mutation_domain_equality` | `m1.dom() =~= m2.dom()` — domain equality (too strong) | ✅ FAILED |
| 3 | `test_mutation_map_equality` | `m1 =~= m2` — map equality (too strong) | ✅ FAILED |

**Conclusion**: The spec correctly rejects all mutated behaviors.

## Logical Tests (unintended reasoning)

| # | Test | Unintended Property | Result |
|---|------|-------------------|--------|
| 1 | `test_logical_derive_false` | `false` — spec consistency check | ✅ FAILED |
| 2 | `test_logical_submap_larger_than_supermap` | `m1.dom().len() > m2.dom().len()` — impossible cardinality | ✅ FAILED |
| 3 | `test_logical_reverse_submap` | `m2.submap_of(m1)` — reverse submap not guaranteed | ✅ FAILED |

**Conclusion**: The spec does not allow unintended logical inferences. The axiom is consistent (cannot derive `false`).

## Overall Assessment

The specification is **well-formed and appropriately bounded**:

- **Preconditions** are enforced: invalid inputs (non-submap, infinite supermap) are rejected.
- **Postcondition** is precise: it guarantees finiteness of the submap's domain without implying stronger properties (domain equality, map equality).
- **No logical inconsistencies** detected: the spec does not introduce contradictions and does not allow deriving impossible cardinality claims or reverse submap relationships.
