# Test Summary: `new_container_preserve_tree_inv_4`

## File Under Test

**File**: `process_manager__container_tree__new_container_preserve_tree_inv_4.rs`

**Main proof function**: `new_container_preserve_tree_inv_4`
- **Requires**: `new_container_ensures(root_container, old_container_perms, new_container_perms, container_ptr, new_container_ptr)` — an open spec describing the state transition when adding a new container to a container tree.
- **Ensures**: `container_subtree_set_wf(root_container, new_container_perms)` — a closed spec asserting the subtree-set well-formedness invariant is preserved after adding the new container.

The lemma proves that inserting a new container (with proper parent linkage, empty children/subtree, correct depth/uppertree) preserves the subtree-set well-formedness invariant.

---

## Correctness Results (should all PASS)

**Verification result: 21 verified, 0 errors**

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_param_basic_postcondition` | Call lemma, assert `container_subtree_set_wf(root, new_perms)` | PASS | ✅ PASS |
| 2 | `test_old_tree_wf` | Assert old tree was well-formed (from open `new_container_ensures`) | PASS | ✅ PASS |
| 3 | `test_old_subtree_set_wf` | Assert old `container_subtree_set_wf` (component of `container_tree_wf`) | PASS | ✅ PASS |
| 4 | `test_perms_wf` | Assert both old and new `container_perms_wf` hold | PASS | ✅ PASS |
| 5 | `test_domain_relationship` | Assert domain: `new == old.insert(new_ptr)`, membership properties | PASS | ✅ PASS |
| 6 | `test_new_container_parent` | Assert new container's parent is `container_ptr` | PASS | ✅ PASS |
| 7 | `test_new_container_empty_children` | Assert new container has empty children sequence | PASS | ✅ PASS |
| 8 | `test_new_container_depth` | Assert depth = parent's depth + 1 | PASS | ✅ PASS |
| 9 | `test_new_container_empty_subtree` | Assert new container's subtree_set is empty | PASS | ✅ PASS |
| 10 | `test_parent_children_grew` | Assert parent's children list has new container appended, length incremented | PASS | ✅ PASS |
| 11 | `test_parent_preserved` | Assert parent's depth, uppertree, parent, parent_rev_ptr unchanged | PASS | ✅ PASS |
| 12 | `test_new_container_uppertree` | Assert uppertree_seq = parent's uppertree pushed with parent | PASS | ✅ PASS |
| 13 | `test_both_subtree_wf` | Assert both old and new `container_subtree_set_wf` hold | PASS | ✅ PASS |
| 14 | `test_parent_rev_ptr_match` | Assert parent_rev_ptr is set and matches children's node ref | PASS | ✅ PASS |

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

**Verification result: 7 verified, 5 errors**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_no_precondition` | Call lemma with no requires at all | FAIL | ✅ FAIL |
| 2 | `test_only_perms_wf` | Only `container_perms_wf`, missing tree/domain conditions | FAIL | ✅ FAIL |
| 3 | `test_only_tree_wf` | Only `container_tree_wf`, missing domain/relationship conditions | FAIL | ✅ FAIL |
| 4 | `test_postcondition_without_lemma` | Assert postcondition without calling the lemma or having precondition | FAIL | ✅ FAIL |
| 5 | `test_new_container_already_in_domain` | Assume `new_container_ptr` already in old domain (violates key precondition) | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions

**Verification result: 7 verified, 5 errors**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_assert_full_tree_wf` | Assert full `container_tree_wf` for new_perms (only `container_subtree_set_wf` guaranteed) | FAIL | ✅ FAIL |
| 2 | `test_assert_children_parent_wf_new` | Assert `container_childern_parent_wf` for new_perms | FAIL | ✅ FAIL |
| 3 | `test_assert_uppertree_wf_new` | Assert `container_uppertree_seq_wf` for new_perms (rlimit exceeded) | FAIL | ✅ FAIL |
| 4 | `test_assert_linkedlist_wf_new` | Assert `containers_linkedlist_wf` for new_perms | FAIL | ✅ FAIL |
| 5 | `test_subtree_wf_arbitrary_perms` | Assert `container_subtree_set_wf` for unrelated arbitrary perms | FAIL | ✅ FAIL |

### Round 3: Negated/Contradicted Postconditions

**Verification result: 7 verified, 5 errors**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_negate_postcondition` | Assert `!container_subtree_set_wf(root, new_perms)` after lemma call | FAIL | ✅ FAIL |
| 2 | `test_postcondition_equals_false` | Assert `container_subtree_set_wf(root, new_perms) == false` | FAIL | ✅ FAIL |
| 3 | `test_negate_old_subtree_wf` | Assert `!container_subtree_set_wf(root, old_perms)` (contradicts precondition) | FAIL | ✅ FAIL |
| 4 | `test_negate_old_tree_wf` | Assert `!container_tree_wf(root, old_perms)` (contradicts precondition) | FAIL | ✅ FAIL |
| 5 | `test_assert_false` | Assert `false` after valid lemma call | FAIL | ✅ FAIL |

### Round 4: Wrong Specific Values

**Verification result: 7 verified, 5 errors**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_depth_zero` | Assert new container has depth 0 (should be parent_depth + 1) | FAIL | ✅ FAIL |
| 2 | `test_wrong_parent` | Assert new container's parent is root (should be container_ptr) | FAIL | ✅ FAIL |
| 3 | `test_not_in_domain` | Assert new container NOT in new domain (contradicts construction) | FAIL | ✅ FAIL |
| 4 | `test_wrong_domain_equality` | Assert old domain equals new domain (new has extra entry) | FAIL | ✅ FAIL |
| 5 | `test_parent_children_unchanged` | Assert parent's children list unchanged (it grew by 1) | FAIL | ✅ FAIL |

### Round 5: Cross-function Misuse & Edge Cases

**Verification result: 7 verified, 5 errors**

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_swapped_container_args` | Call lemma with swapped container_ptr/new_container_ptr | FAIL | ✅ FAIL |
| 2 | `test_swapped_perms` | Call lemma with swapped old/new perms | FAIL | ✅ FAIL |
| 3 | `test_unrelated_perms` | Assert subtree_set_wf for completely unrelated perms | FAIL | ✅ FAIL |
| 4 | `test_nonempty_children` | Assert new container's children list non-empty (should be empty) | FAIL | ✅ FAIL |
| 5 | `test_ptrs_equal` | Assert container_ptr == new_container_ptr (impossible) | FAIL | ✅ FAIL |

---

## Overall Assessment

### Correctness: ✅ PASS
All 14 correctness tests verify successfully (21 verified items total including definitions). The spec correctly captures the expected behavior: calling `new_container_preserve_tree_inv_4` with valid `new_container_ensures` preconditions yields `container_subtree_set_wf` for the new container permissions.

### Completeness: ✅ PASS
All 25 completeness tests fail as expected (5 per round). The specs reject:
- Calls without proper preconditions (round 1)
- Overly strong postcondition claims beyond `container_subtree_set_wf` (round 2)
- Negated/contradicted postconditions (round 3)
- Wrong specific values for depth, parent, domain (round 4)
- Cross-function misuse patterns like swapped arguments (round 5)

### Spec Gaps: None Found
No completeness tests passed unexpectedly. The specs are both correct and complete for the stated invariant. The lemma precisely guarantees `container_subtree_set_wf` and nothing more — other tree invariants (`container_tree_wf` components like `container_childern_parent_wf`, `containers_linkedlist_wf`, `container_uppertree_seq_wf`) are correctly rejected as not provable from this lemma alone.
