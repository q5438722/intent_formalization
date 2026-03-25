# Adversarial Proof Test Summary

**Target**: `container_tree_check_is_ancestor` — checks if `a_ptr` is an ancestor of `child_ptr` in a container tree.

## Results: All 15 tests FAILED verification as expected ✅

The specification correctly rejects all unintended properties.

---

### Boundary Tests (5/5 failed ✓)

| Test | Violated Precondition | Failure Reason |
|------|----------------------|----------------|
| `test_boundary_equal_depth` | `depth ==` instead of `<` | Precondition `depth <` not satisfied |
| `test_boundary_reversed_depth` | `depth >` instead of `<` | Precondition `depth <` not satisfied |
| `test_boundary_a_not_in_domain` | `a_ptr` absent from domain | Precondition `dom.contains(a_ptr)` not satisfied |
| `test_boundary_child_not_in_domain` | `child_ptr` absent from domain | Precondition `dom.contains(child_ptr)` not satisfied |
| `test_boundary_no_tree_wf` | `container_tree_wf` omitted | Precondition `container_tree_wf` not satisfied |

**Conclusion**: All five preconditions are necessary. The spec correctly guards against invalid inputs.

---

### Behavioral Mutation Tests (5/5 failed ✓)

| Test | Mutated Property | Failure Reason |
|------|-----------------|----------------|
| `test_mutation_always_ancestor` | Assert a_ptr is always ancestor | Cannot derive — depth `<` does not imply ancestry |
| `test_mutation_never_ancestor` | Assert a_ptr is never ancestor | Cannot derive — some nodes ARE ancestors |
| `test_mutation_reverse_subtree` | child's subtree contains ancestor | Cannot derive — subtree is downward, not upward |
| `test_mutation_self_in_subtree` | Node in its own subtree | Cannot derive — no self-ancestry |
| `test_mutation_postconditions_disagree` | uppertree and subtree disagree | Cannot derive — they are equivalent by `container_subtree_set_exclusive` |

**Conclusion**: The postconditions correctly constrain the output. Neither always-true, always-false, reversed, nor self-referential mutations are admitted.

---

### Logical Tests (5/5 failed ✓)

| Test | Unintended Property | Failure Reason |
|------|-------------------|----------------|
| `test_logical_symmetry` | Ancestry is symmetric | Cannot derive — ancestry is directional |
| `test_logical_all_deeper_are_descendants` | All deeper nodes are descendants | Cannot derive — depth alone doesn't imply subtree membership |
| `test_logical_depth_gap_at_least_2` | Ancestry requires depth gap ≥ 2 | Cannot derive — parent-child has gap 1 |
| `test_logical_self_ancestry` | A node is its own ancestor | Cannot derive — uppertree_seq excludes self |
| `test_logical_subtree_implies_direct_child` | Subtree ⊆ direct children | Cannot derive — subtree includes all descendants |

**Conclusion**: The spec does not over-constrain or admit unintended logical inferences. Symmetry, universality, stronger inequalities, self-reference, and structural flattening are all correctly rejected.

---

## Overall Assessment

The specification for `container_tree_check_is_ancestor` is **well-formed**:
- **Preconditions** are all necessary (no unnecessary guards, no missing guards)
- **Postconditions** correctly characterize the return value (reject mutations)
- **Logical boundaries** are tight (no unintended entailments)

No spec weaknesses were detected across all 15 adversarial tests.
