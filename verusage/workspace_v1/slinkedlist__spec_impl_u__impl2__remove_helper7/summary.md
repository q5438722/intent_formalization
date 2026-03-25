# Adversarial Test Summary: `remove_helper7`

**Target**: `slinkedlist__spec_impl_u__impl2__remove_helper7.rs`  
**Function**: `StaticLinkedList::remove_helper7` — removes a middle node from a static linked list's value list and appends it to the free list.

## Results: 15/15 tests correctly FAILED ✅

The specification is **consistent** — it correctly rejects all adversarial queries.

---

## (1) Boundary Tests — 5/5 FAILED ✅

| Test | Violated Precondition | Result |
|------|----------------------|--------|
| `test_boundary_no_wf` | Missing `wf()` | precondition not satisfied |
| `test_boundary_value_not_in_list` | `!@.contains(v@)` | precondition not satisfied |
| `test_boundary_single_element` | `value_list_len == 1` | precondition not satisfied |
| `test_boundary_empty_free_list` | `free_list_len == 0` | precondition not satisfied |
| `test_boundary_remove_head` | `value_list_head == remove_index` | precondition not satisfied |

**Conclusion**: All preconditions are necessary and correctly enforced. Invalid inputs are rejected.

---

## (2) Behavioral Mutation Tests — 5/5 FAILED ✅

| Test | Mutated Property | Result |
|------|-----------------|--------|
| `test_mutation_length_unchanged` | `len' == len` (should be `len - 1`) | assertion failed |
| `test_mutation_wrong_return_value` | `ret != v@` (should be `==`) | assertion failed |
| `test_mutation_still_contains_removed` | `@.contains(ret)` (should not) | assertion failed |
| `test_mutation_sequence_unchanged` | `@' == @` (element removed) | assertion failed |
| `test_mutation_length_minus_two` | `len' == len - 2` (should be `- 1`) | assertion failed |

**Conclusion**: Postconditions are strong enough to reject all tested behavioral mutations. The spec correctly constrains length, return value, containment, and sequence identity.

---

## (3) Logical Tests — 5/5 FAILED ✅

| Test | Unguaranteed Property | Result |
|------|----------------------|--------|
| `test_logical_free_head_preserved` | `free_list_head' == free_list_head` | assertion failed |
| `test_logical_head_preserved` | `value_list_head' == value_list_head` | assertion failed |
| `test_logical_free_tail_is_removed_index` | `free_list_tail' == remove_index` | assertion failed |
| `test_logical_tail_preserved` | `value_list_tail' == value_list_tail` | assertion failed |
| `test_logical_result_is_prefix` | `@' == @.subrange(0, len-1)` | assertion failed |

**Conclusion**: The spec does NOT entail internal structural properties (head/tail pointer preservation, free list structure, prefix assumption). These are implementation details not exposed through the postconditions — a sign of good abstraction.

---

## Notable Observations

During development, two initially-designed logical tests **passed** verification unexpectedly:
- `free_list_len == old_free_list_len + 1` — derivable via `wf()` invariant `free_list_len + value_list_len == N`
- `size == old_size` — derivable via `array_wf()` inside `wf()` (both old and new `size == N`)

These passed because `closed spec fn` bodies are visible within the same module in Verus. This reveals the spec is **stronger than the postconditions alone suggest** — the `wf()` invariant provides additional deductive power. These were replaced with truly non-derivable properties (head/tail pointer preservation).

## Overall Assessment

The `remove_helper7` specification is **well-formed and consistent**:
- **Preconditions** correctly gate all 5 required conditions
- **Postconditions** are strong enough to reject incorrect behaviors
- **Abstraction boundary** properly hides implementation details behind `wf()`
