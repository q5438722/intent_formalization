# Adversarial Proof Test Summary: `slinkedlist__spec_impl_u__impl2__pop`

## Target
`StaticLinkedList<T, N>::pop()` — removes and returns the first element from a static linked list.

## Specification Under Test

**Preconditions**: `wf()`, `len() > 0`, `unique()`, `N > 2`

**Postconditions**: `wf()`, `len() == old(len) - 1`, `self@ == old(self)@.skip(1)`, `ret.0 == old(self)@[0]`, `ret.1 == get_node_ref(ret.0)`, node refs preserved for remaining elements.

---

## Results

All 15 adversarial tests **FAILED verification** as expected, meaning the specification correctly rejects all tested unintended properties.

### Boundary Tests (5/5 failed ✓)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_unique_with_duplicates` | Assert `unique()` on sequence [42, 99, 42] | FAIL ✓ |
| 2 | `test_boundary_spec_len_mismatch` | Assert `spec_len() != 3` when `view().len() == 3` | FAIL ✓ |
| 3 | `test_boundary_pop_result_not_empty` | Assert post `len() == 0` when pre `len() >= 2` | FAIL ✓ |
| 4 | `test_boundary_pop_non_unique_result` | Assert `post@[0] == post@[1]` (duplicates in result) | FAIL ✓ |
| 5 | `test_boundary_pop_single_element_not_empty` | Assert `post.len() > 0` after popping sole element | FAIL ✓ |

### Behavioral Mutation Tests (5/5 failed ✓)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_pop_returns_wrong_element` | Claim `ret.0 == sll@[1]` instead of `sll@[0]` | FAIL ✓ |
| 2 | `test_mutation_pop_length_unchanged` | Claim `post.len() == pre.len()` (no decrease) | FAIL ✓ |
| 3 | `test_mutation_pop_skips_two` | Claim `post@ == old@.skip(2)` instead of `skip(1)` | FAIL ✓ |
| 4 | `test_mutation_pop_length_off_by_two` | Claim `post.len() == pre.len() - 2` | FAIL ✓ |
| 5 | `test_mutation_post_first_is_wrong` | Claim `post@[0] == old@[0]` instead of `old@[1]` | FAIL ✓ |

### Logical Tests (5/5 failed ✓)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_pop_ret_index_negative` | Returned node index `ret.1 < 0` | FAIL ✓ |
| 2 | `test_logical_popped_value_constrained` | Popped value must be `> 0` (arbitrary constraint) | FAIL ✓ |
| 3 | `test_logical_free_list_grows_by_two` | Free list grows by 2 instead of 1 | FAIL ✓ |
| 4 | `test_logical_popped_element_still_present` | Popped element still in post sequence | FAIL ✓ |
| 5 | `test_logical_array_size_changes` | Array `size` field changes after pop | FAIL ✓ |

---

## Conclusion

The specification for `StaticLinkedList::pop()` is **consistent** with respect to all 15 tested properties:

- **Boundary**: The spec correctly constrains `unique()`, `spec_len()`, and length transitions at edge values.
- **Behavioral**: The spec correctly rejects mutated return values, wrong skip amounts, and incorrect length changes.
- **Logical**: The spec does not entail unintended properties about value constraints, free-list growth rates, element retention, or structural changes.

No specification weaknesses were detected by this test suite.
