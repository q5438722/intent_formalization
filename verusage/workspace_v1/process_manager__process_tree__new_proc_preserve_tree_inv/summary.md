# Adversarial Test Summary: `new_proc_preserve_tree_inv`

## Target
`process_manager__process_tree__new_proc_preserve_tree_inv.rs` — proves that adding a new process to a process tree preserves the tree well-formedness invariant (`proc_tree_wf`).

## Results Overview

| Category | Tests | All Failed (as expected) |
|---|---|---|
| Boundary | 5 | ✅ Yes (5 errors) |
| Behavioral Mutation | 5 | ✅ Yes (5 errors) |
| Logical | 5 | ✅ Yes (5 errors) |

**Total: 15/15 tests correctly rejected** — the specification is consistent across all tested dimensions.

---

## Boundary Tests (`boundary_tests.rs`)

Each test violates a specific precondition and attempts to assert the postcondition.

| Test | Violated Precondition | Result |
|---|---|---|
| B1 | `proc_ptr` not in tree domain | ✅ FAILED |
| B2 | `new_proc_ptr` already in tree domain | ✅ FAILED |
| B3 | Children list at max capacity (`== PROC_CHILD_LIST_LEN`) | ✅ FAILED |
| B4 | Parent depth at `usize::MAX` (overflow) | ✅ FAILED |
| B5 | `new_proc_perms` domain unchanged (missing `new_proc_ptr`) | ✅ FAILED |

**Conclusion**: The preconditions correctly reject all tested invalid inputs.

---

## Behavioral Mutation Tests (`behavioral_tests.rs`)

Each test assumes valid preconditions (`new_proc_ensures`) and asserts a mutated/incorrect output property.

| Test | Mutated Property | Result |
|---|---|---|
| M1 | New process parent is `None` (should be `Some(proc_ptr)`) | ✅ FAILED |
| M2 | New process depth is 0 (should be `parent_depth + 1`) | ✅ FAILED |
| M3 | Parent's children list unchanged (should have `new_proc_ptr` appended) | ✅ FAILED |
| M4 | New process is in its own subtree (subtree should be empty) | ✅ FAILED |
| M5 | New process's `uppertree_seq` is empty (should contain ancestors) | ✅ FAILED |

**Conclusion**: The specification correctly rejects all tested incorrect behaviors.

---

## Logical Tests (`logical_tests.rs`)

Each test asserts a property NOT explicitly guaranteed by the specification.

| Test | Unintended Property | Result |
|---|---|---|
| L1 | Tree well-formed using `new_proc_ptr` as root | ✅ FAILED |
| L2 | New process inherits parent's `owning_container` | ✅ FAILED |
| L3 | Old tree domain still well-formed with new perms | ✅ FAILED |
| L4 | New process depth bounded by constant (`< 10`) | ✅ FAILED |
| L5 | `proc_ptr` must equal `root_proc` | ✅ FAILED |

**Conclusion**: The specification does not entail any of the tested unintended properties.

---

## Overall Assessment

The specification for `new_proc_preserve_tree_inv` is **consistent** across all three testing dimensions:

1. **Boundary correctness**: Invalid inputs (violated preconditions) are properly rejected.
2. **Behavioral correctness**: Incorrect output relationships are properly rejected.
3. **Logical soundness**: Unintended reasoning (stronger properties, cross-function misuse, structural assumptions) is not admitted.

No specification weaknesses were detected in this round of adversarial testing.
