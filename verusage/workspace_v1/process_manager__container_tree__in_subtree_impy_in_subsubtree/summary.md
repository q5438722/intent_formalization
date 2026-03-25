# Adversarial Test Summary: `in_subtree_impy_in_subsubtree`

## Target Function
`in_subtree_impy_in_subsubtree` proves **subtree transitivity**: if `child_ptr ∈ subtree(c_ptr)` and `s_ptr ∈ subtree(child_ptr)`, then `s_ptr ∈ subtree(c_ptr)`.

---

## Results Overview

| Test Category         | Tests | All Failed? | Spec Weakness Found? |
|-----------------------|-------|-------------|----------------------|
| Boundary Tests        | 5     | ✅ Yes (5/5) | No                   |
| Behavioral Mutations  | 4     | ✅ Yes (4/4) | No                   |
| Logical Tests         | 4     | ✅ Yes (4/4) | No                   |
| **Total**             | **13**| **✅ 13/13** | **None**             |

---

## Boundary Tests (5/5 FAILED ✅)
All precondition violations were correctly rejected.

| # | Test | Violated Precondition | Result |
|---|------|-----------------------|--------|
| 1 | `boundary_test_missing_perms_wf` | Omitted `container_perms_wf` | ❌ precondition not satisfied |
| 2 | `boundary_test_missing_tree_wf` | Omitted `container_tree_wf` | ❌ precondition not satisfied |
| 3 | `boundary_test_c_domain_not_established` | Omitted `c_ptr ∈ dom` | ❌ precondition not satisfied |
| 4 | `boundary_test_child_not_in_subtree` | Negated `child_ptr ∈ subtree(c_ptr)` | ❌ precondition not satisfied |
| 5 | `boundary_test_s_not_in_child_subtree` | Negated `s_ptr ∈ subtree(child_ptr)` | ❌ precondition not satisfied |

## Behavioral Mutation Tests (4/4 FAILED ✅)
All mutated postconditions/relations were correctly rejected.

| # | Test | Mutated Property | Result |
|---|------|-----------------|--------|
| 1 | `mutation_negated_postcondition` | `s_ptr ∉ subtree(c_ptr)` | ❌ assertion failed |
| 2 | `mutation_reversed_containment` | `c_ptr ∈ subtree(s_ptr)` (reversed) | ❌ assertion failed |
| 3 | `mutation_wrong_subtree_direction` | `c_ptr ∈ subtree(child_ptr)` (reversed) | ❌ assertion failed |
| 4 | `mutation_identity_s_equals_c` | `s_ptr == c_ptr` | ❌ assertion failed |

## Logical Tests (4/4 FAILED ✅)
All unintended logical properties were correctly rejected.

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `logical_exact_depth_difference` | `depth(s_ptr) == depth(c_ptr) + 2` (exact, not just ≥) | ❌ assertion failed |
| 2 | `logical_c_must_be_root` | `c_ptr == root_container` (structural assumption) | ❌ assertion failed |
| 3 | `logical_child_is_direct_child` | `child_ptr ∈ children(c_ptr)` (direct child, not subtree) | ❌ assertion failed |
| 4 | `logical_universal_subtree` | `∀ d_ptr ∈ dom → d_ptr ∈ subtree(c_ptr)` (overgeneralization) | ❌ assertion failed |

---

## Conclusion

The specification for `in_subtree_impy_in_subsubtree` is **well-formed and tight**:
- **Preconditions** are each independently necessary — removing any one causes verification failure.
- **Postconditions** correctly characterize the output — mutated outputs are rejected.
- **Logical boundaries** are sound — the spec does not admit unintended properties like exact depth constraints, structural assumptions about the root, confusion between subtree membership and direct children, or universal subtree claims.

No specification weaknesses were detected across all 13 adversarial tests.
