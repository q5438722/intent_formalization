# Adversarial Proof Test Summary

**Target**: `remove_container_preserve_tree_inv_6` — proves that removing a leaf container from a container tree preserves `container_uppertree_seq_wf`.

## Results: 12/12 tests FAILED verification as expected ✅

### Boundary Tests (4/4 FAILED ✅)
| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_remove_root_container` | `container_ptr == root_container` | FAILED — precondition correctly rejects |
| `test_boundary_container_not_in_domain` | `container_ptr ∉ dom(old_perms)` | FAILED — precondition correctly rejects |
| `test_boundary_nonempty_children` | Container has children (non-empty) | FAILED — precondition correctly rejects |
| `test_boundary_new_perms_not_wf` | Missing `container_perms_wf(new_perms)` | FAILED — precondition correctly rejects |

### Behavioral Mutation Tests (4/4 FAILED ✅)
| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_removed_still_in_domain` | Assert removed container still in domain | FAILED — spec correctly excludes it |
| `test_mutation_full_tree_wf` | Assert full `container_tree_wf` preserved | FAILED — only `uppertree_seq_wf` guaranteed |
| `test_mutation_parent_children_same` | Assert parent's children unchanged | FAILED — spec correctly removes container |
| `test_mutation_domain_unchanged` | Assert domain unchanged after removal | FAILED — domain correctly shrinks |

### Logical Tests (4/4 FAILED ✅)
| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_uppertree_implies_root_wf` | `uppertree_seq_wf ⟹ root_wf` | FAILED — independent closed specs |
| `test_logical_quota_preserved` | Quota field preserved across removal | FAILED — spec doesn't constrain quota |
| `test_logical_can_have_children_preserved` | `can_have_children` preserved | FAILED — spec doesn't constrain this field |
| `test_logical_uppertree_implies_linkedlist_wf` | `uppertree_seq_wf ⟹ linkedlist_wf` | FAILED — independent closed specs |

## Notable Finding

During initial testing, two logical tests **passed unexpectedly**:
- `container_subtree_set_wf(root, new_perms)` was derivable from `remove_container_ensures` + `uppertree_seq_wf`
- `container_subtree_set_exclusive(root, new_perms)` was similarly derivable

This indicates the specification is **strong enough** to entail these tree invariants for the new permissions, which is desirable — the precondition's detailed preservation clauses combined with the proved `uppertree_seq_wf` are sufficient to reconstruct these properties.

## Spec Weakness Identified

The `remove_container_ensures` specification does **not constrain** non-tree-structural fields during removal:
- `quota`, `owned_procs`, `owned_endpoints`, `owned_threads`, `owned_cpus`, `scheduler`, `can_have_children`, `root_process`

These fields could change arbitrarily for surviving containers. If the intent is that removal only affects tree structure, this is a **spec incompleteness** — the specification allows unintended mutations of these fields.

## Conclusion

The specification correctly:
- **Rejects** invalid inputs (boundary tests)
- **Rejects** incorrect behavioral claims (mutation tests)
- **Rejects** unsupported logical inferences (logical tests)

One area of potential weakness: non-structural container fields are unconstrained by `remove_container_ensures`.
