# Adversarial Test Results: `remove_helper1`

**Target**: `slinkedlist__spec_impl_u__impl2__remove_helper1.rs`  
**Function**: `StaticLinkedList::remove_helper1` — removes the sole element from a 1-element static linked list.

## Summary

All **15 adversarial tests** across 3 categories **FAILED verification as expected**, confirming the specification correctly rejects invalid inputs, incorrect behaviors, and unwarranted logical inferences.

---

## Boundary Tests (`boundary_tests.rs`) — 5/5 FAILED ✓

| # | Test | Violated Precondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_boundary_missing_wf` | Missing `old(self).wf()` | ❌ precondition not satisfied |
| 2 | `test_boundary_value_not_contained` | `!old(self)@.contains(v@)` | ❌ precondition not satisfied |
| 3 | `test_boundary_multi_element_list` | `value_list_len == 2` (should be 1) | ❌ precondition not satisfied |
| 4 | `test_boundary_wrong_remove_index` | `get_node_ref(v@) != remove_index` | ❌ precondition not satisfied |
| 5 | `test_boundary_get_value_negative_index` | `index = -1` (violates `0 <= index`) | ❌ precondition not satisfied |

**Note**: An earlier version tested `value_list_len == 0`, which created a vacuously contradictory precondition set (`wf() && contains(v@) && value_list_len == 0` is unsatisfiable because the verifier can unfold `closed spec fn wf()` within the same compilation unit). Fixed to use `value_list_len == 2`.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 5/5 FAILED ✓

| # | Test | Mutated Property | Result |
|---|------|------------------|--------|
| 1 | `test_mutation_wrong_return_value` | `ret != v@` (spec says `ret == v@`) | ❌ assertion failed |
| 2 | `test_mutation_length_unchanged` | `len == 1` after remove (spec says `len == old_len - 1`) | ❌ assertion failed |
| 3 | `test_mutation_result_not_wf` | `!self.wf()` (spec ensures `self.wf()`) | ❌ assertion failed |
| 4 | `test_mutation_element_still_present` | `self@.contains(ret)` (spec ensures removal) | ❌ assertion failed |
| 5 | `test_mutation_uniqueness_violated` | `!self.unique()` (spec ensures uniqueness) | ❌ assertion failed |

---

## Logical Tests (`logical_tests.rs`) — 5/5 FAILED ✓

| # | Test | Unwarranted Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_result_nonempty_after_remove` | Non-empty result after removing sole element | ❌ assertion failed |
| 2 | `test_logical_structural_equivalence` | Same view ⟹ same `value_list_head` | ❌ assertion failed |
| 3 | `test_logical_wf_no_value_constraint` | `wf()` constrains element values (`sll@[0] > 0`) | ❌ assertion failed |
| 4 | `test_logical_determinism_internal_state` | Two results matching postconditions share `free_list_head` | ❌ assertion failed |
| 5 | `test_logical_remove_creates_elements` | `remove_value` introduces elements not in original | ❌ assertion failed |

---

## Conclusion

The specification of `remove_helper1` is **consistent** across all three semantic dimensions tested:

1. **Input validation**: All preconditions (`wf`, `contains`, `value_list_len == 1`, `get_node_ref == index`) are properly enforced — no invalid call is accepted.
2. **Output correctness**: All postconditions (`wf`, length decrease, return value, view update, uniqueness) are tight — no incorrect behavior is admitted.
3. **Logical boundaries**: The spec does not entail structural determinism, value constraints, or element creation — the semantic boundary is properly controlled.

**Observation**: `closed spec fn wf()` is transparently unfolded by the verifier within the same `verus!{}` block, so tests relying on its opacity must avoid contradictory precondition sets.
