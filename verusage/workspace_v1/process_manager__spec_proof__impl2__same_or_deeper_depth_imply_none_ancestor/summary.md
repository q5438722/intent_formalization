# Test Execution Summary

**Target**: `same_or_deeper_depth_imply_none_ancestor`
**Spec**: If `depth(ancestor) >= depth(child)`, then `child ∉ ancestor.subtree_set`.

## Results: All 12 tests FAILED verification ✅

The specification correctly rejects all adversarial queries.

---

### Boundary Tests (4/4 FAILED ✅)

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_no_wf` | Missing `wf()` — only `container_perms_wf()` assumed | FAILED |
| `test_boundary_ancestor_not_in_dom` | `ancestor_ptr ∉ container_dom` | FAILED |
| `test_boundary_reversed_depth` | `depth(ancestor) < depth(child)` | FAILED |
| `test_boundary_child_not_in_dom` | `child_ptr ∉ container_dom` | FAILED |

**Interpretation**: The spec correctly requires all four preconditions (`wf()`, both domain memberships, depth ordering). None are redundant.

---

### Behavioral Mutation Tests (4/4 FAILED ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_negated_postcondition` | Assert `child ∈ subtree` (opposite) | FAILED |
| `test_mutation_empty_subtree` | Assert `subtree = ∅` (too strong) | FAILED |
| `test_mutation_depth_equality` | Assert `depth(a) == depth(c)` (strengthen `>=` to `==`) | FAILED |
| `test_mutation_parent_relation` | Assert `child.parent == Some(ancestor)` (fabricated) | FAILED |

**Interpretation**: The spec precisely guarantees only the exclusion of `child_ptr` from the subtree — no stronger behavioral claims are entailed.

---

### Logical Tests (4/4 FAILED ✅)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_symmetry` | `ancestor ∉ child.subtree` (symmetry) | FAILED |
| `test_logical_distinct_containers` | `ancestor ≠ child` (distinctness) | FAILED |
| `test_logical_third_container_exclusion` | `x ∈ child.subtree ⟹ x ∉ ancestor.subtree` | FAILED |
| `test_logical_uppertree_membership` | `ancestor ∈ child.uppertree_seq` | FAILED |

**Interpretation**: The spec does not entail symmetry, pointer distinctness, subtree disjointness, or uppertree membership — all correct. These are structurally plausible but unwarranted inferences from the postcondition.

---

## Conclusion

The specification for `same_or_deeper_depth_imply_none_ancestor` is **tight**: it rejects invalid inputs (boundary), incorrect behaviors (mutation), and unintended logical inferences. No specification weakness was detected across the 12 adversarial tests.
