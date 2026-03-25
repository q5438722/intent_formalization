# Adversarial Test Summary: `new_proc_preserve_tree_inv_5`

## Target
`process_manager__process_tree__new_proc_preserve_tree_inv_5.rs`

The proof function `new_proc_preserve_tree_inv_5` proves that adding a new process to a process tree preserves the `proc_uppertree_seq_wf` invariant.

---

## Results Overview

| Test File | Tests | All Failed (as expected) |
|-----------|-------|--------------------------|
| `boundary_tests.rs` | 3 | ✅ Yes (3 errors) |
| `behavioral_mutation_tests.rs` | 3 | ✅ Yes (3 errors) |
| `logical_tests.rs` | 3 | ✅ Yes (3 errors) |

**Total: 9/9 adversarial tests correctly rejected by the specification.**

---

## Boundary Tests (3/3 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_no_preconditions` | Call proof function with zero preconditions | ✅ Rejected (precondition not satisfied) |
| `test_boundary_swapped_pointers` | Swap `proc_ptr` and `new_proc_ptr` arguments | ✅ Rejected (precondition not satisfied) |
| `test_boundary_wrong_root` | Use `new_proc_ptr` as root instead of `root_proc` | ✅ Rejected (precondition not satisfied) |

**Conclusion**: The specification correctly rejects all boundary violations. The `new_proc_ensures` precondition cannot be satisfied with invalid/swapped inputs.

---

## Behavioral Mutation Tests (3/3 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_mutation_nonempty_children` | Assert new process has non-empty children (should be empty) | ✅ Rejected (assertion failed) |
| `test_mutation_old_perms` | Assert invariant with `old_proc_perms` instead of `new_proc_perms` | ✅ Rejected (assertion failed) |
| `test_mutation_wrong_depth` | Assert new depth == parent depth (missing +1) | ✅ Rejected (assertion failed) |

**Conclusion**: The specification correctly distinguishes correct outputs from mutated ones. Depth relationships, children lists, and permission maps are all precisely constrained.

---

## Logical Tests (3/3 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_logical_full_tree_wf` | Assert full `proc_tree_wf` (7 sub-invariants) instead of just `proc_uppertree_seq_wf` | ✅ Rejected (assertion failed) |
| `test_logical_container_inheritance` | Assert `owning_container` inherited from parent (not in spec) | ✅ Rejected (assertion failed) |
| `test_logical_children_parent_wf` | Assert `proc_childern_parent_wf` (different invariant component) | ✅ Rejected (assertion failed) |

**Conclusion**: The specification does not allow unintended logical inferences about stronger invariants or unspecified properties.

---

## Notable Finding (from initial iteration)

During initial testing, two tests **unexpectedly passed**:

1. **`proc_uppertree_seq_wf` on old domain with new perms**: The invariant holds on `proc_tree_dom` (without `new_proc_ptr`) using `new_proc_perms`. This is a legitimate consequence — existing nodes' `uppertree_seq` values are preserved, and subtree sets only grow by adding `new_proc_ptr`, so containment relationships for old nodes remain valid.

2. **`proc_subtree_set_exclusive` on new domain**: This invariant component (not part of the ensures) is derivable from the preconditions alone. The preconditions in `new_proc_ensures` are rich enough to establish this cross-invariant property without the proof function explicitly claiming it.

These findings show the preconditions (`new_proc_ensures`) are sufficiently strong — possibly stronger than minimally necessary — but do not constitute specification weaknesses in a correctness sense.

---

## Overall Assessment

The specification for `new_proc_preserve_tree_inv_5` is **well-constrained**:
- Invalid inputs are properly rejected by preconditions
- Behavioral mutations (wrong depth, wrong perms, wrong children) are detected
- Unintended logical inferences (stronger invariants, unspecified properties) are blocked
- The specification neither admits unintended behaviors nor allows unsound reasoning
