# Adversarial Test Summary: `remove_proc_preserve_tree_inv_7`

## Target
Proof lemma that removing a leaf process from a process tree preserves the `procs_linkedlist_wf` invariant.

## Results: All 9 tests FAILED verification as expected ✅

### Boundary Tests (3/3 failed) — `boundary_tests.rs`
| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_proc_not_in_tree` | `!proc_tree_dom.contains(proc_ptr)` | ❌ precondition not satisfied |
| `test_boundary_remove_root_depth` | `depth == 0` (root-level node) | ❌ precondition not satisfied |
| `test_boundary_proc_has_children` | `children@.len() > 0` (non-leaf) | ❌ precondition not satisfied |

**Conclusion**: The `remove_proc_ensures` precondition correctly rejects invalid inputs — proc not in tree, root-depth node, and non-leaf node.

### Behavioral Mutation Tests (3/3 failed) — `behavioral_mutation_tests.rs`
| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_wrong_domain` | Postcondition on `proc_tree_dom` instead of `proc_tree_dom.remove(proc_ptr)` | ❌ postcondition not satisfied |
| `test_mutation_proc_still_exists` | Assert `proc_ptr` still in `new_proc_perms.dom()` | ❌ postcondition not satisfied |
| `test_mutation_wrong_root` | Use `proc_ptr` as root in postcondition | ❌ postcondition not satisfied |

**Conclusion**: The spec correctly rejects mutated behaviors — wrong domain, claiming the removed proc still exists, and swapping the root identity.

### Logical Tests (3/3 failed) — `logical_tests.rs`
| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_full_tree_wf` | Full `proc_tree_wf` preserved (not just `procs_linkedlist_wf`) | ❌ postcondition not satisfied |
| `test_logical_uniform_depth` | All remaining nodes have equal depth | ❌ postcondition not satisfied |
| `test_logical_children_parent_wf` | `proc_childern_parent_wf` preserved | ❌ postcondition not satisfied |

**Conclusion**: The lemma correctly limits its guarantee to `procs_linkedlist_wf` only. Stronger properties (full tree WF, depth uniformity, parent-child WF) are not entailed.

### Notable Finding During Development
- `procs_linkedlist_wf(root_proc, proc_tree_dom.remove(proc_ptr), old_proc_perms)` (using old perms instead of new on the reduced domain) is **correctly entailed** by the spec — this is valid because the removed node is a childless leaf, so the linked-list invariant holds on the reduced domain even with old permissions.
- `proc_subtree_set_wf` preservation is also entailed when tested within the same module (closed specs are transparent within the defining module), indicating the preconditions are strong enough for subtree set reasoning.

### Overall Assessment
The specification is **well-constrained**: all 9 adversarial tests were correctly rejected. The preconditions enforce necessary validity conditions, incorrect behavioral mutations are rejected, and the postcondition does not over-promise beyond `procs_linkedlist_wf`.
