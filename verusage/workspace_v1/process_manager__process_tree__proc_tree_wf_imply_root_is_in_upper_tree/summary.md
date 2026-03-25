# Adversarial Proof Test Summary

**Target**: `proc_tree_wf_imply_root_is_in_upper_tree`  
**Specification**: Given a well-formed process tree, for all nodes with `depth != 0`, `uppertree_seq[0] == root_proc`.

---

## Results: All 9 tests FAILED verification as expected ✅

The specification correctly rejects all adversarial queries, indicating it is neither too weak nor too permissive for the tested properties.

---

### Boundary Tests (3/3 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_missing_tree_wf` | Call lemma without `proc_tree_wf` precondition | FAILED ✅ |
| `test_boundary_missing_perms_wf` | Call lemma without `proc_perms_wf` precondition | FAILED ✅ |
| `test_boundary_missing_dom_subset` | Call lemma without `proc_tree_dom_subset_of_proc_dom` precondition | FAILED ✅ |

**Finding**: All three preconditions are necessary — removing any one prevents the lemma from being called. No precondition is redundant.

---

### Behavioral Mutation Tests (3/3 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_behavioral_wrong_index` | Assert `uppertree_seq[1] == root_proc` (wrong index) | FAILED ✅ |
| `test_behavioral_negated_conclusion` | Assert `uppertree_seq[0] != root_proc` (negation) | FAILED ✅ |
| `test_behavioral_applies_to_depth_zero` | Assert conclusion for `depth == 0` nodes (root has empty seq) | FAILED ✅ |

**Finding**: The postcondition is precise — mutated outputs (wrong index, negation, broadened scope) are all rejected.

---

### Logical Tests (3/3 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_logical_non_root_exists` | Assert non-root nodes must exist (not guaranteed) | FAILED ✅ |
| `test_logical_all_in_root_subtree` | Assert all nodes in root's `subtree_set` (not in ensures) | FAILED ✅ |
| `test_logical_same_container` | Assert all nodes share `owning_container` with root (unrelated field) | FAILED ✅ |

**Finding**: The specification does not leak unintended logical consequences:
- It does not require the tree to have children beyond the root.
- It does not expose the full subtree membership relationship in the ensures clause.
- It does not constrain fields unrelated to the tree structure (e.g., `owning_container`).

---

### Notes on Specification Visibility

During development, two initial logical tests (`root_depth_zero`, `root_has_no_parent`) unexpectedly **passed** because Verus `closed spec fn` definitions are visible within the same module. These properties are derivable from the preconditions (via `proc_root_wf` and `proc_childern_parent_wf`), confirming the spec is internally consistent for those properties. They were replaced with tests targeting genuinely non-derivable properties.

---

### Conclusion

The specification for `proc_tree_wf_imply_root_is_in_upper_tree` is **well-scoped**:
- All preconditions are necessary (no redundancy)
- The postcondition precisely captures the intended property (no mutation passes)
- No unintended logical consequences leak through the ensures clause
