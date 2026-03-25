# Adversarial Proof Test Summary

**Target**: `process_manager__container_tree__new_container_preserve_tree_inv_7.rs`
**Function under test**: `new_container_preserve_tree_inv_7`
**Spec**: Given `new_container_ensures(...)` → proves `containers_linkedlist_wf(root_container, new_container_perms)`

## Results: All 9 tests correctly FAILED ✅

The specification correctly rejects all adversarial queries — no spec weakness detected.

---

### Boundary Tests (3/3 failed ✅)

| Test | Description | Result |
|------|-------------|--------|
| `boundary_test_no_preconditions` | Assert `containers_linkedlist_wf` with zero assumptions | FAILED ✅ |
| `boundary_test_wrong_root` | Assert `containers_linkedlist_wf` with `new_container_ptr` as root instead of `root_container` | FAILED ✅ |
| `boundary_test_only_perms_wf` | Assert `containers_linkedlist_wf` assuming only `container_perms_wf` (no tree invariants) | FAILED ✅ |

**Conclusion**: The closed spec `containers_linkedlist_wf` is not trivially true, is root-specific, and requires more than basic well-formedness.

### Behavioral Mutation Tests (3/3 failed ✅)

| Test | Description | Result |
|------|-------------|--------|
| `mutation_test_parent_is_none` | Assert new container's parent is `None` (spec says `Some(container_ptr)`) | FAILED ✅ |
| `mutation_test_depth_zero` | Assert new container's depth is 0 (spec says `parent_depth + 1 ≥ 1`) | FAILED ✅ |
| `mutation_test_domain_unchanged` | Assert domain didn't grow (spec says `old_dom.insert(new_container_ptr)`) | FAILED ✅ |

**Conclusion**: The `new_container_ensures` spec correctly constrains parent, depth, and domain growth — incorrect output relations are rejected.

### Logical Tests (3/3 failed ✅)

| Test | Description | Result |
|------|-------------|--------|
| `logical_test_full_tree_wf` | Derive full `container_tree_wf` on new perms (function only proves `containers_linkedlist_wf`) | FAILED ✅ |
| `logical_test_unconstrained_field` | Assert new container's `scheduler` is empty (spec doesn't constrain this field) | FAILED ✅ |
| `logical_test_determinism` | Assert two perms maps satisfying the same ensures must be equal | FAILED ✅ |

**Conclusion**: The spec does not over-entail — it doesn't allow deriving the full tree invariant from a single component, doesn't constrain unconstrained fields, and doesn't imply determinism of the output.

### Notable Finding

During development, an initial test asserting `container_root_wf(root_container, new_container_perms)` **PASSED** verification. This is because `container_root_wf` (despite being `closed spec`) is visible within the same crate, and the spec genuinely preserves all root well-formedness properties (root stays in domain, root depth stays 0, new container has depth ≥ 1, all parents preserved). This is correct behavior, not a spec weakness — the spec is strong enough to preserve root well-formedness as an emergent property.
