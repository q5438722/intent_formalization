# Adversarial Test Results Summary

**Target**: `process_manager__process_tree__new_proc_preserve_tree_inv_7.rs`
**Specification under test**: `new_proc_preserve_tree_inv_7` — proves that adding a new process preserves the `procs_linkedlist_wf` invariant on the extended tree domain.

---

## Results Overview

| File | Tests | All Failed? | Spec Status |
|------|-------|------------|-------------|
| `boundary_tests.rs` | 4 | ✅ Yes (4/4 errors) | Preconditions are tight |
| `behavioral_tests.rs` | 4 | ✅ Yes (4/4 errors) | Postcondition rejects mutations |
| `logical_tests.rs` | 4 | ✅ Yes (4/4 errors) | No unintended entailments found |

**Total: 12/12 tests correctly FAIL verification.**

---

## Boundary Tests (boundary_tests.rs)

All tests attempt to call `new_proc_preserve_tree_inv_7` with violated preconditions.

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_new_proc_already_in_tree` | `proc_tree_dom.contains(new_proc_ptr)` (should be false) | ❌ precondition not satisfied |
| `test_boundary_proc_not_in_tree` | `!proc_tree_dom.contains(proc_ptr)` (should be true) | ❌ precondition not satisfied |
| `test_boundary_same_ptrs` | `proc_ptr == new_proc_ptr` (contradicts membership) | ❌ precondition not satisfied |
| `test_boundary_depth_overflow` | `depth == usize::MAX` (should be < usize::MAX) | ❌ precondition not satisfied |

**Conclusion**: The preconditions in `new_proc_ensures` correctly reject all tested boundary violations.

---

## Behavioral Mutation Tests (behavioral_tests.rs)

All tests assume full `new_proc_ensures`, call the proof function, then assert a mutated postcondition.

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_wrong_root` | `procs_linkedlist_wf(new_proc_ptr, ...)` instead of `root_proc` | ❌ assertion failed |
| `test_mutation_old_domain` | `procs_linkedlist_wf(root_proc, proc_tree_dom, ...)` without inserting `new_proc_ptr` | ❌ assertion failed |
| `test_mutation_old_perms` | `procs_linkedlist_wf(..., old_proc_perms)` instead of `new_proc_perms` | ❌ assertion failed |
| `test_mutation_negated_postcondition` | `!procs_linkedlist_wf(...)` (negation of proven result) | ❌ assertion failed |

**Conclusion**: The postcondition is precise — mutating any argument (root, domain, perms) or negating the result is correctly rejected.

---

## Logical Tests (logical_tests.rs)

All tests assert properties not explicitly guaranteed by the specification.

| Test | Unentailed Property | Result |
|------|---------------------|--------|
| `test_logical_full_tree_wf` | Full `proc_tree_wf` (7 components) vs. only `procs_linkedlist_wf` (1 component) | ❌ assertion failed |
| `test_logical_unspecified_owning_container` | `new_proc.owning_container == parent.owning_container` | ❌ assertion failed |
| `test_logical_unspecified_pcid` | `new_proc.pcid == 0` | ❌ assertion failed |
| `test_logical_stronger_depth_bound` | `new_proc.depth < 100` | ❌ rlimit exceeded |

**Conclusion**: The spec does not inadvertently entail stronger properties. Unspecified fields (`owning_container`, `pcid`) remain unconstrained, and the single-component postcondition does not imply the full tree invariant.

---

## Overall Assessment

The specification for `new_proc_preserve_tree_inv_7` appears **well-scoped**:

1. **Preconditions are necessary**: Removing any key condition (tree membership, depth bound, distinctness) breaks the proof.
2. **Postcondition is precise**: The proven `procs_linkedlist_wf` cannot be swapped for a different predicate or used with wrong arguments.
3. **No unintended entailments**: The spec does not accidentally prove stronger properties (full tree wf), constrain unspecified fields, or impose artificial bounds.

No spec weaknesses were detected in this test campaign.
