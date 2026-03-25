# Adversarial Test Results: `remove_container_preserve_tree_inv_2`

## Target

**Lemma**: `remove_container_preserve_tree_inv_2`  
**Precondition**: `remove_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr)`  
**Postcondition**: `container_childern_parent_wf(root_container, new_container_perms)`

This lemma proves that removing a leaf container (no children, not root) from a container tree preserves the parent-child well-formedness invariant.

---

## Results Summary

| # | File | Test Name | Type | Expected | Actual | Status |
|---|------|-----------|------|----------|--------|--------|
| 1 | boundary_tests.rs | `boundary_test_remove_root` | Boundary | FAIL | precondition not satisfied | ✅ |
| 2 | boundary_tests.rs | `boundary_test_not_in_domain` | Boundary | FAIL | precondition not satisfied | ✅ |
| 3 | boundary_tests.rs | `boundary_test_has_children` | Boundary | FAIL | precondition not satisfied | ✅ |
| 4 | behavioral_tests.rs | `behavioral_test_container_still_exists` | Behavioral | FAIL | assertion failed | ✅ |
| 5 | behavioral_tests.rs | `behavioral_test_parent_children_unchanged` | Behavioral | FAIL | assertion failed | ✅ |
| 6 | behavioral_tests.rs | `behavioral_test_depth_mutated` | Behavioral | FAIL | assertion failed | ✅ |
| 7 | logical_tests.rs | `logical_test_full_tree_wf` | Logical | FAIL | assertion failed | ✅ |
| 8 | logical_tests.rs | `logical_test_owned_procs_preserved` | Logical | FAIL | assertion failed | ✅ |
| 9 | logical_tests.rs | `logical_test_scheduler_preserved` | Logical | FAIL | assertion failed | ✅ |

**All 9/9 tests FAILED verification as expected.** The specification correctly rejects all adversarial queries.

---

## Detailed Analysis

### Boundary Tests (3/3 rejected ✅)

These tests violate preconditions of `remove_container_ensures`:

1. **Remove root**: Passing `root_container` as the container to remove violates `container_ptr != root_container`.
2. **Not in domain**: Using a `container_ptr` not in `old_container_perms.dom()` violates the domain membership requirement.
3. **Has children**: Using a container with non-empty children violates the `children@ == Seq::empty()` requirement.

**Conclusion**: The preconditions are necessary — weakening any of them prevents the postcondition from being proven.

### Behavioral Mutation Tests (3/3 rejected ✅)

These tests assume valid preconditions but assert incorrect output relationships:

4. **Container still exists**: Asserts the removed container is still in the new domain. Contradicted by `new_domain == old_domain.remove(container_ptr)`.
5. **Parent children unchanged**: Asserts the parent's children list wasn't modified. Contradicted by the explicit `remove_value` update.
6. **Depth mutated**: Asserts a remaining container's depth changed. Contradicted by the frame condition preserving depth.

**Conclusion**: The specification correctly captures the expected behavioral changes from container removal.

### Logical Tests (3/3 rejected ✅)

These tests assert properties not explicitly guaranteed by the postcondition:

7. **Full tree_wf**: The lemma only proves `container_childern_parent_wf`, not the full `container_tree_wf` (which is a conjunction of 7 sub-invariants). This correctly fails.
8. **owned_procs preserved**: The spec's frame conditions cover `parent`, `parent_rev_ptr`, `children`, `depth`, `uppertree_seq`, and `subtree_set`, but NOT `owned_procs`. This correctly fails.
9. **scheduler preserved**: Similarly, `scheduler` is not covered by the frame conditions. This correctly fails.

**Conclusion**: The lemma's postcondition is appropriately scoped — it only guarantees what it claims (parent-child wf), not the full tree invariant or unrelated container fields.

---

## Notable Finding

During initial testing, `containers_linkedlist_wf` and `container_subtree_set_exclusive` were tested as logical queries expected to fail, but they **passed** verification. This means the precondition `remove_container_ensures` combined with the postcondition `container_childern_parent_wf` is strong enough to derive these properties. This indicates the specification is well-designed — the frame conditions in `remove_container_ensures` preserve enough structure to maintain these invariants.

The specification's **true semantic gap** lies in:
- Not guaranteeing the **full** tree invariant (other sub-invariants like `container_subtree_set_wf` via the postcondition alone)
- Not preserving **operational fields** (`owned_procs`, `scheduler`, `quota`, etc.) — these are outside the scope of tree-structural reasoning
