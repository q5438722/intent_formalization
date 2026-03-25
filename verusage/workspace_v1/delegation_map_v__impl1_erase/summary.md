# Test Summary: `delegation_map_v__impl1_erase`

## Target Specification

The target file defines `StrictlyOrderedVec<K>` with two key operations:

- **`remove(i)`**: Removes element at index `i` from a sorted, duplicate-free vector.
  - Pre: `valid()`, `i < len`
  - Post: `valid()`, returns `old[i]`, result = `old.remove(i)`, set relation preserved

- **`erase(start, end)`**: Removes a contiguous range `[start, end)` from the vector.
  - Pre: `valid()`, `start <= end <= len`
  - Post: `valid()`, result = prefix + suffix, set union preserved

## Results

All **9 tests** correctly **FAILED** verification, confirming the specification rejects invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (3/3 FAILED ✅)

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_remove_index_eq_len` | `i == s.len()` (off-by-one) | ✅ Precondition rejected |
| `test_boundary_remove_empty_seq` | Remove on empty sequence | ✅ Precondition rejected |
| `test_boundary_erase_start_gt_end` | `start > end` (reversed range) | ✅ Precondition rejected |

### Behavioral Mutation Tests (3/3 FAILED ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_wrong_removed_element` | Claimed `remove(1)` returns `s[0]` instead of `s[1]` | ✅ Assertion rejected |
| `test_mutation_erase_length_unchanged` | Claimed length unchanged after erase | ✅ Assertion rejected |
| `test_mutation_remove_no_change` | Claimed result equals original after remove | ✅ Assertion rejected |

### Logical Tests (3/3 FAILED ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_logical_remove_neq_erase_two` | Cross-function: `remove(0)` ≡ `erase(0,2)` | ✅ Assertion rejected |
| `test_logical_contiguous_range_assumption` | Structural: result elements form contiguous integers | ✅ Assertion rejected |
| `test_logical_erase_all_nonempty` | Global: erasing all elements yields non-empty result | ✅ Assertion rejected |

## Conclusion

The specification for `remove` and `erase` on `StrictlyOrderedVec` is **consistent** with respect to all tested queries:

1. **Preconditions** correctly reject out-of-bounds indices, empty sequences, and reversed ranges.
2. **Postconditions** correctly constrain the return value, result sequence, and result length — mutated behaviors are all rejected.
3. **Logical properties** not guaranteed by the spec (cross-function equivalence, structural assumptions, edge-case emptiness) are all correctly rejected.

**No specification weaknesses detected** in the tested semantic space.

> **Note**: The spec comment mentions a potential strengthening: making the erased-set and result-set disjointness explicit. While not tested as a weakness here (the sequence-level postcondition fully determines the result), this could be relevant for clients that reason only about the set-level postcondition.
