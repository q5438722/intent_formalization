# Test Results Summary: `remove_helper2`

**Target**: `slinkedlist__spec_impl_u__impl2__remove_helper2.rs`
**Function**: `StaticLinkedList::remove_helper2` — removes the head node from a full value list (free_list_len == 0) and moves it to the free list.

## Specification Under Test

**Preconditions**: `wf()`, `contains(v@)`, `get_node_ref(v@) == remove_index`, `value_list_len != 1`, `free_list_len == 0 && value_list_head == remove_index`

**Postconditions**: `wf()`, `len == old.len - 1`, `ret == v@`, `unique()`, `self@ =~= old@.remove_value(ret)`, node refs preserved for remaining elements.

---

## Results: All 15/15 tests FAILED as expected ✅

The specification correctly rejects all undesirable properties.

### Boundary Tests (5/5 FAILED ✓)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_unique_with_duplicates` | Duplicates at indices 0,2 violate `unique()` | FAILED ✓ |
| 2 | `test_boundary_spec_len_mismatch` | `spec_len()` matches `@.len()` by definition | FAILED ✓ |
| 3 | `test_boundary_post_contains_removed` | `remove_value` eliminates the element from no-dup seq | FAILED ✓ |
| 4 | `test_boundary_no_wf_len_link` | Without `wf()`, `value_list_len` ≠ `spec_seq.len()` | FAILED ✓ |
| 5 | `test_boundary_post_len_equals_pre` | Post length is `pre - 1`, not equal to pre | FAILED ✓ |

### Behavioral Mutation Tests (5/5 FAILED ✓)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_wrong_return_value` | Claim `ret == 99` when head is 42 | FAILED ✓ |
| 2 | `test_mutation_length_unchanged` | Claim post length equals pre length | FAILED ✓ |
| 3 | `test_mutation_seq_unchanged` | Claim sequence is unchanged after removal | FAILED ✓ |
| 4 | `test_mutation_double_decrease` | Claim length decreases by 2 instead of 1 | FAILED ✓ |
| 5 | `test_mutation_ret_differs_from_head` | Claim `ret != sll@[0]` | FAILED ✓ |

### Logical Tests (5/5 FAILED ✓)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_value_constrained` | Stronger inequality: `ret > 0` | FAILED ✓ |
| 2 | `test_logical_free_list_grows_by_two` | Structural: free list grows by 2 | FAILED ✓ |
| 3 | `test_logical_node_ref_preserved_for_removed` | Cross-function: node ref for removed value | FAILED ✓ |
| 4 | `test_logical_array_size_changes` | Closed spec: derive `size` changed via `wf()` | FAILED ✓ |
| 5 | `test_logical_index_specific_value` | Determinism: `get_node_ref` returns slot 0 | FAILED ✓ |

## Conclusion

The specification for `remove_helper2` is **consistent** across all tested dimensions:
- **Boundary**: Invalid inputs and violated preconditions are properly rejected.
- **Behavioral**: Incorrect output mutations are properly rejected.
- **Logical**: Unintended properties (stronger inequalities, determinism, cross-function misuse, closed-spec leakage) are not entailed.

No spec weaknesses were detected.
