# Test Summary: `remove_helper7` Specification Consistency

**Target**: `slinkedlist__spec_impl_u__impl2__remove_helper7.rs`
**Function**: `remove_helper7(&mut self, remove_index: SLLIndex, v: Ghost<T>) -> T`
**Purpose**: Removes a **middle** element (neither head nor tail) from a static linked list.

---

## Results Overview

| Test Category         | Total | Failed (as expected) | Passed (spec weakness) |
|----------------------|-------|---------------------|----------------------|
| Boundary Tests        | 6     | 6                   | 0                    |
| Behavioral Mutation   | 5     | 5                   | 0                    |
| Logical Tests         | 5     | 5                   | 0                    |
| **Total**             | **16**| **16**              | **0**                |

**Conclusion**: All 16 adversarial tests were correctly rejected by the specification. No specification weaknesses were detected.

---

## Boundary Tests (6/6 failed ✅)

| Test | Violated Precondition | Result |
|------|----------------------|--------|
| `test_boundary_missing_wf` | `wf()` not provided | `precondition not satisfied` ✅ |
| `test_boundary_value_not_contained` | `!self@.contains(v@)` | `precondition not satisfied` ✅ |
| `test_boundary_single_element` | `value_list_len == 1` | `precondition not satisfied` ✅ |
| `test_boundary_empty_free_list` | `free_list_len == 0` | `precondition not satisfied` ✅ |
| `test_boundary_removing_tail` | `value_list_tail == remove_index` | `precondition not satisfied` ✅ |
| `test_boundary_removing_head` | `value_list_head == remove_index` | `precondition not satisfied` ✅ |

## Behavioral Mutation Tests (5/5 failed ✅)

| Test | Mutated Postcondition | Result |
|------|----------------------|--------|
| `test_mutation_length_unchanged` | `len() == old_len` (should be `-1`) | `postcondition not satisfied` ✅ |
| `test_mutation_length_decreases_by_two` | `len() == old_len - 2` (should be `-1`) | `postcondition not satisfied` ✅ |
| `test_mutation_wrong_return_value` | `ret != v@` (should be `==`) | `postcondition not satisfied` ✅ |
| `test_mutation_seq_unchanged` | `self@ =~= old@` (should remove value) | `postcondition not satisfied` ✅ |
| `test_mutation_element_still_contained` | `self@.contains(ret)` (removed value should be absent) | `assertion failed` ✅ |

## Logical Tests (5/5 failed ✅)

| Test | Unentailed Property | Result |
|------|---------------------|--------|
| `test_logical_remove_index_is_zero` | `remove_index == 0` (index is arbitrary) | `assertion failed` ✅ |
| `test_logical_free_list_len_unchanged` | `free_list_len` unchanged (should increase) | `assertion failed` ✅ |
| `test_logical_removed_leq_new_first` | `ret <= self@[0]` (no ordering invariant) | `assertion failed` ✅ |
| `test_logical_head_unchanged` | `value_list_head` unchanged (not in postconditions) | `assertion failed` ✅ |
| `test_logical_stronger_length_bound` | `len() >= 3` (old could be 3 → new is 2) | `assertion failed` ✅ |

---

## Specification Strength Assessment

The `remove_helper7` specification is **well-constrained** for its intended use case:

1. **Preconditions** correctly guard all edge cases: non-well-formed lists, missing values, single-element lists, empty free lists, head/tail removal attempts.
2. **Postconditions** precisely capture the behavioral contract: length decreases by exactly 1, return value matches input, spec sequence reflects removal, and node references are preserved.
3. **No unintended entailments** were detected: the spec does not leak internal structure details (free list size, head pointer changes, ordering) through its public contract.

**Notable insight**: The preconditions `value_list_head != remove_index ∧ value_list_tail != remove_index` implicitly guarantee the list has ≥3 elements (since a 2-element list has every element as either head or tail). This means `len() >= 2` after removal is actually entailed, but `len() >= 3` is correctly rejected.
