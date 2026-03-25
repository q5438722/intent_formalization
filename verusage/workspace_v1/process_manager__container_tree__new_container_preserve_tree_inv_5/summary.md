# Adversarial Test Summary: `new_container_preserve_tree_inv_5`

## Target Specification
- **Proof function**: `new_container_preserve_tree_inv_5`
- **Requires**: `new_container_ensures(root, old_perms, new_perms, c_ptr, new_c_ptr)` — open spec encoding all constraints for adding a new container to a tree
- **Ensures**: `container_uppertree_seq_wf(root, new_perms)` — closed spec for uppertree sequence well-formedness

## Results: All 15 tests FAIL verification ✅

| # | Category | Test Name | Failure Mode | Result |
|---|----------|-----------|-------------|--------|
| 1 | Boundary | `test_boundary_same_ptr` | `container_ptr == new_container_ptr` violates domain disjointness | FAIL ✅ |
| 2 | Boundary | `test_boundary_depth_overflow` | `depth == usize::MAX` violates `depth < usize::MAX` | FAIL ✅ |
| 3 | Boundary | `test_boundary_children_full` | `children.len() >= PROC_CHILD_LIST_LEN` violates capacity | FAIL ✅ |
| 4 | Boundary | `test_boundary_container_not_in_domain` | `container_ptr` not in old domain | FAIL ✅ |
| 5 | Boundary | `test_boundary_no_tree_wf` | Missing `container_tree_wf` precondition | FAIL ✅ |
| 6 | Behavioral | `test_behavioral_wrong_depth` | Assert depth = parent+2 (should be +1) | FAIL ✅ |
| 7 | Behavioral | `test_behavioral_nonempty_children` | Assert non-empty children (should be empty) | FAIL ✅ |
| 8 | Behavioral | `test_behavioral_parent_children_unchanged` | Assert children count unchanged (should be +1) | FAIL ✅ |
| 9 | Behavioral | `test_behavioral_wrong_parent` | Assert parent is None (should be Some) | FAIL ✅ |
| 10 | Behavioral | `test_behavioral_nonempty_subtree_set` | Assert non-empty subtree_set (should be empty) | FAIL ✅ |
| 11 | Logical | `test_logical_full_tree_wf_from_partial` | Derive full `container_tree_wf` from `uppertree_seq_wf` | FAIL ✅ |
| 12 | Logical | `test_logical_linkedlist_wf_not_entailed` | Derive `containers_linkedlist_wf` from `uppertree_seq_wf` | FAIL ✅ |
| 13 | Logical | `test_logical_arbitrary_uppertree_wf` | Assert `uppertree_seq_wf` without proof function | FAIL ✅ |
| 14 | Logical | `test_logical_determinism` | Two different insertions produce identical states | FAIL ✅ |
| 15 | Logical | `test_logical_self_in_uppertree` | New container's uppertree contains itself | FAIL ✅ |

## Findings

### Specification Strength
The specification is **well-constrained** — all 15 adversarial tests are properly rejected:
- **Boundary**: All 5 precondition violations are caught. Invalid inputs (same pointer, overflow, full children, missing domain membership, no tree WF) cannot satisfy `new_container_ensures`.
- **Behavioral**: All 5 output mutations are rejected. The open spec `new_container_ensures` precisely constrains depth (+1), children (empty for new, +1 for parent), parent (Some), and subtree_set (empty).
- **Logical**: All 5 unintended properties are rejected. The proof only guarantees `container_uppertree_seq_wf`, not the full tree invariant or other sub-invariants like `containers_linkedlist_wf`.

### Notable Observations During Development
During initial testing, two logical tests unexpectedly **passed**:
1. `container_subtree_set_exclusive(root, new_perms)` — derivable from precondition + postcondition
2. `container_root_wf(root, new_perms)` — derivable from precondition + postcondition
3. `container_childern_depth_wf(root, new_perms)` — derivable from precondition + postcondition

These are **correct entailments** (not spec weaknesses): the combination of `new_container_ensures` + `container_uppertree_seq_wf` is strong enough to re-derive these sub-invariants. However, `containers_linkedlist_wf` and `container_childern_parent_wf` are NOT derivable from the postcondition alone, confirming the proof function has a well-scoped guarantee.

### Conclusion
The specification for `new_container_preserve_tree_inv_5` is **consistent with respect to the tested semantic queries**. It correctly:
- Rejects all tested invalid inputs (boundary)
- Rejects all tested incorrect behaviors (behavioral mutation)
- Does not entail unintended logical properties (logical)
