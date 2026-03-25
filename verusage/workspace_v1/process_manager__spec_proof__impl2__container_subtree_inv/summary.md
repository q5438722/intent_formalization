# Test Execution Summary: `container_subtree_inv`

## Target Specification

**Function:** `container_subtree_inv`  
**Requires:** `container_perms_wf(container_perms)` ∧ `container_tree_wf(root_container, container_perms)`  
**Ensures:** ∀ `c_ptr` ∈ `container_dom`:
1. `subtree_set ⊆ container_dom`
2. `c_ptr ∉ subtree_set(c_ptr)` (no self-containment)

---

## Results: All 9 tests FAILED verification ✅

### Boundary Tests (3/3 failed) — `boundary_tests.rs`

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_no_preconditions` | Neither `container_perms_wf` nor `container_tree_wf` provided | ❌ FAIL (precondition not satisfied) |
| `test_boundary_missing_perms_wf` | Only `container_tree_wf` provided, missing `container_perms_wf` | ❌ FAIL (precondition not satisfied) |
| `test_boundary_missing_tree_wf` | Only `container_perms_wf` provided, missing `container_tree_wf` | ❌ FAIL (precondition not satisfied) |

**Conclusion:** The specification correctly rejects calls with incomplete preconditions. Both preconditions are independently necessary.

### Behavioral Mutation Tests (3/3 failed) — `mutation_tests.rs`

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_self_in_subtree` | Assert `c_ptr ∈ subtree_set(c_ptr)` (negate postcondition 2) | ❌ FAIL (assertion failed) |
| `test_mutation_not_subset` | Assert `subtree_set ⊄ container_dom` (negate postcondition 1) | ❌ FAIL (assertion failed) |
| `test_mutation_element_outside_domain` | Assert foreign pointer in subtree_set | ❌ FAIL (assertion failed) |

**Conclusion:** The specification correctly rejects all three behavioral mutations. The ensures clauses are strong enough to prevent inversions of the guaranteed properties.

### Logical Tests (3/3 failed) — `logical_tests.rs`

| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_subtree_always_empty` | `subtree_set = ∅` for all containers | ❌ FAIL (assertion failed) |
| `test_logical_subtree_symmetry` | `b ∈ subtree(a) ⟹ a ∈ subtree(b)` | ❌ FAIL (assertion failed) |
| `test_logical_subtree_equal` | `subtree(a) = subtree(b)` for distinct a, b | ❌ FAIL (assertion failed) |

**Conclusion:** The specification does NOT entail these unintended logical properties. The spec correctly refuses to prove: universal emptiness, symmetry of the subtree relation, and equality of subtree sets.

---

## Overall Assessment

The `container_subtree_inv` specification is **consistent** with respect to all tested queries:
- **Boundary integrity:** Both preconditions are independently enforced.
- **Behavioral correctness:** Mutated postconditions are properly rejected.
- **Logical soundness:** Unintended structural properties (emptiness, symmetry, equality) are not derivable.

No specification weaknesses were identified in this test suite.
