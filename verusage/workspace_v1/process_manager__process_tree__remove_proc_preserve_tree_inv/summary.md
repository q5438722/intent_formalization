# Adversarial Test Summary: `remove_proc_preserve_tree_inv`

## Target Specification

The spec proves that removing a leaf process (no children, non-root) from a well-formed process tree preserves the tree invariant (`proc_tree_wf`). Key preconditions: tree is well-formed, process has no children, depth ≠ 0, and the new permissions map is properly updated.

## Results: All 15/15 tests FAILED verification ✅

The specification correctly rejects all adversarial queries.

---

### Boundary Tests (5/5 failed) — `boundary_tests.rs`

| Test | Violated Precondition | Result |
|---|---|---|
| `boundary_test_remove_root` | `depth == 0` instead of `depth != 0` | ❌ Failed (precondition not satisfied) |
| `boundary_test_has_children` | `children@.len() > 0` instead of `children@ == empty` | ❌ Failed (precondition not satisfied) |
| `boundary_test_not_in_domain` | `!proc_tree_dom.contains(proc_ptr)` | ❌ Failed (precondition not satisfied) |
| `boundary_test_wrong_new_domain` | `new_perms.dom() == old.dom()` (not removed) | ❌ Failed (precondition not satisfied) |
| `boundary_test_tree_not_wf` | `!proc_tree_wf(...)` | ❌ Failed (precondition not satisfied) |

**Conclusion**: All preconditions are necessary; none are redundant.

---

### Behavioral Mutation Tests (5/5 failed) — `behavioral_mutation_tests.rs`

| Test | Mutated Assertion | Result |
|---|---|---|
| `mutation_test_wrong_domain` | `proc_tree_wf` on original domain (not reduced) | ❌ Failed (assertion failed) |
| `mutation_test_proc_still_in_domain` | `new_proc_perms.dom().contains(proc_ptr)` | ❌ Failed (assertion failed) |
| `mutation_test_wrong_root` | `proc_tree_wf` with `proc_ptr` as root | ❌ Failed (assertion failed) |
| `mutation_test_empty_domain` | Reduced domain is empty | ❌ Failed (assertion failed) |
| `mutation_test_parent_becomes_leaf` | Parent's children become empty | ❌ Failed (assertion failed) |

**Conclusion**: The spec correctly distinguishes valid postconditions from mutated ones.

---

### Logical Tests (5/5 failed) — `logical_tests.rs`

| Test | Unwarranted Property | Result |
|---|---|---|
| `logical_test_pcid_not_preserved` | `pcid` field preserved across removal | ❌ Failed (assertion failed) |
| `logical_test_owning_container_not_preserved` | `owning_container` field preserved | ❌ Failed (assertion failed) |
| `logical_test_cross_invariant` | `proc_root_wf` alone implies `proc_childern_parent_wf` | ❌ Failed (assertion failed) |
| `logical_test_stronger_depth_bound` | `depth >= 2` (spec only requires `>= 1`) | ❌ Failed (assertion failed) |
| `logical_test_determinism` | Two valid results must be identical | ❌ Failed (assertion failed) |

**Conclusion**: The spec does not entail unintended properties — unrelated fields are unconstrained, sub-invariants are independent, and the operation is not spuriously deterministic.

---

## Overall Assessment

The specification for `remove_proc_preserve_tree_inv` is **consistent** across all three tested dimensions:
- **Boundary**: Invalid inputs are rejected (all 5 preconditions are non-redundant)
- **Behavioral**: Incorrect outputs are rejected (mutated postconditions fail)
- **Logical**: Unintended reasoning is rejected (unrelated fields, cross-invariant leakage, determinism, stronger bounds all fail)

No spec weaknesses were detected.
