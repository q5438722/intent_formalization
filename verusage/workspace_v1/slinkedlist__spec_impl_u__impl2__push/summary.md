# Test Results Summary: `slinkedlist__spec_impl_u__impl2__push`

## Target Function
`StaticLinkedList<T, N>::push(&mut self, new_value: &T) -> SLLIndex`

Appends `new_value` to a static linked list, consuming a free node and returning its index.

## Results: ALL 15 TESTS FAILED ✅ (as expected)

All adversarial tests were correctly rejected by the specification, indicating the spec is sufficiently strong for the tested properties.

---

### Boundary Tests (5/5 FAILED ✅)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_push_when_full` | Full list (len==N) has no free nodes | FAIL ✅ |
| 2 | `test_boundary_push_duplicate_value` | Pushing existing value breaks uniqueness | FAIL ✅ |
| 3 | `test_boundary_unique_with_duplicates` | Sequence with duplicates is not unique | FAIL ✅ |
| 4 | `test_boundary_spec_len_mismatch` | spec_len matches view().len() | FAIL ✅ |
| 5 | `test_boundary_push_result_not_empty` | Post-push list cannot be empty | FAIL ✅ |

### Behavioral Mutation Tests (5/5 FAILED ✅)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_push_length_increases_by_two` | len+2 instead of len+1 | FAIL ✅ |
| 2 | `test_mutation_push_wrong_value` | Last element is other_value, not new_value | FAIL ✅ |
| 3 | `test_mutation_push_length_unchanged` | Length unchanged after push | FAIL ✅ |
| 4 | `test_mutation_push_last_element_wrong` | Last element is old first, not new_value | FAIL ✅ |
| 5 | `test_mutation_push_first_element_changed` | First element becomes new_value | FAIL ✅ |

### Logical Tests (5/5 FAILED ✅)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_push_return_always_zero` | Return index always 0 (false determinism) | FAIL ✅ |
| 2 | `test_logical_push_return_negative` | Return index negative (violates bounds) | FAIL ✅ |
| 3 | `test_logical_pushed_value_must_be_positive` | Pushed value > 0 (unwarranted constraint) | FAIL ✅ |
| 4 | `test_logical_free_list_shrinks_by_two` | Free list shrinks by 2 (stronger claim) | FAIL ✅ |
| 5 | `test_logical_array_size_changes_after_push` | Array size changes (structural violation) | FAIL ✅ |

---

## Conclusion

The `push` specification is **consistent** across all tested dimensions:
- **Boundary**: Correctly rejects invalid inputs (full list, duplicates, mismatched lengths)
- **Behavioral**: Correctly rejects mutated postconditions (wrong length delta, wrong appended value, element order)
- **Logical**: Correctly rejects unintended properties (deterministic index, negative index, value constraints, structural changes)

No spec weaknesses were detected in these 15 tests.
