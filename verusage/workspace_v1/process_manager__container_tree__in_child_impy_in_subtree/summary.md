# Test Execution Summary: `in_child_impy_in_subtree`

## Target
`process_manager__container_tree__in_child_impy_in_subtree.rs`

The lemma proves: if `child_ptr` is a child of `c_ptr` and `s_ptr` is in `child_ptr`'s subtree, then `s_ptr` is in `c_ptr`'s subtree.

---

## Results Overview

| File | Tests | Failed (expected) | Passed (unexpected) |
|------|-------|--------------------|---------------------|
| `boundary_tests.rs` | 5 | 5 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 5 | 5 ✅ | 0 |
| `logical_tests.rs` | 5 | 5 ✅ | 0 |
| **Total** | **15** | **15 ✅** | **0** |

All 15 adversarial tests were correctly **rejected** by Verus. No specification weakness detected.

---

## Boundary Tests (5/5 failed ✅)

| # | Test | Removed Precondition | Result |
|---|------|----------------------|--------|
| 1 | `test_boundary_missing_perms_wf` | `container_perms_wf` | FAIL ✅ |
| 2 | `test_boundary_missing_tree_wf` | `container_tree_wf` | FAIL ✅ |
| 3 | `test_boundary_c_ptr_not_in_domain` | `dom.contains(c_ptr)` | FAIL ✅ |
| 4 | `test_boundary_child_not_in_children` | `children@.contains(child_ptr)` | FAIL ✅ |
| 5 | `test_boundary_s_ptr_not_in_child_subtree` | `subtree_set@.contains(s_ptr)` | FAIL ✅ |

**Conclusion**: Every precondition is necessary. The specification does not admit the postcondition when any single precondition is dropped.

---

## Behavioral Mutation Tests (5/5 failed ✅)

| # | Test | Mutated Postcondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_mutation_negate_postcondition` | `!subtree_set@.contains(s_ptr)` | FAIL ✅ |
| 2 | `test_mutation_s_ptr_equals_c_ptr` | `s_ptr == c_ptr` | FAIL ✅ |
| 3 | `test_mutation_s_ptr_is_direct_child` | `children@.contains(s_ptr)` | FAIL ✅ |
| 4 | `test_mutation_wrong_depth` | `s_ptr.depth == c_ptr.depth + 1` | FAIL ✅ |
| 5 | `test_mutation_s_ptr_parent_is_child` | `s_ptr.parent == Some(child_ptr)` | FAIL ✅ |

**Conclusion**: The specification correctly rejects all mutated output relations. It does not over-constrain results (e.g., s_ptr is not forced to be a direct child or equal to c_ptr), nor does it allow contradictory conclusions.

---

## Logical Tests (5/5 failed ✅)

| # | Test | Unintended Property Tested | Result |
|---|------|----------------------------|--------|
| 1 | `test_logical_subtree_symmetric` | Subtree containment is symmetric | FAIL ✅ |
| 2 | `test_logical_self_in_subtree` | c_ptr ∈ c_ptr.subtree_set | FAIL ✅ |
| 3 | `test_logical_stronger_depth_bound` | s_ptr.depth == c_ptr.depth + 2 | FAIL ✅ |
| 4 | `test_logical_subtree_unique_ancestor` | Unique ancestor (c_ptr == d_ptr) | FAIL ✅ |
| 5 | `test_logical_subtree_subset_of_child` | c_ptr.subtree ⊆ child_ptr.subtree | FAIL ✅ |

**Conclusion**: The specification does not entail any of the unintended logical properties tested. Subtree is correctly asymmetric, non-reflexive, allows multiple ancestors, and does not force all descendants through a single child.

---

## Overall Assessment

The specification for `in_child_impy_in_subtree` is **consistent** with respect to all 15 adversarial queries:
- All preconditions are essential (no vacuous guards).
- Incorrect behavioral mutations are rejected.
- No unintended logical consequences are derivable.
