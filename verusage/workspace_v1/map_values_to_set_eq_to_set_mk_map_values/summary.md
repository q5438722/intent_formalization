# Adversarial Test Summary: `map_values_to_set_eq_to_set_mk_map_values`

## Specification Under Test

Two proof functions from `vstd_exd/seq_lib`:

1. **`push_to_set_eq_to_set_insert`** (axiom, `external_body`):  
   `ensures s.push(e).to_set() == s.to_set().insert(e)`

2. **`map_values_to_set_eq_to_set_mk_map_values`** (lemma):  
   `ensures s.map_values(map).to_set() == s.to_set().mk_map(map).values()`

Neither function has `requires` clauses (no preconditions).

---

## Test Results

| # | File | Test Name | Property Queried | Expected | Actual |
|---|------|-----------|-----------------|----------|--------|
| 1 | boundary_tests.rs | `test_boundary_empty_seq_contains_element` | Empty seq mapped contains an element | FAIL | ✅ FAIL |
| 2 | boundary_tests.rs | `test_boundary_push_axiom_wrong_membership` | Push axiom derives wrong element membership | FAIL | ✅ FAIL |
| 3 | boundary_tests.rs | `test_boundary_singleton_set_oversized` | Singleton maps to set of size > 1 | FAIL | ✅ FAIL |
| 4 | boundary_tests.rs | `test_boundary_empty_map_nonempty_values` | Empty seq's mk_map has non-empty values | FAIL | ✅ FAIL |
| 5 | mutation_tests.rs | `test_mutation_wrong_element_in_result` | Mapped set contains unmapped element | FAIL | ✅ FAIL |
| 6 | mutation_tests.rs | `test_mutation_negated_postcondition` | Postcondition negation holds | FAIL | ✅ FAIL |
| 7 | mutation_tests.rs | `test_mutation_wrong_function_equivalence` | Different maps produce same set | FAIL | ✅ FAIL |
| 8 | mutation_tests.rs | `test_mutation_collapsing_map_wrong_member` | Constant map result contains original value | FAIL | ✅ FAIL |
| 9 | logical_tests.rs | `test_logical_size_preservation_claim` | map_values preserves cardinality | FAIL | ✅ FAIL |
| 10 | logical_tests.rs | `test_logical_non_identity_treated_as_identity` | Non-identity map treated as identity | FAIL | ✅ FAIL |
| 11 | logical_tests.rs | `test_logical_axiom_chain_phantom_element` | Chained axiom creates phantom membership | FAIL | ✅ FAIL |
| 12 | logical_tests.rs | `test_logical_false_injectivity` | Constant map falsely claimed injective | FAIL | ✅ FAIL |

**Result: 12/12 tests failed verification as expected.**

---

## Analysis

### Boundary Tests (4/4 rejected)
The specification correctly handles edge cases:
- Empty sequences produce empty result sets; no spurious elements emerge.
- The `push_to_set_eq_to_set_insert` axiom does not over-extend to elements not actually pushed.
- Singleton sequences map to singleton sets with correct cardinality.

### Behavioral Mutation Tests (4/4 rejected)
The specification correctly rejects mutated behaviors:
- Incorrect output elements are not admitted into mapped sets.
- The postcondition cannot be negated — the equality is enforced.
- Different mapping functions produce provably different result sets.
- Constant (collapsing) maps correctly restrict the result set to the constant value only.

### Logical Tests (4/4 rejected)
The specification does not entail unintended logical properties:
- Cardinality preservation is correctly NOT guaranteed (non-injective maps collapse).
- Non-identity maps cannot be confused with identity.
- Chained axiom applications do not create phantom set memberships.
- Constant maps are not falsely treated as injective.

---

## Conclusion

The specification is **consistent** with respect to all 12 adversarial queries tested. It:
- Correctly rejects invalid inputs at boundaries (despite having no explicit `requires`).
- Rejects all mutated/incorrect behavioral claims.
- Does not entail unintended logical properties (no false injectivity, no cardinality preservation, no phantom elements).

No specification weaknesses were identified. The `external_body` axiom `push_to_set_eq_to_set_insert` does not appear to introduce unsoundness in any of the tested scenarios.
