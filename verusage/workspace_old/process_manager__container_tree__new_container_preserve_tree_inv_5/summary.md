# Summary: `new_container_preserve_tree_inv_5` Specification Tests

## File Under Test

`process_manager__container_tree__new_container_preserve_tree_inv_5.rs` defines a proof lemma that shows adding a new child container to an existing container tree preserves the `container_uppertree_seq_wf` invariant. This is one of several companion lemmas (inv_1 through inv_5+) that together prove the full `container_tree_wf` invariant is preserved.

**Main proof function:**
- `new_container_preserve_tree_inv_5`
  - **requires**: `new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr)` — describes the valid transition from old to new permission maps when inserting a child container
  - **ensures**: `container_uppertree_seq_wf(root_container, new_container_perms)` — the upper-tree sequence well-formedness invariant holds for the updated permissions

---

## Correctness Results

All tests use the full precondition (`new_container_ensures`) and verify properties that should hold.

**Verus output:** `17 verified, 0 errors` ✅

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_param_postcondition` | Call proof fn, assert `container_uppertree_seq_wf` postcondition | PASS | ✅ PASS |
| 2 | `test_old_perms_wf` | Assert `container_perms_wf` for both old and new perms | PASS | ✅ PASS |
| 3 | `test_old_tree_wf` | Assert `container_tree_wf` for old perms | PASS | ✅ PASS |
| 4 | `test_domain_membership` | Assert domain membership properties (container_ptr in, new not in old, new in new) | PASS | ✅ PASS |
| 5 | `test_new_container_properties` | Assert new container's parent, parent_rev_ptr, children, subtree_set | PASS | ✅ PASS |
| 6 | `test_container_ptr_updated` | Assert container_ptr's children updated, depth/parent/uppertree preserved | PASS | ✅ PASS |
| 7 | `test_other_containers_preserved` | Assert arbitrary other container's properties unchanged | PASS | ✅ PASS |
| 8 | `test_old_tree_sub_invariants` | Assert all 7 sub-invariants of `container_tree_wf` for old perms | PASS | ✅ PASS |
| 9 | `test_new_container_uppertree` | Assert new container's uppertree_seq = parent's uppertree_seq.push(parent) | PASS | ✅ PASS |
| 10 | `test_new_container_depth` | Assert new container's depth = parent's depth + 1 | PASS | ✅ PASS |

---

## Completeness Results

All tests attempt to assert something invalid — either violating preconditions or claiming unguaranteed postconditions.

### Round 1: Precondition Violations

**Verus output:** `7 verified, 4 errors` ✅

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_no_precondition` | Call proof fn with no requires clause at all | FAIL | ✅ FAIL |
| 2 | `test_only_perms_wf` | Only provide container_perms_wf, missing tree_wf and structural constraints | FAIL | ✅ FAIL |
| 3 | `test_new_already_in_domain` | new_container_ptr already in old domain (violates exclusion) | FAIL | ✅ FAIL |
| 4 | `test_container_ptr_not_in_domain` | container_ptr not in old domain (violates membership) | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions

**Verus output:** `7 verified, 4 errors` ✅

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_assert_full_tree_wf` | Assert full `container_tree_wf` for new perms (lemma only proves one component) | FAIL | ✅ FAIL |
| 2 | `test_assert_children_parent_wf_new` | Assert `container_childern_parent_wf` for new perms (not proven) | FAIL | ✅ FAIL |
| 3 | `test_assert_linkedlist_wf_new` | Assert `containers_linkedlist_wf` for new perms (not proven) | FAIL | ✅ FAIL |
| 4 | `test_assert_wf_for_arbitrary_perms` | Assert postcondition for arbitrary unrelated perms | FAIL | ✅ FAIL |

### Round 3: Negated/Contradicted Postconditions

**Verus output:** `7 verified, 4 errors` ✅

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_negate_uppertree_seq_wf` | Assert `!container_uppertree_seq_wf` (negate postcondition) | FAIL | ✅ FAIL |
| 2 | `test_negate_new_in_domain` | Assert new_container_ptr not in new domain (contradicts precondition) | FAIL | ✅ FAIL |
| 3 | `test_negate_parent` | Assert new container has no parent (contradicts precondition) | FAIL | ✅ FAIL |
| 4 | `test_negate_old_tree_wf` | Assert old tree is NOT well-formed (contradicts precondition) | FAIL | ✅ FAIL |

### Round 4: Wrong Specific Values

**Verus output:** `7 verified, 4 errors` ✅

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_parent_value` | Assert new container's parent is itself (should be container_ptr) | FAIL | ✅ FAIL |
| 2 | `test_wrong_depth_zero` | Assert new container's depth is 0 (should be parent_depth + 1 ≥ 1) | FAIL | ✅ FAIL |
| 3 | `test_wrong_children_empty` | Assert container_ptr's children are empty after adding (should contain new_container_ptr) | FAIL | ✅ FAIL |
| 4 | `test_wrong_domain_equal` | Assert old and new domains are equal (new has one extra element) | FAIL | ✅ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases

**Verus output:** `7 verified, 4 errors` ✅

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_postcondition_arbitrary_perms` | Assert postcondition for unrelated perms (not the ones from the lemma) | FAIL | ✅ FAIL |
| 2 | `test_unproven_children_parent_wf` | Assert `container_childern_parent_wf` for new perms (separate lemma needed) | FAIL | ✅ FAIL |
| 3 | `test_swapped_perms` | Call proof fn with old/new perms swapped (precondition fails) | FAIL | ✅ FAIL |
| 4 | `test_perms_maps_equal` | Assert old_perms =~= new_perms (domains differ) | FAIL | ✅ FAIL |

---

## Overall Assessment

### Correctness: ✅ PASS
All 10 correctness tests verify successfully. The specification correctly captures that `new_container_preserve_tree_inv_5` preserves the `container_uppertree_seq_wf` invariant when adding a new container.

### Completeness: ✅ PASS
All 20 completeness tests fail as expected. The specification is tight enough to reject:
- Calls without proper preconditions
- Claims stronger than the proven postcondition
- Contradictions of the postcondition
- Incorrect specific values
- Cross-function misuse

### Notable Observations
1. **Derivable sub-invariants**: Within the same module, `container_root_wf`, `container_childern_depth_wf`, `container_subtree_set_wf`, and `container_subtree_set_exclusive` can be derived for new_container_perms from the precondition alone (closed specs are visible within the same module). Only `container_childern_parent_wf` and `containers_linkedlist_wf` genuinely require separate lemmas.
2. **`container_uppertree_seq_wf` is root-independent**: The `root_container` parameter is not used in the body of `container_uppertree_seq_wf`, making it equivalent regardless of root value.
3. **No spec gaps found**: All tested boundaries behave correctly — the spec is neither too weak nor too strong.
