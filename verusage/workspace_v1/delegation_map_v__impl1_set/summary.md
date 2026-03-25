# Test Summary: `delegation_map_v__impl1_set`

## Target Specification

The target file defines `StrictlyOrderedVec<K>::set(i, k)`, which replaces the element at index `i` with a new key `k` while maintaining sorted order and uniqueness.

- **Pre**: `valid()`, `i < len`, `i > 0 ⟹ old[i-1] < k`, `i < len-1 ⟹ k < old[i+1]`
- **Post**: `valid()`, `self@ == old@.update(i, k)`

## Results

All **9 tests** correctly **FAILED** verification, confirming the specification rejects invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (3/3 FAILED ✅)

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_index_eq_len` | `i == s.len()` (off-by-one) | ✅ Precondition rejected |
| `test_boundary_k_equal_predecessor` | `k == s[i-1]` (not strictly greater) | ✅ Precondition rejected |
| `test_boundary_k_equal_successor` | `k == s[i+1]` (not strictly less) | ✅ Precondition rejected |

### Behavioral Mutation Tests (3/3 FAILED ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_wrong_update_position` | Claimed `k` appears at index 0 instead of index 1 | ✅ Assertion rejected |
| `test_mutation_length_changed` | Claimed length changed from 3 to 2 after set | ✅ Assertion rejected |
| `test_mutation_result_unchanged` | Claimed result equals original after updating with different value | ✅ Assertion rejected |

### Logical Tests (3/3 FAILED ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_logical_cross_position_equivalence` | Cross-position: `set(s,1,4)` ≡ `set(s,2,4)` | ✅ Assertion rejected |
| `test_logical_midpoint_assumption` | Stronger inequality: `k == (left + right) / 2` | ✅ Assertion rejected |
| `test_logical_different_k_same_result` | Determinism of choice: different valid `k` values yield same result | ✅ Assertion rejected |

## Conclusion

The specification for `set` on `StrictlyOrderedVec` is **consistent** with respect to all tested queries:

1. **Preconditions** correctly reject out-of-bounds indices and boundary-equal values that would violate strict ordering.
2. **Postconditions** correctly constrain the update position, result length, and result contents — mutated behaviors are all rejected.
3. **Logical properties** not guaranteed by the spec (cross-position equivalence, midpoint assumptions, choice determinism) are all correctly rejected.

**No specification weaknesses detected** in the tested semantic space.
