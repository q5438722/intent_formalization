# Test Summary: `delegation_map_v__impl1_remove`

## Target Specification

The target file defines `StrictlyOrderedVec<K>` with one key operation:

- **`remove(i)`**: Removes element at index `i` from a sorted, duplicate-free vector.
  - Pre: `valid()`, `i < len`
  - Post: `valid()`, returns `old[i]`, result = `old.remove(i)`, set relation preserved

## Results

All **9 tests** correctly **FAILED** verification, confirming the specification rejects invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (3/3 FAILED ✅)

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_remove_index_eq_len` | `i == s.len()` (off-by-one) | ✅ Precondition rejected |
| `test_boundary_remove_empty_seq` | Remove on empty sequence | ✅ Precondition rejected |
| `test_boundary_remove_negative_index` | `i = -1` (negative index) | ✅ Precondition rejected |

### Behavioral Mutation Tests (3/3 FAILED ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_wrong_removed_element` | Claimed `remove(1)` returns `s[0]` instead of `s[1]` | ✅ Assertion rejected |
| `test_mutation_length_unchanged` | Claimed length unchanged after remove | ✅ Assertion rejected |
| `test_mutation_removed_element_still_in_set` | Claimed removed element still in result set | ✅ Assertion rejected |

### Logical Tests (3/3 FAILED ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_logical_different_indices_same_result` | Cross-index: `remove(0)` ≡ `remove(2)` | ✅ Assertion rejected |
| `test_logical_remove_first_preserves_head` | Structural: removing first element preserves head value | ✅ Assertion rejected |
| `test_logical_singleton_remove_nonempty` | Global: removing from singleton yields non-empty result | ✅ Assertion rejected |

## Conclusion

The specification for `remove` on `StrictlyOrderedVec` is **consistent** with respect to all tested queries:

1. **Preconditions** correctly reject out-of-bounds indices (off-by-one, negative), and empty sequences.
2. **Postconditions** correctly constrain the return value, result sequence length, and set membership — mutated behaviors are all rejected.
3. **Logical properties** not guaranteed by the spec (cross-index equivalence, head preservation after first-element removal, non-emptiness after singleton removal) are all correctly rejected.

**No specification weaknesses detected** in the tested semantic space.
