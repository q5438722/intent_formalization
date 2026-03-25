# Adversarial Test Summary: `no_child_imply_no_subtree`

**Target**: `process_manager__process_tree__no_child_imply_no_subtree.rs`  
**Lemma**: If a process has no children (`children@ =~= Seq::empty()`), then its subtree is empty (`subtree_set@ =~= Set::empty()`).

---

## Results: All 9 tests FAILED verification ✅

This confirms the specification correctly rejects invalid inputs, incorrect behaviors, and unintended logical inferences.

### Boundary Tests (`boundary_tests.rs`) — 3/3 failed ✅

| Test | Violated Precondition | Result |
|------|----------------------|--------|
| `boundary_test_missing_dom_membership` | `p_ptr` not in `proc_tree_dom` | **FAILED** — spec cannot derive subtree emptiness for out-of-domain nodes |
| `boundary_test_nonempty_children` | children sequence non-empty | **FAILED** — empty subtree not provable with children present |
| `boundary_test_missing_tree_wf` | `proc_tree_wf` omitted | **FAILED** — without tree well-formedness, no subtree properties hold |

### Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 3/3 failed ✅

| Test | Mutation | Result |
|------|----------|--------|
| `mutation_test_subtree_contains_element` | Subtree contains arbitrary `s_ptr` | **FAILED** — contradicts proven empty subtree |
| `mutation_test_subtree_contains_self` | Subtree contains `p_ptr` itself | **FAILED** — contradicts proven empty subtree |
| `mutation_test_subtree_contains_root` | Subtree contains `root_proc` | **FAILED** — contradicts proven empty subtree |

### Logical Tests (`logical_tests.rs`) — 3/3 failed ✅

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `logical_test_no_child_implies_root` | No children ⟹ `p_ptr == root_proc` | **FAILED** — non-root leaf nodes exist |
| `logical_test_no_child_implies_depth_zero` | No children ⟹ `depth == 0` | **FAILED** — leaves can exist at any depth |
| `logical_test_no_child_implies_no_parent` | No children ⟹ `parent.is_None()` | **FAILED** — only root lacks a parent |

---

## Conclusion

The specification for `no_child_imply_no_subtree` is **consistent** with respect to all tested adversarial queries:

- **Boundary completeness**: Each precondition is necessary; removing any one prevents verification.
- **Behavioral precision**: The postcondition is tight; mutated outputs are correctly rejected.
- **Logical soundness**: The spec does not entail unintended stronger properties (e.g., equating childless nodes with the root or forcing depth-zero).

No specification weaknesses were detected.
