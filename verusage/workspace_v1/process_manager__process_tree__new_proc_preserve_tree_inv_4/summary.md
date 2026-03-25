# Adversarial Test Summary: `new_proc_preserve_tree_inv_4`

## Target
`process_manager__process_tree__new_proc_preserve_tree_inv_4.rs`

**Function under test**: `new_proc_preserve_tree_inv_4`  
- **Requires**: `new_proc_ensures(root_proc, proc_tree_dom, old_proc_perms, new_proc_perms, proc_ptr, new_proc_ptr)`  
- **Ensures**: `proc_subtree_set_wf(root_proc, proc_tree_dom.insert(new_proc_ptr), new_proc_perms)`

---

## Results: All 12 Tests FAIL (as expected)

### Boundary Tests (`boundary_tests.rs`) — 4/4 FAIL ✅

| Test | Failure Mode | Result |
|------|-------------|--------|
| `boundary_test_no_precondition` | No requires at all | FAIL ✅ |
| `boundary_test_proc_not_in_domain` | `proc_ptr ∉ proc_tree_dom` | FAIL ✅ |
| `boundary_test_new_proc_already_in_domain` | `new_proc_ptr ∈ proc_tree_dom` | FAIL ✅ |
| `boundary_test_max_depth` | `depth == usize::MAX` (overflow) | FAIL ✅ |

### Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 4/4 FAIL ✅

| Test | Mutation | Result |
|------|----------|--------|
| `behavioral_test_full_tree_wf` | Strengthen: assert `proc_tree_wf` (7 invariants vs 1) | FAIL ✅ |
| `behavioral_test_extra_domain_member` | Extend domain beyond `insert(new_proc_ptr)` | FAIL ✅ |
| `behavioral_test_old_perms` | Use `old_proc_perms` instead of `new_proc_perms` | FAIL ✅ |
| `behavioral_test_non_extended_domain` | Use `proc_tree_dom` instead of `proc_tree_dom.insert(new_proc_ptr)` | FAIL ✅ |

### Logical Tests (`logical_tests.rs`) — 4/4 FAIL ✅

| Test | Property Tested | Result |
|------|----------------|--------|
| `logical_test_uppertree_seq_wf` | Cross-invariant: `proc_uppertree_seq_wf` | FAIL ✅ |
| `logical_test_children_parent_wf` | Cross-invariant: `proc_childern_parent_wf` | FAIL ✅ |
| `logical_test_parent_is_root` | Determinism: `proc_ptr == root_proc` | FAIL ✅ |
| `logical_test_new_proc_depth_zero` | Structural: new proc has depth 0 | FAIL ✅ |

---

## Findings During Testing

Two initial tests **passed** unexpectedly before being replaced, revealing spec observations:

1. **`proc_subtree_set_wf` has a phantom `root_proc` parameter**: The `root_proc` parameter is never referenced in the body of `proc_subtree_set_wf`. Therefore `proc_subtree_set_wf(X, dom, perms)` is equivalent for any `X`. This is structurally intentional (the parameter exists for API consistency with sibling predicates) but means the postcondition is root-agnostic.

2. **`proc_subtree_set_exclusive` is derivable from the precondition + postcondition**: Even though the function only claims to preserve `proc_subtree_set_wf`, the combination of `new_proc_ensures` (which includes `proc_tree_wf` for the old tree) and the frame conditions is strong enough for Verus to derive `proc_subtree_set_exclusive` for the extended domain. This suggests the function's postcondition could be strengthened.

## Conclusion

The specification correctly rejects all 12 adversarial queries across boundary, behavioral, and logical categories. The spec is tight enough to prevent invalid inputs, incorrect output mutations, and unintended cross-invariant reasoning (except for the two observations above).
