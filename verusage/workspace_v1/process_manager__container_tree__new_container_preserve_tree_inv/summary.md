# Adversarial Proof Test Results Summary

**Target**: `process_manager__container_tree__new_container_preserve_tree_inv.rs`  
**Spec under test**: `new_container_preserve_tree_inv` — proves that adding a new child container to an existing parent preserves the `container_tree_wf` invariant.

---

## Results Overview

| Test Category         | Total | Failed (expected) | Passed (spec weakness) |
|-----------------------|-------|--------------------|------------------------|
| Boundary Tests        | 5     | 5 ✅               | 0                      |
| Behavioral Mutations  | 5     | 5 ✅               | 0                      |
| Logical Tests         | 5     | 5 ✅               | 0                      |

**All 15 tests FAILED verification as expected.** No spec weaknesses were detected.

---

## (1) Boundary Tests — `boundary_tests.rs`

Tests that violate preconditions of the specification:

| Test | Violation | Result |
|------|-----------|--------|
| `boundary_test_container_ptr_not_in_domain` | `container_ptr` NOT in old domain | ❌ FAILED ✅ |
| `boundary_test_new_container_already_in_domain` | `new_container_ptr` already in domain | ❌ FAILED ✅ |
| `boundary_test_children_at_capacity` | Parent children ≥ `PROC_CHILD_LIST_LEN` | ❌ FAILED ✅ |
| `boundary_test_depth_at_max` | Parent depth == `usize::MAX` | ❌ FAILED ✅ |
| `boundary_test_no_tree_wf` | Old tree `container_tree_wf` not assumed | ❌ FAILED ✅ |

**Conclusion**: The spec correctly guards all preconditions. Invalid inputs cannot produce valid tree invariants.

---

## (2) Behavioral Mutation Tests — `behavioral_mutation_tests.rs`

Tests that assume valid inputs but assert incorrect output properties:

| Test | Mutated Property | Result |
|------|------------------|--------|
| `mutation_test_wrong_depth` | New depth == parent depth (should be +1) | ❌ FAILED ✅ |
| `mutation_test_wrong_parent` | New container parent is None (should be Some) | ❌ FAILED ✅ |
| `mutation_test_nonempty_children` | New container has children (should be empty) | ❌ FAILED ✅ |
| `mutation_test_domain_unchanged` | Domain unchanged (should grow by 1) | ❌ FAILED ✅ |
| `mutation_test_parent_children_unchanged` | Parent children length unchanged (should +1) | ❌ FAILED ✅ |

**Conclusion**: The spec correctly constrains all core structural outputs. Mutated behaviors are properly rejected.

---

## (3) Logical Tests — `logical_tests.rs`

Tests for properties NOT explicitly guaranteed by the specification:

| Test | Unguaranteed Property | Result |
|------|----------------------|--------|
| `logical_test_root_process_none` | New container's `root_process` is None | ❌ FAILED ✅ |
| `logical_test_can_have_children_false` | New container's `can_have_children` is false | ❌ FAILED ✅ |
| `logical_test_owned_endpoints_empty` | New container's `owned_endpoints` is empty | ❌ FAILED ✅ |
| `logical_test_parent_quota_preserved` | Parent's `quota.mem_4k` is preserved | ❌ FAILED ✅ |
| `logical_test_sibling_owned_threads_preserved` | Sibling's `owned_threads` is preserved | ❌ FAILED ✅ |

**Conclusion**: The spec does NOT over-promise. It correctly avoids entailing properties about:
- Unspecified fields of the new container (`root_process`, `can_have_children`, `owned_endpoints`)
- Non-structural fields of the parent (`quota`)
- Non-structural fields of sibling containers (`owned_threads`)

---

## Overall Assessment

The specification for `new_container_preserve_tree_inv` is **well-bounded**:

1. **Precondition coverage**: All boundary conditions (domain membership, capacity, depth overflow, tree well-formedness) are properly guarded.
2. **Behavioral precision**: The spec tightly constrains structural outputs (depth, parent, children, domain changes) and rejects all tested mutations.
3. **Logical restraint**: The spec does not over-specify — fields not relevant to tree structure (`root_process`, `quota`, `owned_threads`, `owned_endpoints`, `can_have_children`) are correctly left unconstrained.

**No spec weaknesses (inconsistencies) were found.** The specification rejects all 15 adversarial queries.
