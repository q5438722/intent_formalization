# Adversarial Test Summary

**Target**: `remove_container_preserve_tree_inv` — proves that removing a leaf container from a container tree preserves the tree well-formedness invariant (`container_tree_wf`).

## Results Overview

| Test File | Tests | All Failed (as expected) |
|-----------|-------|------------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (5 errors) |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5 errors) |
| `logical_tests.rs` | 5 | ✅ Yes (5 errors) |

**All 15 adversarial tests were correctly rejected by the specification.**

---

## Boundary Tests (Precondition Violations)

| # | Test | Violation | Result |
|---|------|-----------|--------|
| 1 | `test_boundary_remove_root` | `container_ptr == root_container` | ✅ FAIL (precondition) |
| 2 | `test_boundary_not_in_domain` | `!old.dom().contains(ptr)` | ✅ FAIL (precondition) |
| 3 | `test_boundary_has_children` | `children@.len() > 0` (non-leaf) | ✅ FAIL (precondition) |
| 4 | `test_boundary_depth_not_preserved` | Surviving container depth changed | ✅ FAIL (precondition) |
| 5 | `test_boundary_wrong_domain` | `new.dom() == old.dom()` (no removal) | ✅ FAIL (precondition) |

**Note**: An earlier test for `parent.is_None()` was found to be **redundant** — the property `parent.is_Some()` for non-root containers is already entailed by `container_tree_wf` (via `container_root_wf`). This is a minor spec redundancy finding.

## Behavioral Mutation Tests (Wrong Output Relations)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_still_in_domain` | Removed container still in new domain | ✅ FAIL (assertion) |
| 2 | `test_mutation_domain_unchanged` | New domain equals old domain | ✅ FAIL (assertion) |
| 3 | `test_mutation_wf_negated` | `!container_perms_wf(new)` | ✅ FAIL (assertion) |
| 4 | `test_mutation_unrelated_children_changed` | Unrelated container's children changed | ✅ FAIL (assertion) |
| 5 | `test_mutation_root_depth_nonzero` | Root depth became non-zero | ✅ FAIL (assertion) |

## Logical Tests (Unintended Reasoning)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_logical_determinism` | Two valid results must be equal | ✅ FAIL (assertion) |
| 2 | `test_logical_owned_procs_preserved` | `owned_procs` preserved after removal | ✅ FAIL (assertion) |
| 3 | `test_logical_scheduler_preserved` | `scheduler` preserved after removal | ✅ FAIL (assertion) |
| 4 | `test_logical_owned_cpus_preserved` | `owned_cpus` preserved after removal | ✅ FAIL (assertion) |
| 5 | `test_logical_tree_wf_arbitrary_map` | `tree_wf` on arbitrary map | ✅ FAIL (assertion) |

---

## Findings

### Spec Strengths
- **Boundary control**: The spec correctly rejects all tested invalid inputs (root removal, out-of-domain, non-leaf, wrong domain update, depth changes).
- **Behavioral correctness**: The spec correctly rejects all tested incorrect output mutations (domain, children, depth, well-formedness).
- **Logical tightness**: The spec does not admit unintended reasoning (determinism, frame conditions on non-tree fields, arbitrary map well-formedness).

### Spec Observations
1. **Redundant precondition**: `old[container_ptr].value().parent.is_Some()` is already implied by `container_tree_wf(root, old) ∧ old.dom().contains(ptr) ∧ ptr ≠ root` via `container_root_wf`. This is a minor redundancy, not a correctness issue.

2. **Non-determinism by design**: The spec intentionally does not constrain non-tree fields (`owned_procs`, `scheduler`, `quota`, `owned_cpus`, `owned_endpoints`, `owned_threads`, `root_process`, `can_have_children`) during removal. This is correct for a tree-structural lemma. A stronger frame condition (preserving these fields) would be needed if callers depend on their stability.

3. **Closed spec opacity**: Despite `closed spec fn` declarations, Verus's SMT solver can still reason about their contents within the same crate, providing strong internal consistency checking.
