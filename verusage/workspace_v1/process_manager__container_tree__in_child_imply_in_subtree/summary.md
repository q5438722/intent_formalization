# Adversarial Test Summary: `in_child_imply_in_subtree`

## Target
`process_manager__container_tree__in_child_imply_in_subtree.rs`

**Function under test**: `in_child_imply_in_subtree` — proves that if `child_ptr` is in the children list of `c_ptr`, then `child_ptr` is in the subtree set of `c_ptr`.

---

## Results Overview

| Test Category         | Tests | Failed (expected) | Passed (unexpected) |
|-----------------------|-------|--------------------|---------------------|
| Boundary Tests        | 5     | 5 ✅               | 0                   |
| Behavioral Mutations  | 5     | 5 ✅               | 0                   |
| Logical Tests         | 5     | 5 ✅               | 0                   |
| **Total**             | **15**| **15 ✅**          | **0**               |

All 15 adversarial tests **failed verification**, which is the expected outcome. The specification is strong enough to reject all tested invalid properties.

---

## Boundary Tests (`boundary_tests.rs`)

| # | Test Name | Failure Mode | Result |
|---|-----------|-------------|--------|
| 1 | `test_boundary_missing_perms_wf` | Omit `container_perms_wf` precondition | FAILED ✅ |
| 2 | `test_boundary_missing_tree_wf` | Omit `container_tree_wf` precondition | FAILED ✅ |
| 3 | `test_boundary_c_ptr_not_in_domain` | `c_ptr` not in `container_perms` domain | FAILED ✅ |
| 4 | `test_boundary_child_not_in_children` | `child_ptr` NOT in children (negated) | FAILED ✅ |
| 5 | `test_boundary_call_with_invalid_child` | Call function without child-in-children guarantee | FAILED ✅ (precondition not satisfied) |

**Analysis**: Every precondition is individually necessary. Removing any one prevents the postcondition from being derived.

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

| # | Test Name | Mutation | Result |
|---|-----------|----------|--------|
| 1 | `test_mutation_negated_postcondition` | Assert child NOT in subtree | FAILED ✅ |
| 2 | `test_mutation_wrong_container_subtree` | Assert child in arbitrary other container's subtree | FAILED ✅ |
| 3 | `test_mutation_wrong_depth` | Assert child has same depth as parent (not +1) | FAILED ✅ |
| 4 | `test_mutation_wrong_parent` | Assert child's parent is root instead of `c_ptr` | FAILED ✅ |
| 5 | `test_mutation_reverse_children` | Assert `c_ptr` is in `child_ptr`'s children (reverse) | FAILED ✅ |

**Analysis**: The specification correctly distinguishes correct from incorrect behavioral relations. Mutated depth, parent, and membership claims are all rejected.

---

## Logical Tests (`logical_tests.rs`)

| # | Test Name | Property Tested | Result |
|---|-----------|----------------|--------|
| 1 | `test_logical_reverse_subtree` | Subtree symmetry: parent in child's subtree | FAILED ✅ |
| 2 | `test_logical_child_is_root` | Child could be root container | FAILED ✅ |
| 3 | `test_logical_stronger_depth_inequality` | Depth difference is 2 instead of 1 | FAILED ✅ |
| 4 | `test_logical_self_in_subtree` | Container in its own subtree | FAILED ✅ |
| 5 | `test_logical_swapped_args` | Cross-function misuse with swapped c_ptr/child_ptr | FAILED ✅ (precondition not satisfied) |

**Analysis**: The specification does not admit unintended logical inferences. It correctly rejects reverse subtree claims, self-containment, strengthened inequalities, and cross-function misuse.

---

## Conclusion

The specification for `in_child_imply_in_subtree` is **consistent** across all three tested dimensions:
- **Boundary**: All four preconditions are individually necessary.
- **Behavioral**: Incorrect output mutations are all rejected.
- **Logical**: No unintended properties (symmetry, self-containment, stronger bounds) are entailed.

No specification weaknesses were detected.
