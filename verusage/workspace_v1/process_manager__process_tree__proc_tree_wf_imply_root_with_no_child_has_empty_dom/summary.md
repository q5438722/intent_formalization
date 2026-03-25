# Adversarial Proof Test Summary

**Target**: `proc_tree_wf_imply_root_with_no_child_has_empty_dom`
**Lemma**: If a well-formed process tree's root has no children, the tree domain is exactly `{root_proc}`.

---

## Results: 11/11 tests correctly FAILED verification ✅

All adversarial tests were rejected by the specification, indicating the spec is **consistent** with respect to the tested properties.

---

### Boundary Tests (4/4 FAILED) — `boundary_tests.rs`

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | Missing `proc_tree_dom_subset_of_proc_dom` | Precondition not satisfied | ✅ FAIL |
| 2 | Missing `proc_perms_wf` | Precondition not satisfied | ✅ FAIL |
| 3 | Missing `proc_tree_wf` | Precondition not satisfied | ✅ FAIL |
| 4 | Non-empty children (`len > 0`) | Precondition not satisfied | ✅ FAIL |

**Conclusion**: All four preconditions are independently necessary. Removing any one prevents the lemma from being invoked.

---

### Behavioral Mutation Tests (4/4 FAILED) — `behavioral_tests.rs`

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | Assert domain is empty (`Set::empty()`) | Assertion failed | ✅ FAIL |
| 2 | Assert root not in domain | Assertion failed | ✅ FAIL |
| 3 | Assert domain contains extra element (`other_ptr != root_proc`) | Assertion failed | ✅ FAIL |
| 4 | Assert subtree set is non-empty after `no_child_imply_no_subtree` | Assertion failed | ✅ FAIL |

**Conclusion**: The postconditions are precise. The spec correctly rejects all four mutated output assertions, confirming the ensures clauses tightly constrain the result.

---

### Logical Tests (3/3 FAILED) — `logical_tests.rs`

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `root_proc < 100` (structural assumption on pointer value) | Assertion failed | ✅ FAIL |
| 2 | Conclude `dom == {root_proc}` without empty-children precondition | Postcondition not satisfied | ✅ FAIL |
| 3 | `root.pcid == 0` (unconstrained field) | Assertion failed | ✅ FAIL |

**Conclusion**: The specification does not entail unintended properties. Pointer values, unconstrained fields (pcid), and the empty-children precondition cannot be reasoned about beyond what the spec explicitly states.

---

## Overall Assessment

The specification for `proc_tree_wf_imply_root_with_no_child_has_empty_dom` is **consistent**:

- **Boundary completeness**: All preconditions are enforced and independently necessary.
- **Behavioral precision**: The postcondition `proc_tree_dom =~= set!(root_proc)` is tight — neither weaker (empty set) nor stronger (extra elements) alternatives are admitted.
- **Logical soundness**: The spec does not leak unintended entailments about unconstrained fields or structural assumptions.

No specification weaknesses were identified through these 11 adversarial tests.
