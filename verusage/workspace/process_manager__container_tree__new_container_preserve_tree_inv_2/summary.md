# Summary: new_container_preserve_tree_inv_2

## File Under Test
`process_manager__container_tree__new_container_preserve_tree_inv_2.rs`

Defines a proof function `new_container_preserve_tree_inv_2` that proves when a new container is added to a container tree (satisfying `new_container_ensures`), the children-parent well-formedness invariant (`container_childern_parent_wf`) is preserved in the new container permissions map. Also includes helper spec functions for tree invariants and external-body sequence lemmas.

## Correctness Results (all should PASS)

| Test | Description | Expected | Actual |
|------|-------------|----------|--------|
| test_param_basic | Call lemma, check postcondition holds | PASS | PASS ✅ |
| test_param_old_tree_wf | Precondition implies old tree was well-formed | PASS | PASS ✅ |
| test_param_perms_wf | Precondition implies old and new perms are well-formed | PASS | PASS ✅ |
| test_param_domain_relations | container_ptr in old domain, new_container_ptr not in old, new domain = old + new | PASS | PASS ✅ |
| test_param_new_container_props | New container has correct parent, empty children, empty subtree | PASS | PASS ✅ |
| test_param_depth_relation | New container depth = parent depth + 1 | PASS | PASS ✅ |
| test_param_parent_children_updated | Parent's children list includes new container, length +1 | PASS | PASS ✅ |
| test_param_old_and_new_parent_wf | Old tree + lemma call → new tree has container_childern_parent_wf | PASS | PASS ✅ |
| test_seq_push_lemma_basic | seq_push_lemma ensures pushed value is contained | PASS | PASS ✅ |
| test_seq_push_unique_lemma_basic | seq_push_unique_lemma is callable | PASS | PASS ✅ |
| test_param_unchanged_containers | Containers other than container_ptr have unchanged properties | PASS | PASS ✅ |
| test_param_parent_preserved_fields | Parent's parent, depth, uppertree_seq are unchanged | PASS | PASS ✅ |

**Result: 19 verified, 0 errors**

## Completeness Results (all should FAIL)

### Round 1: Precondition Violations

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| test_no_preconditions | Call lemma with no requires | FAIL | FAIL ✅ |
| test_only_perms_wf | Only perms_wf, no tree_wf or domain conditions | FAIL | FAIL ✅ |
| test_container_not_in_domain | container_ptr not in old domain | FAIL | FAIL ✅ |
| test_new_container_already_exists | new_container_ptr already in old domain | FAIL | FAIL ✅ |
| test_postcondition_without_lemma | Assert postcondition without calling lemma | FAIL | FAIL ✅ |

**Result: 7 verified, 5 errors** (all 5 tests correctly fail)

### Round 2: Overly Strong Postconditions

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| test_assert_full_tree_wf | Assert full container_tree_wf (only parent_wf guaranteed) | FAIL | FAIL ✅ |
| test_assert_children_len_plus_2 | Assert children length +2 (should be +1) | FAIL | FAIL ✅ |
| test_assert_nonempty_subtree | Assert new container's subtree contains parent (should be empty) | FAIL | FAIL ✅ |
| test_assert_uppertree_seq_wf | Assert uppertree_seq_wf for new perms (not guaranteed) | FAIL | FAIL ✅ |
| test_assert_depth_too_large | Assert depth = parent depth + 2 (should be +1) | FAIL | FAIL ✅ |

**Result: 7 verified, 5 errors** (all 5 tests correctly fail)

### Round 3: Negated/Contradicted Postconditions

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| test_negate_postcondition | Assert NOT container_childern_parent_wf | FAIL | FAIL ✅ |
| test_negate_old_tree_wf | Assert old tree was NOT well-formed | FAIL | FAIL ✅ |
| test_negate_container_in_domain | Assert container_ptr NOT in old domain | FAIL | FAIL ✅ |
| test_negate_new_container_parent | Assert new container has no parent (None) | FAIL | FAIL ✅ |
| test_negate_domain_change | Assert domains are equal (no new container added) | FAIL | FAIL ✅ |

**Result: 7 verified, 5 errors** (all 5 tests correctly fail)

### Round 4: Wrong Specific Values

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| test_wrong_depth_same_as_parent | Assert depth equals parent depth (should be +1) | FAIL | FAIL ✅ |
| test_wrong_children_len_unchanged | Assert children length unchanged (should be +1) | FAIL | FAIL ✅ |
| test_wrong_new_container_has_children | Assert new container has non-empty children | FAIL | FAIL ✅ |
| test_wrong_depth_zero | Assert new container depth is 0 (should be > 1) | FAIL | FAIL ✅ |
| test_wrong_new_in_old_domain | Assert new_container_ptr in old domain | FAIL | FAIL ✅ |

**Result: 7 verified, 5 errors** (all 5 tests correctly fail)

### Round 5: Cross-function Misuse & Edge Cases

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| test_wrong_domain_equality | Assert new domain equals old domain (missing new container) | FAIL | FAIL ✅ |
| test_same_container_ptr | Assert container_ptr == new_container_ptr (must differ) | FAIL | FAIL ✅ |
| test_postcondition_swapped_perms | Assert new_container_ptr == root_container | FAIL | FAIL ✅ |
| test_wrong_parent_is_root | Assert parent is root_container (should be container_ptr) | FAIL | FAIL ✅ |
| test_wrong_old_domain_extended | Assert old domain contains new_container_ptr | FAIL | FAIL ✅ |

**Result: 7 verified, 5 errors** (all 5 tests correctly fail)

## Overall Assessment

- **Correctness**: ✅ All 12 correctness tests pass (19 verified, 0 errors). The spec correctly captures the preservation of the children-parent well-formedness invariant when adding a new container.
- **Completeness**: ✅ All 25 completeness tests fail as expected. The specs are tight enough to reject:
  - Calls without proper preconditions
  - Overly strong postcondition claims
  - Negated/contradicted properties
  - Incorrect specific values
  - Cross-function misuse

**Notable observations:**
- The `new_container_ensures` precondition is very rich, providing detailed information about how the new container perms relate to the old ones.
- Several closed spec functions (like `container_childern_parent_wf`, `container_root_wf`, `container_childern_depth_wf`) can be derived for the new perms from the preconditions alone, even though the lemma only explicitly guarantees `container_childern_parent_wf`.
- The specs appear both correct and complete for this proof function's interface.
