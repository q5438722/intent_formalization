# Consistency Test Summary: `container_subtree_disjoint_inv`

## Target Function
`ProcessManager::container_subtree_disjoint_inv` — proves that for any two distinct containers at the same depth, their subtree sets are disjoint (and neither contains the other).

**Requires:** `self.wf()`  
**Ensures:** For all `c_ptr_i ≠ c_ptr_j` in the container domain with equal depth:
- `subtree_set(i)` and `subtree_set(j)` are disjoint
- Neither subtree set contains the other container pointer
- `subtree_set(i) ∪ {i}` and `subtree_set(j) ∪ {j}` are also disjoint

---

## Results Overview

| Test File | Tests | All Failed (as expected) |
|-----------|-------|------------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (5 errors, 23 verified) |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5 errors, 23 verified) |
| `logical_tests.rs` | 5 | ✅ Yes (5 errors, 23 verified) |

**Total: 15/15 tests correctly rejected by Verus.**

---

## Boundary Tests (precondition violations)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_no_wf` | Call without `self.wf()` | ✅ FAILED — precondition `self.wf()` not satisfied |
| 2 | `test_boundary_no_container_perms_wf` | Missing `container_perms_wf` on standalone fn | ✅ FAILED — precondition `container_perms_wf` not satisfied |
| 3 | `test_boundary_no_tree_wf` | Missing `container_tree_wf` on standalone fn | ✅ FAILED — precondition `container_tree_wf` not satisfied |
| 4 | `test_boundary_container_not_in_domain` | `c_ptr_i` not proven in domain | ✅ FAILED — assertion failed (cannot apply ensures) |
| 5 | `test_boundary_same_container` | `c_ptr_i == c_ptr_j` (not distinct) | ✅ FAILED — assertion failed (ensures requires distinctness) |

## Behavioral Mutation Tests (output mutations)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_mutation_subtrees_not_disjoint` | Assert NOT disjoint (negation of ensures) | ✅ FAILED — contradicts proven disjointness |
| 2 | `test_mutation_subtree_contains_other` | Assert `subtree_set(i)` contains `c_ptr_j` | ✅ FAILED — contradicts proven non-containment |
| 3 | `test_mutation_inserted_subtrees_not_disjoint` | Assert inserted sets NOT disjoint | ✅ FAILED — contradicts proven inserted disjointness |
| 4 | `test_mutation_reverse_subtree_contains` | Assert `subtree_set(j)` contains `c_ptr_i` | ✅ FAILED — contradicts proven non-containment |
| 5 | `test_mutation_shared_subtree_element` | Assert shared element in both subtree sets | ✅ FAILED — contradicts proven disjointness |

## Logical Tests (unguaranteed properties)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_logical_different_depth_disjoint` | Disjointness at *different* depths | ✅ FAILED — spec only covers same-depth |
| 2 | `test_logical_subtree_set_empty` | All subtree sets are empty | ✅ FAILED — not guaranteed by spec |
| 3 | `test_logical_all_depth_zero` | All containers at depth 0 | ✅ FAILED — depth is unconstrained |
| 4 | `test_logical_subtree_contains_self` | Container is in its own subtree set | ✅ FAILED — self-membership not guaranteed |
| 5 | `test_logical_subtree_disjoint_from_domain` | Subtree disjoint from full container domain | ✅ FAILED — overly strong, not entailed |

---

## Conclusion

The specification for `container_subtree_disjoint_inv` is **well-bounded**:
- **Preconditions are enforced**: all 3 boundary tests that violate `requires` were rejected.
- **Outputs are precise**: all 5 behavioral mutations (negating or inverting ensures clauses) were rejected.
- **No unintended entailment detected**: all 5 logical tests asserting unguaranteed properties were rejected.

The specification correctly constrains its semantic boundary — it entails exactly the stated disjointness property for same-depth distinct containers and nothing beyond.
