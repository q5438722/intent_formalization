# Adversarial Test Summary: `no_child_imply_no_subtree`

## Target Specification

**Lemma**: `no_child_imply_no_subtree` — proves that if a container has no children (`children@ =~= Seq::empty()`), then its subtree set is empty (`subtree_set@ =~= Set::empty()`).

**Preconditions**: `container_perms_wf`, `container_tree_wf`, `c_ptr` in domain, empty children.

## Results: All 11 tests FAILED verification ✅

The specification correctly rejects all adversarial queries.

### Boundary Tests (4/4 failed — precondition violations rejected)

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_missing_domain_membership` | Omit `container_perms.dom().contains(c_ptr)` | ❌ precondition not satisfied |
| `test_boundary_nonempty_children` | Children contain an element (non-empty) | ❌ precondition not satisfied |
| `test_boundary_missing_perms_wf` | Omit `container_perms_wf` | ❌ precondition not satisfied |
| `test_boundary_missing_tree_wf` | Omit `container_tree_wf` | ❌ precondition not satisfied |

### Behavioral Mutation Tests (3/3 failed — incorrect behaviors rejected)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_subtree_contains_element` | Assert subtree contains arbitrary `s_ptr` | ❌ postcondition not satisfied |
| `test_mutation_subtree_contains_self` | Assert subtree contains `c_ptr` itself | ❌ postcondition not satisfied |
| `test_mutation_converse_direction` | Converse: empty subtree → empty children | ❌ postcondition not satisfied |

### Logical Tests (4/4 failed — unintended reasoning rejected)

| Test | Unwarranted Property | Result |
|------|---------------------|--------|
| `test_logical_empty_children_implies_root` | Empty children → is root container | ❌ postcondition not satisfied |
| `test_logical_empty_children_implies_zero_depth` | Empty children → depth == 0 | ❌ postcondition not satisfied |
| `test_logical_empty_children_implies_no_parent` | Empty children → no parent | ❌ postcondition not satisfied |
| `test_logical_overgeneralize_to_other_container` | Result extends to unrelated container | ❌ postcondition not satisfied |

## Conclusion

The specification for `no_child_imply_no_subtree` is **consistent** across all three query dimensions:

1. **Boundary**: All four preconditions are necessary — removing any one causes verification failure.
2. **Behavioral**: The postcondition precisely captures the guaranteed property; mutated outputs and the converse direction are correctly rejected.
3. **Logical**: The spec does not over-entail — it does not imply unrelated structural properties (root identity, depth, parent existence) or overgeneralize to other containers.

No spec weaknesses were detected.
