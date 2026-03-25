# Adversarial Test Summary: `slinkedlist__spec_impl_u__impl2__pop`

## Target: `StaticLinkedList::pop()` and helper functions

## Results: 15/15 tests FAILED as expected ✅

The specification correctly rejects all adversarial queries, indicating strong semantic boundaries.

---

### Boundary Tests (5/5 failed ✓)

| Test | Violated Precondition | Result |
|---|---|---|
| `test_pop_missing_wf` | `old(self).wf()` not provided | ✓ REJECTED |
| `test_pop_empty_list` | `old(self).len() == 0` (requires `> 0`) | ✓ REJECTED |
| `test_pop_twice_from_singleton` | Second pop on empty list (`len == 0` after first pop) | ✓ REJECTED |
| `test_get_value_negative_index` | `index == -1` (requires `0 <= index`) | ✓ REJECTED |
| `test_get_next_out_of_bounds` | `index == N` (requires `index < N`) | ✓ REJECTED |

### Behavioral Mutation Tests (5/5 failed ✓)

| Test | Mutated Property | Result |
|---|---|---|
| `test_pop_length_unchanged` | `len == old_len` (should be `old_len - 1`) | ✓ REJECTED |
| `test_pop_returns_second` | `ret.0 == old@[1]` (should be `old@[0]`) | ✓ REJECTED |
| `test_pop_seq_unchanged` | `self@ == old@` (should be `old@.skip(1)`) | ✓ REJECTED |
| `test_pop_length_increases` | `len == old_len + 1` (should be `old_len - 1`) | ✓ REJECTED |
| `test_pop_wrong_skip` | `self@ == old@.skip(2)` (should be `.skip(1)`) | ✓ REJECTED |

### Logical Tests (5/5 failed ✓)

| Test | Unentailed Property | Result |
|---|---|---|
| `test_pop_index_is_zero` | Returned index always `== 0` (not guaranteed; depends on `get_node_ref`) | ✓ REJECTED |
| `test_pop_result_still_contained` | Popped value remains in list (false: unique + skip removes it) | ✓ REJECTED |
| `test_double_pop_same_value` | Two pops return same value (false: distinct head elements) | ✓ REJECTED |
| `test_pop_returns_min` | Popped value `<=` all remaining (no ordering guarantee) | ✓ REJECTED |
| `test_pop_free_list_len_unchanged` | `free_list_len` unchanged after pop (not publicly entailed) | ✓ REJECTED |

---

## Observations

1. **Preconditions are tight**: The spec correctly rejects missing `wf()`, empty lists, out-of-bounds indices, and sequential precondition violations.

2. **Behavioral mutations are caught**: All five mutations to the postcondition (wrong length, wrong return value, wrong sequence, wrong direction, wrong skip amount) are detected.

3. **Logical boundaries are sound**: The spec does not over-entail — it refuses to prove index constants, containment of removed elements, ordering properties, or internal structural claims not in the public ensures.

4. **Note on `closed spec fn`**: Within the same compilation unit, `wf()` (a `closed spec fn`) is visible to the solver — so `unique()` IS derivable from `wf()`. This is expected Verus behavior (closed hides across module boundaries, not within the same file).

## Conclusion

The specification for `StaticLinkedList::pop()` is **consistent**: it rejects all 15 adversarial queries across boundary, behavioral, and logical dimensions. No specification weaknesses were detected.
