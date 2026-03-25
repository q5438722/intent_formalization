# Adversarial Proof Test Summary: `in_child_impy_in_subtree`

## Target Specification

The `in_child_impy_in_subtree` proof function establishes:
> If `child_ptr` is a child of `p_ptr` and `s_ptr` is in `child_ptr`'s subtree, then `s_ptr` is in `p_ptr`'s subtree.

This is a transitivity property over the process tree's subtree relation.

---

## Test Results Overview

| Category | Tests | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary | 5 | 5 | 0 |
| Behavioral Mutation | 3 | 3 | 0 |
| Logical | 4 | 4 | 0 |
| **Total** | **12** | **12** | **0** |

**Verdict: The specification is consistent.** All adversarial tests were correctly rejected.

---

## Boundary Tests (5/5 FAILED ✓)

All tests drop one precondition and attempt to prove the postcondition.

| Test | Dropped Precondition | Result |
|---|---|---|
| `test_boundary_missing_p_ptr_in_domain` | `proc_tree_dom.contains(p_ptr)` | FAILED ✓ |
| `test_boundary_missing_child_in_children` | `children@.contains(child_ptr)` | FAILED ✓ |
| `test_boundary_missing_s_in_subtree` | `subtree_set@.contains(s_ptr)` | FAILED ✓ |
| `test_boundary_missing_tree_wf` | `proc_tree_wf(...)` | FAILED ✓ |
| `test_boundary_missing_perms_wf` | `proc_perms_wf(...)` | FAILED ✓ |

**Interpretation:** Every precondition is necessary. The specification properly rejects invalid inputs — no single precondition is redundant for this proof.

---

## Behavioral Mutation Tests (3/3 FAILED ✓)

All tests use valid preconditions but assert mutated (wrong) postconditions.

| Test | Mutated Postcondition | Result |
|---|---|---|
| `test_mutation_negated_postcondition` | `!subtree_set@.contains(s_ptr)` (negation) | FAILED ✓ |
| `test_mutation_reverse_subtree` | `s_ptr.subtree_set.contains(p_ptr)` (reverse) | FAILED ✓ |
| `test_mutation_s_ptr_direct_child` | `p_ptr.children.contains(s_ptr)` (too strong) | FAILED ✓ |

**Interpretation:** The specification correctly rejects incorrect behaviors:
- The negation confirms the postcondition is actually entailed (not vacuously).
- The reverse test confirms subtree is not symmetric.
- The direct-child test confirms the spec distinguishes subtree membership from direct child membership.

---

## Logical Tests (4/4 FAILED ✓)

All tests use valid preconditions but assert properties not guaranteed by the specification.

| Test | Asserted Property | Result |
|---|---|---|
| `test_logical_s_equals_child` | `s_ptr == child_ptr` | FAILED ✓ |
| `test_logical_wrong_depth_relation` | `s_ptr.depth == p_ptr.depth + 1` | FAILED ✓ |
| `test_logical_s_parent_is_p` | `s_ptr.parent == p_ptr` | FAILED ✓ |
| `test_logical_p_is_root` | `p_ptr == root_proc` | FAILED ✓ |

**Interpretation:** The specification does not entail unintended logical consequences:
- `s_ptr` is not forced to equal `child_ptr` (it can be deeper).
- `s_ptr`'s depth is not pinned to `p_ptr.depth + 1` (it must be ≥ `p_ptr.depth + 2`).
- `s_ptr`'s parent is not forced to be `p_ptr` (it could be `child_ptr` or deeper).
- `p_ptr` is not forced to be the root (the lemma applies at any tree level).

---

## Conclusion

The specification for `in_child_impy_in_subtree` is **well-formed and consistent**:

1. **No redundant preconditions** — all 5 preconditions are individually necessary.
2. **No incorrect behaviors admitted** — wrong output relations are properly rejected.
3. **No unintended logical consequences** — the spec does not over-constrain the inputs beyond what is needed.
