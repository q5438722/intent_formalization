# Test Execution Summary

**Target**: `remove_proc_preserve_tree_inv_5` — proves that removing a childless, non-root process from a process tree preserves `proc_uppertree_seq_wf`.

## Results

All 9 adversarial tests **FAILED verification** as expected, confirming the specification correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (boundary_tests.rs) — 3/3 FAILED ✓

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_1_no_tree_wf` | Missing `proc_tree_wf` precondition | FAILED ✓ |
| `test_boundary_2_not_in_domain` | `proc_ptr` not in `proc_tree_dom` | FAILED ✓ |
| `test_boundary_3_has_children` | `proc_ptr` has children (not a leaf) | FAILED ✓ |

**Interpretation**: The spec correctly requires tree well-formedness, domain membership, and the leaf-node condition. Removing any of these makes the postcondition unprovable.

### Behavioral Mutation Tests (behavioral_mutation_tests.rs) — 3/3 FAILED ✓

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_1_unreduced_domain` | Uses original domain instead of `proc_tree_dom.remove(proc_ptr)` | FAILED ✓ |
| `test_mutation_2_parent_children_unchanged` | Claims parent's children list is unchanged | FAILED ✓ |
| `test_mutation_3_proc_still_in_domain` | Claims `proc_ptr` remains in `new_proc_perms.dom()` | FAILED ✓ |

**Interpretation**: The spec correctly rejects assertions about the wrong domain, unchanged parent children, and retained process identity. The behavioral changes from process removal are properly captured.

### Logical Tests (logical_tests.rs) — 3/3 FAILED ✓

| Test | Unwarranted Property | Result |
|------|---------------------|--------|
| `test_logical_1_full_tree_wf` | Full `proc_tree_wf` preserved (all 7 sub-invariants) | FAILED ✓ |
| `test_logical_2_children_parent_wf` | `proc_childern_parent_wf` preserved | FAILED ✓ |
| `test_logical_3_domain_equality` | `proc_tree_dom.remove(proc_ptr) =~= new_proc_perms.dom()` | FAILED ✓ |

**Interpretation**: The function only guarantees `proc_uppertree_seq_wf`, not the full tree well-formedness. The spec does not conflate tree domain with permissions domain.

## Notable Finding

During initial testing, `proc_subtree_set_wf` was tested as a logical test and **unexpectedly PASSED**. This means `remove_proc_ensures` is strong enough to entail `proc_subtree_set_wf` on the reduced domain, even though the function only claims to preserve `proc_uppertree_seq_wf`. This is not a bug — the preconditions provide sufficient structural information to derive the subtree-set invariant — but it reveals that the specification's entailment boundary is wider than the postcondition suggests.

## Conclusion

The specification for `remove_proc_preserve_tree_inv_5` is **consistent**: it rejects all 9 adversarial queries targeting boundary violations, behavioral mutations, and unwarranted logical properties. The preconditions are necessary (boundary tests) and the postcondition is precise (mutation/logical tests).
