# Adversarial Proof Test Summary

**Target**: `slinkedlist__spec_impl_u__impl2__init.rs`
**Functions tested**: `init`, `set_value`, `set_next`, `set_prev`, `len`

## Results

| Test File | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| boundary_tests.rs | 12 | 12 ✅ | 0 |
| behavioral_mutation_tests.rs | 12 | 12 ✅ | 0 |
| logical_tests.rs | 12 | 12 ✅ | 0 |
| **Total** | **36** | **36** | **0** |

**All 36 adversarial tests were correctly rejected by the specification.**

## Boundary Tests (12/12 FAIL ✅)

| # | Test | Property Violated |
|---|---|---|
| 1 | `test_boundary_n_equals_2` | N > 2 (strict boundary) |
| 2 | `test_boundary_n_equals_0` | N > 2 (zero edge case) |
| 3 | `test_boundary_n_equals_1` | N > 2 (below boundary) |
| 4 | `test_boundary_n_at_i32_max` | N < SLLIndex::MAX |
| 5 | `test_boundary_init_len_positive` | init ensures len == 0 |
| 6 | `test_boundary_init_seq_nonempty` | init ensures Seq::empty() |
| 7 | `test_boundary_free_index_at_n` | free_list indices in [0, N) |
| 8 | `test_boundary_index_below_sentinel` | SLLIndex >= -1 for valid values |
| 9 | `test_boundary_partition_mismatch` | free_list_len + value_list_len == N |
| 10 | `test_boundary_array_size_mismatch` | arr_seq.len() == N |
| 11 | `test_boundary_len_equals_n` | len == 0 contradicts len == N |
| 12 | `test_boundary_set_index_exceeds_arr_len` | index within array bounds |

## Behavioral Mutation Tests (12/12 FAIL ✅)

| # | Test | Mutated Postcondition |
|---|---|---|
| 1 | `test_mutation_view_nonempty` | spec_seq length 0 → >0 |
| 2 | `test_mutation_len_is_one` | len 0 → 1 |
| 3 | `test_mutation_view_equals_singleton` | empty seq → [42] |
| 4 | `test_mutation_set_value_head_changes` | value_list_head preserved → changed |
| 5 | `test_mutation_set_value_spec_seq_changes` | spec_seq preserved → changed |
| 6 | `test_mutation_set_next_free_len_changes` | free_list_len preserved → changed |
| 7 | `test_mutation_set_prev_tail_changes` | value_list_tail preserved → changed |
| 8 | `test_mutation_set_value_wrong_value` | value at index == v → ≠ v |
| 9 | `test_mutation_set_next_wrong_next` | next at index == v → ≠ v |
| 10 | `test_mutation_set_prev_wrong_prev` | prev at index == v → ≠ v |
| 11 | `test_mutation_set_value_other_node_changes` | other nodes preserved → changed |
| 12 | `test_mutation_free_list_len_zero` | free_list_len == N → 0 (with partition constraint) |

## Logical Tests (12/12 FAIL ✅)

| # | Test | Unintended Property Queried |
|---|---|---|
| 1 | `test_logical_determinism_free_head` | Determinism of internal state |
| 2 | `test_logical_value_list_len_from_view` | value_list@.len() from view alone |
| 3 | `test_logical_free_list_len_from_value_len` | free_list_len from value_list_len alone |
| 4 | `test_logical_all_values_none` | All node values None (not in ensures) |
| 5 | `test_logical_free_list_first_element` | free_list@[0] == 0 (internal detail) |
| 6 | `test_logical_spec_len_from_value_list_len` | spec_len from value_list_len (different fields) |
| 7 | `test_logical_same_view_same_len` | Same view → same value_list_len |
| 8 | `test_logical_set_value_free_indices_valid` | Unconstrained free_list entries valid |
| 9 | `test_logical_init_unique_free_tail` | Determinism of free_list_tail |
| 10 | `test_logical_free_list_head_nonneg` | free_list_head >= 0 (not derivable) |
| 11 | `test_logical_not_unique_after_init` | ¬unique() for empty seq (vacuously true) |
| 12 | `test_logical_prev_always_ge_neg1` | Unconstrained i32 >= -1 |

## Conclusion

The specification for `StaticLinkedList::init` and its helper functions (`set_value`, `set_next`, `set_prev`) is **consistent** with respect to all 36 adversarial queries:

- **Preconditions** correctly reject invalid inputs (N ≤ 2, N ≥ i32::MAX, out-of-bounds indices).
- **Postconditions** correctly reject mutated behaviors (wrong lengths, changed metadata, incorrect field values).
- **Logical boundaries** are sound: the spec does not entail determinism of internal state, cross-field implications without `wf()`, or properties only established in the implementation body but not exported in `ensures`.

The `closed spec fn wf()` effectively encapsulates internal invariants, preventing callers from reasoning about implementation details (free_list ordering, node values, head/tail pointers) beyond what is explicitly guaranteed.
