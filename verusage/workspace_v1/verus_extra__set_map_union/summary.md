# Adversarial Proof Test Summary

**Target**: `verus_extra__set_map_union_auto.rs`
**Specification**: `(s1 + s2).map(f) == s1.map(f) + s2.map(f)` — map distributes over set union.

## Results

All **12 tests** across 3 files **failed verification** as expected (SHOULD FAIL → FAILED ✓).

### Boundary Tests (4/4 rejected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_boundary_both_empty_nonempty` | Empty union mapped contains 0 | ✅ Rejected |
| `test_boundary_one_empty_result_empty` | Empty + singleton mapped is empty | ✅ Rejected |
| `test_boundary_self_union_extra_element` | Self-union mapped has unmapped value | ✅ Rejected |
| `test_boundary_constant_fn_original_value_persists` | Constant fn preserves original values | ✅ Rejected |

### Behavioral Mutation Tests (4/4 rejected)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_drop_s2` | Omit s2.map(f) from RHS | ✅ Rejected |
| `test_mutation_drop_s1` | Omit s1.map(f) from RHS | ✅ Rejected |
| `test_mutation_negated_postcondition` | Negate the equality | ✅ Rejected |
| `test_mutation_wrong_function_rhs` | Use different function g on RHS | ✅ Rejected |

### Logical Tests (4/4 rejected)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_inconsistency_check` | `false` derivable (vacuous axiom) | ✅ Rejected |
| `test_logical_map_over_difference` | Map distributes over set difference | ✅ Rejected |
| `test_logical_map_over_intersection` | Map distributes over intersection | ✅ Rejected |
| `test_logical_map_implies_injectivity` | Map preserves element identity | ✅ Rejected |

## Conclusion

The specification is **consistent** with respect to all tested queries:

- **Boundary**: Edge cases (empty sets, self-union, constant functions) are handled correctly.
- **Behavioral**: Mutated postconditions (dropped operands, negation, wrong function) are properly rejected.
- **Logical**: The spec does not inadvertently entail intersection/difference distributivity, injectivity, or inconsistency.

No specification weaknesses were detected.
