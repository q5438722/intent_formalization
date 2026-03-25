# Adversarial Proof Test Summary

**Target**: `new_container_preserve_tree_inv_2` — proves that adding a new container to a container tree preserves `container_childern_parent_wf`.

## Results Overview

| Category | Tests | All Failed (as expected) |
|----------|-------|--------------------------|
| Boundary | 5 | ✅ Yes (5/5 errors) |
| Behavioral Mutation | 5 | ✅ Yes (5/5 errors) |
| Logical | 5 | ✅ Yes (5/5 errors) |

**Total: 15/15 tests correctly rejected by the verifier.**

---

## Boundary Tests (`boundary_tests.rs`)

All 5 tests violate preconditions of `new_container_ensures` and correctly fail with "precondition not satisfied":

| # | Test | Violated Precondition |
|---|------|-----------------------|
| 1 | `boundary_container_not_in_domain` | `container_ptr` not in old domain |
| 2 | `boundary_new_container_already_exists` | `new_container_ptr` already in domain |
| 3 | `boundary_depth_overflow` | `depth == usize::MAX` (requires `< usize::MAX`) |
| 4 | `boundary_children_full` | `children.len() >= PROC_CHILD_LIST_LEN` (requires `<`) |
| 5 | `boundary_perms_not_wf` | `container_perms_wf` negated |

**Conclusion**: The spec correctly rejects all tested invalid inputs.

---

## Behavioral Mutation Tests (`mutation_tests.rs`)

All 5 tests assert mutated (incorrect) output properties and correctly fail with "assertion failed":

| # | Test | Mutated Property |
|---|------|------------------|
| 1 | `mutation_wrong_depth` | depth == parent_depth (should be +1) |
| 2 | `mutation_no_parent` | parent is None (should be Some) |
| 3 | `mutation_children_unchanged` | children count same (should be +1) |
| 4 | `mutation_domain_unchanged` | domain same (should grow by 1) |
| 5 | `mutation_subtree_nonempty` | subtree non-empty (should be empty) |

**Conclusion**: The spec correctly rejects all tested incorrect behaviors.

---

## Logical Tests (`logical_tests.rs`)

All 5 tests assert properties NOT guaranteed by the specification and correctly fail:

| # | Test | Unentailed Property | Failure Reason |
|---|------|---------------------|----------------|
| 1 | `logical_full_tree_wf` | Full `container_tree_wf` for new perms | Only `container_childern_parent_wf` is proved |
| 2 | `logical_wrong_root` | `container_root_wf(new_container_ptr, ...)` | Cross-function misuse: new container has depth ≥ 1, not 0 |
| 3 | `logical_new_in_parent_uppertree` | new_container_ptr in parent's uppertree | Impossible: new_container_ptr was not in old domain |
| 4 | `logical_depth_greater_than_one` | New container depth > 1 | Not guaranteed: parent could be root (depth 0) |
| 5 | `logical_not_root` | container_ptr ≠ root_container | Not guaranteed: parent container could be root |

**Conclusion**: The spec does not allow unintended logical reasoning for all tested properties.

---

## Notable Finding

During development, an initial test (`logical_linkedlist_wf`) attempting to assert `containers_linkedlist_wf(root, new_container_perms)` **passed verification**. This reveals that the specification is strong enough to entail `containers_linkedlist_wf` even though the proof function only explicitly ensures `container_childern_parent_wf`. This is not a bug — the `new_container_ensures` precondition combined with the postcondition provides sufficient information for the verifier to derive this property. However, it indicates the ensures clause is weaker than what the spec actually guarantees.

---

## Overall Assessment

The specification for `new_container_preserve_tree_inv_2` is **well-constrained**:
- All precondition boundaries are enforced
- All behavioral mutations are rejected  
- No unintended logical inferences were found among tested properties
