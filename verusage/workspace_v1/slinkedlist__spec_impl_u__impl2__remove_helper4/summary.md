# Adversarial Test Summary: `remove_helper4`

## Target
`slinkedlist__spec_impl_u__impl2__remove_helper4.rs` — removes a middle node from a `StaticLinkedList` when the free list is empty.

## Preconditions (requires)
- `wf()`, value contains `v`, `get_node_ref(v) == remove_index`
- `value_list_len != 1`, `free_list_len == 0`
- `remove_index` is neither head nor tail (middle node)

## Postconditions (ensures)
- `wf()`, `len() == old.len() - 1`, `ret == v`
- Node refs preserved for remaining values
- `unique()`, `self@ =~= old@.remove_value(ret)`

---

## Results

All **15 adversarial tests** correctly **FAIL** verification, indicating the specification properly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (5/5 FAIL ✓)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_unique_with_duplicates` | Asserts `unique()` on sequence with duplicates [42, 99, 42] | FAIL ✓ |
| 2 | `test_boundary_spec_len_mismatch` | Asserts `spec_len() != 4` when `@.len() == 4` | FAIL ✓ |
| 3 | `test_boundary_remove_result_not_empty` | Asserts post `len() == 0` when removing from 4-element list | FAIL ✓ |
| 4 | `test_boundary_unique_implies_distinct` | Asserts `@[0] == @[1]` under `unique()` with `len >= 2` | FAIL ✓ |
| 5 | `test_boundary_no_wf_len_link` | Asserts `spec_len() == 4` from `value_list_len == 4` without `wf()` | FAIL ✓ |

### Behavioral Mutation Tests (5/5 FAIL ✓)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_wrong_return_value` | Claims `ret == 99` when `ret == 42` (mutated postcondition 3) | FAIL ✓ |
| 2 | `test_mutation_length_unchanged` | Claims post `len == pre len` instead of `pre len - 1` | FAIL ✓ |
| 3 | `test_mutation_seq_unchanged` | Claims `post@ =~= pre@` (no removal) | FAIL ✓ |
| 4 | `test_mutation_length_off_by_two` | Claims `post len == pre len - 2` instead of `-1` | FAIL ✓ |
| 5 | `test_mutation_ret_is_first_element` | Claims `ret == @[0]` when `ret == @[1]` (wrong element) | FAIL ✓ |

### Logical Tests (5/5 FAIL ✓)

| # | Test | Unwarranted Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_value_constrained` | Asserts `ret > 0` (value magnitude not constrained) | FAIL ✓ |
| 2 | `test_logical_free_list_grows_by_two` | Asserts free list grows by 2 (should be 1) | FAIL ✓ |
| 3 | `test_logical_node_ref_preserved_for_removed` | Asserts `get_node_ref` preserved for removed value | FAIL ✓ |
| 4 | `test_logical_array_size_changes` | Asserts `size` differs between pre/post (wf is closed) | FAIL ✓ |
| 5 | `test_logical_index_specific_value` | Asserts `remove_index == 1` (implementation detail) | FAIL ✓ |

---

## Conclusion

The specification for `remove_helper4` is **consistent** across all three query categories:
- **Boundary**: Invalid inputs (duplicates, mismatched lengths, missing wf linkage) are properly rejected.
- **Behavioral**: Incorrect outputs (wrong return values, wrong lengths, unchanged sequences) are properly rejected.
- **Logical**: Unwarranted properties (value constraints, stronger free-list claims, opaque struct internals, implementation-specific indices) are properly rejected.

No specification weaknesses were detected.
