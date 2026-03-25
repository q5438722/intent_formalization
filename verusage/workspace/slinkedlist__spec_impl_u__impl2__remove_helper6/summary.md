# Test Execution Summary: `remove_helper6` Specification Consistency

**Target**: `slinkedlist__spec_impl_u__impl2__remove_helper6.rs`  
**Function under test**: `StaticLinkedList<T,N>::remove_helper6(&mut self, remove_index, v) -> T`  
**Purpose**: Removes the tail element from a doubly-linked static list, moving its node to the free list.

---

## Results Overview

| Test File | Tests | Expected Failures | Actual Failures | Status |
|-----------|-------|-------------------|-----------------|--------|
| `boundary_tests.rs` | 5 | 5 | 5 | ✅ All FAIL as expected |
| `behavioral_mutation_tests.rs` | 5 | 5 | 5 | ✅ All FAIL as expected |
| `logical_tests.rs` | 5 | 5 | 5 | ✅ All FAIL as expected |

**Total: 15/15 tests correctly rejected by the specification.**

---

## Boundary Tests (Precondition Violations)

| # | Test | Violated Precondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_boundary_single_element` | `value_list_len == 1` (requires `!= 1`) | ❌ FAIL (correct) |
| 2 | `test_boundary_empty_free_list` | `free_list_len == 0` (requires `!= 0`) | ❌ FAIL (correct) |
| 3 | `test_boundary_not_tail` | `value_list_tail != remove_index` (requires `==`) | ❌ FAIL (correct) |
| 4 | `test_boundary_value_not_in_list` | `!contains(v@)` (requires `contains`) | ❌ FAIL (correct) |
| 5 | `test_boundary_wrong_index` | `get_node_ref(v@) != remove_index` (requires `==`) | ❌ FAIL (correct) |

**Conclusion**: All preconditions are properly enforced. Invalid inputs are correctly rejected.

---

## Behavioral Mutation Tests (Incorrect Output Relations)

| # | Test | Mutated Property | Result |
|---|------|------------------|--------|
| 1 | `test_mutation_length_unchanged` | `len == old_len` instead of `old_len - 1` | ❌ FAIL (correct) |
| 2 | `test_mutation_wrong_return_value` | `ret == 0` instead of `ret == v@` | ❌ FAIL (correct) |
| 3 | `test_mutation_sequence_unchanged` | `sll@ =~= old(sll)@` (no removal) | ❌ FAIL (correct) |
| 4 | `test_mutation_uniqueness_lost` | `!sll.unique()` (uniqueness broken) | ❌ FAIL (correct) |
| 5 | `test_mutation_length_increases` | `len == old_len + 1` (wrong direction) | ❌ FAIL (correct) |

**Conclusion**: All incorrect behavioral mutations are rejected. The spec correctly constrains output relations.

---

## Logical Tests (Unintended Entailment Probes)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_removed_value_still_present` | `sll@.contains(v@)` (removed value persists) | ❌ FAIL (correct) |
| 2 | `test_logical_head_preserved` | `value_list_head == old(value_list_head)` | ❌ FAIL (correct) |
| 3 | `test_logical_result_has_multiple_elements` | `len > 1` (stronger bound) | ❌ FAIL (correct) |
| 4 | `test_logical_free_head_unchanged` | `free_list_head == old(free_list_head)` | ❌ FAIL (correct) |
| 5 | `test_logical_size_changes` | `size != old(size)` (invariant violation) | ❌ FAIL (correct) |

**Conclusion**: The spec does not entail unintended logical properties through its explicit postconditions.

---

## Notable Finding: Implementation Body Leakage

During investigation, an earlier version of logical test 1 asserted `sll.free_list_len == old(sll).free_list_len + 1`. This property is **not** in the postcondition but **passed** verification when using the real implementation body. When the same test was run against an `external_body` stub (postconditions only), it correctly **failed**.

**Root cause**: Verus shares the SMT context across functions in the same file. The implementation's `self.free_list_len = self.free_list_len + 1` statement leaks field-level mutation facts to callers, beyond what the ensures clause guarantees.

**Implication**: The specification of `remove_helper6` is **incomplete** regarding `free_list_len`. The postcondition does not explicitly state that `free_list_len` increases by 1, yet this fact is derivable from the implementation body in non-modular verification. This is a **spec weakness** — the postcondition should ideally include `self.free_list_len == old(self).free_list_len + 1` for full specification completeness, or rely on `wf()` being sufficient (which it would be if the caller could unfold it).
