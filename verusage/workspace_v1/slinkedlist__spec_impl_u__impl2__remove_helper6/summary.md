# Adversarial Proof Test Summary

## Target: `slinkedlist__spec_impl_u__impl2__remove_helper6.rs`
**Function**: `StaticLinkedList::remove_helper6` — removes the tail element from a static linked list, returning it and appending its node to the free list.

---

## Results Overview

| Category | Tests | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary | 5 | 5 ✅ | 0 |
| Behavioral Mutation | 5 | 5 ✅ | 0 |
| Logical | 5 | 5 ✅ | 0 |
| **Total** | **15** | **15** | **0** |

**Conclusion**: All 15 adversarial tests were correctly **rejected** by the specification. The spec appears consistent — it does not entail unintended properties for the tested queries.

---

## Boundary Tests (`boundary_tests.rs`)

Each test violates a single precondition and attempts to call `remove_helper6`. All correctly fail at the call site.

| # | Test | Violated Precondition | Result |
|---|---|---|---|
| 1 | `test_boundary_missing_wf` | `old(self).wf()` omitted | ❌ FAIL |
| 2 | `test_boundary_value_not_contained` | `!old(self)@.contains(v@)` | ❌ FAIL |
| 3 | `test_boundary_single_element` | `value_list_len == 1` | ❌ FAIL |
| 4 | `test_boundary_empty_free_list` | `free_list_len == 0` | ❌ FAIL |
| 5 | `test_boundary_not_tail` | `value_list_tail != remove_index` | ❌ FAIL |

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

Each test calls `remove_helper6` with valid inputs but asserts a **mutated** postcondition. All correctly fail.

| # | Test | Mutated Postcondition | Result |
|---|---|---|---|
| 1 | `test_mutation_length_unchanged` | `sll@.len() == old(sll)@.len()` | ❌ FAIL |
| 2 | `test_mutation_length_decreased_by_two` | `sll@.len() == old(sll)@.len() - 2` | ❌ FAIL |
| 3 | `test_mutation_wrong_return` | `ret != v@` | ❌ FAIL |
| 4 | `test_mutation_not_wf` | `!sll.wf()` | ❌ FAIL |
| 5 | `test_mutation_length_increased` | `sll@.len() > old(sll)@.len()` | ❌ FAIL |

---

## Logical Tests (`logical_tests.rs`)

Each test asserts a property that is plausible but **not explicitly guaranteed** by the postconditions. All correctly fail.

| # | Test | Non-guaranteed Property | Result |
|---|---|---|---|
| 1 | `test_logical_first_element_preserved` | `sll@[0] == old(sll)@[0]` | ❌ FAIL |
| 2 | `test_logical_old_list_long` | `old(sll)@.len() > 8` | ❌ FAIL |
| 3 | `test_logical_free_head_is_remove_index` | `free_list_head == remove_index` | ❌ FAIL |
| 4 | `test_logical_head_unchanged` | `value_list_head` preserved | ❌ FAIL |
| 5 | `test_logical_removed_still_present` | `sll@.contains(ret)` | ❌ FAIL |

---

## Notable Findings (from initial iteration)

During initial testing, 3 logical tests **unexpectedly passed** verification, revealing the spec is stronger than surface-level analysis suggests:

1. **`free_list_len == old.free_list_len + 1`** — The spec entails the free list grows by exactly 1.
2. **`ret == old(sll)@[old(sll)@.len() - 1]`** — The spec entails the removed value was the last element.
3. **`value_list_len == old.value_list_len - 1`** — The spec entails the concrete field tracks the abstract length.

These properties are derivable because Verus can reason about `closed spec fn` definitions within the same module, connecting internal invariants (e.g., `spec_seq_wf`, `wf`) with the postconditions. These were replaced with harder non-entailed properties in the final test suite.
