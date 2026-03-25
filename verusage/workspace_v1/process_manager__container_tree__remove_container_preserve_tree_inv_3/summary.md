# Adversarial Test Summary: `remove_container_preserve_tree_inv_3`

**Target**: Proves that `containers_linkedlist_wf` is preserved after removing a leaf container from the container tree.

## Results: All 9 tests FAILED verification ✅ (as expected)

The specification correctly rejects all invalid inputs, incorrect behaviors, and unintended reasoning.

---

### Boundary Tests (`boundary_tests.rs`) — 3/3 FAILED ✅

| Test | Failure Mode | Result |
|------|-------------|--------|
| `boundary_test_no_precondition` | Call lemma with no preconditions established | FAIL (precondition not satisfied) |
| `boundary_test_container_is_root` | Pass `root_container` as the container to remove | FAIL (precondition not satisfied) |
| `boundary_test_not_in_domain` | Container not in permissions domain | FAIL (precondition not satisfied) |

**Conclusion**: Preconditions are tight — invalid inputs are properly rejected.

---

### Behavioral Mutation Tests (`behavioral_tests.rs`) — 3/3 FAILED ✅

| Test | Mutation | Result |
|------|----------|--------|
| `behavioral_test_container_still_in_domain` | Assert removed container is still in new domain | FAIL (postcondition not satisfied) |
| `behavioral_test_parent_still_has_child` | Assert parent still lists removed container as child | FAIL (postcondition not satisfied) |
| `behavioral_test_root_not_in_domain` | Assert root was removed from domain | FAIL (postcondition not satisfied) |

**Conclusion**: The spec correctly constrains domain and children relationships after removal.

---

### Logical Tests (`logical_tests.rs`) — 3/3 FAILED ✅

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `logical_test_full_tree_wf` | Derive full `container_tree_wf` from this single lemma | FAIL (postcondition not satisfied) |
| `logical_test_owned_procs_preserved` | Derive `owned_procs` preservation (unspecified field) | FAIL (postcondition not satisfied) |
| `logical_test_ancestor_subtree_unchanged` | Assert ancestor's `subtree_set` is unchanged (should have container removed) | FAIL (postcondition not satisfied) |

**Conclusion**: The spec does not entail properties beyond what is explicitly stated. The lemma only proves `containers_linkedlist_wf` (not the full tree invariant), does not constrain unmentioned fields like `owned_procs`, and correctly requires `subtree_set` updates for ancestors.

---

### Overall Assessment

The specification for `remove_container_preserve_tree_inv_3` is **consistent**: it rejects all tested boundary violations, behavioral mutations, and logical over-approximations. No spec weaknesses were detected.
