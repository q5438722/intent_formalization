# Adversarial Proof Test Summary: `slinkedlist__spec_impl_u__impl2__remove_helper1`

## Target
`StaticLinkedList<T, N>::remove_helper1()` — removes a specific value from a static linked list when the value list has exactly 1 element.

## Specification Under Test

**Preconditions**: `wf()`, `self@.contains(v@)`, `get_node_ref(v@) == remove_index`, `value_list_len == 1`

**Postconditions**: `wf()`, `len() == old(len) - 1`, `ret == v@`, `unique()`, `self@ =~= old(self)@.remove_value(ret)`, node refs preserved for remaining elements.

---

## Results

All 15 adversarial tests **FAILED verification** as expected, meaning the specification correctly rejects all tested unintended properties.

### Boundary Tests (5/5 failed ✓)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_unique_with_duplicates` | Assert `unique()` on sequence [42, 99, 42] with duplicates | FAIL ✓ |
| 2 | `test_boundary_spec_len_mismatch` | Assert `spec_len() != 1` when `view().len() == 1` | FAIL ✓ |
| 3 | `test_boundary_remove_result_not_empty` | Assert post `len() > 0` after removing sole element | FAIL ✓ |
| 4 | `test_boundary_post_contains_removed` | Assert removed element still in post view (empty seq) | FAIL ✓ |
| 5 | `test_boundary_no_wf_len_link` | Assert `spec_len() == 1` without `wf()` (no len link) | FAIL ✓ |

### Behavioral Mutation Tests (5/5 failed ✓)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_wrong_return_value` | Claim `ret == 99` instead of `ret == 42` | FAIL ✓ |
| 2 | `test_mutation_length_unchanged` | Claim `post.len() == pre.len()` (no decrease) | FAIL ✓ |
| 3 | `test_mutation_seq_unchanged` | Claim `post@ =~= pre@` (nothing removed) | FAIL ✓ |
| 4 | `test_mutation_post_has_element` | Claim `post@.len() == 1` (element remains) | FAIL ✓ |
| 5 | `test_mutation_ret_differs_from_element` | Claim `ret != sll@[0]` | FAIL ✓ |

### Logical Tests (5/5 failed ✓)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_value_constrained` | Returned value must be `> 0` (arbitrary constraint) | FAIL ✓ |
| 2 | `test_logical_free_list_grows_by_two` | Free list grows by 2 instead of 1 | FAIL ✓ |
| 3 | `test_logical_node_ref_preserved_for_removed` | `get_node_ref` preserved for removed (not remaining) value | FAIL ✓ |
| 4 | `test_logical_array_size_changes` | Array `size` field changes after removal | FAIL ✓ |
| 5 | `test_logical_index_specific_value` | Node index must be slot 0 (allocation-specific) | FAIL ✓ |

---

## Conclusion

The specification for `StaticLinkedList::remove_helper1()` is **consistent** with respect to all 15 tested properties:

- **Boundary**: The spec correctly constrains `unique()`, `spec_len()`, and rejects non-empty post-state after single-element removal. It also requires `wf()` to link `value_list_len` to `spec_seq` length.
- **Behavioral**: The spec correctly rejects wrong return values, unchanged lengths, unchanged sequences, and contradictory element counts.
- **Logical**: The spec does not entail unintended properties about value constraints, free-list growth rates, node-ref preservation for removed elements, structural changes, or allocation-specific slot assignments.

No specification weaknesses were detected by this test suite.
