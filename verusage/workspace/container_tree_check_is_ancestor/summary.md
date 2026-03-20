# Test Summary: container_tree_check_is_ancestor

## File Under Test

`process_manager__container_tree__container_tree_check_is_ancestor.rs` defines a container tree structure for a process manager. The main function `container_tree_check_is_ancestor` checks whether container `a_ptr` is an ancestor of `child_ptr` by walking up the tree. It returns `true` iff `a_ptr` appears in `child_ptr`'s `uppertree_seq` (equivalently, iff `child_ptr` is in `a_ptr`'s `subtree_set`).

Key specs tested:
- `container_perms_wf` (open): containers are init, addr matches, children wf/unique, no self-child, subtree finite, depth == uppertree_seq.len()
- `container_tree_wf` (open): conjunction of 7 closed specs governing root, parent-child, linked list, depth, subtree set, uppertree seq, and subtree-uppertree exclusivity
- `seq_push_lemma`: helper lemma about Seq.push preserving/adding containment
- `container_tree_check_is_ancestor` ensures: `ret == uppertree_seq.contains(a_ptr) == subtree_set.contains(child_ptr)`

---

## Correctness Results (correctness_tests.rs) — 30 verified, 0 errors

All tests **PASS** as expected.

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | test_seq_push_contains_pushed | Push element is contained | PASS | PASS ✅ |
| 2 | test_seq_push_preserves_existing | Existing elements preserved after push | PASS | PASS ✅ |
| 3 | test_seq_push_not_contained_stays | Non-contained different element stays out after push | PASS | PASS ✅ |
| 4 | test_seq_push_always_contains_pushed | Push always adds element | PASS | PASS ✅ |
| 5 | test_perms_wf_is_init | perms_wf ⟹ is_init() | PASS | PASS ✅ |
| 6 | test_perms_wf_addr | perms_wf ⟹ addr() == c_ptr | PASS | PASS ✅ |
| 7 | test_perms_wf_children_wf | perms_wf ⟹ children.wf() | PASS | PASS ✅ |
| 8 | test_perms_wf_children_unique | perms_wf ⟹ children.unique() | PASS | PASS ✅ |
| 9 | test_perms_wf_uppertree_no_duplicates | perms_wf ⟹ uppertree_seq.no_duplicates() | PASS | PASS ✅ |
| 10 | test_perms_wf_not_self_child | perms_wf ⟹ not own child | PASS | PASS ✅ |
| 11 | test_perms_wf_subtree_finite | perms_wf ⟹ subtree_set.finite() | PASS | PASS ✅ |
| 12 | test_perms_wf_depth_eq_uppertree_len | perms_wf ⟹ depth == uppertree_seq.len() | PASS | PASS ✅ |
| 13 | test_tree_wf_implies_root_wf | tree_wf ⟹ root_wf | PASS | PASS ✅ |
| 14 | test_tree_wf_implies_subtree_exclusive | tree_wf ⟹ subtree_set_exclusive | PASS | PASS ✅ |
| 15 | test_tree_wf_implies_subtree_set_wf | tree_wf ⟹ subtree_set_wf | PASS | PASS ✅ |
| 16 | test_tree_wf_implies_uppertree_seq_wf | tree_wf ⟹ uppertree_seq_wf | PASS | PASS ✅ |
| 17 | test_tree_wf_implies_children_parent_wf | tree_wf ⟹ children_parent_wf | PASS | PASS ✅ |
| 18 | test_tree_wf_implies_linkedlist_wf | tree_wf ⟹ linkedlist_wf | PASS | PASS ✅ |
| 19 | test_tree_wf_implies_depth_wf | tree_wf ⟹ depth_wf | PASS | PASS ✅ |
| 20 | test_root_wf_root_depth_zero | reveal root_wf: root has depth 0 | PASS | PASS ✅ |
| 21 | test_root_wf_nonroot_nonzero_depth | reveal root_wf: non-root has depth ≠ 0 | PASS | PASS ✅ |
| 22 | test_root_wf_nonroot_has_parent | reveal root_wf: non-root has parent | PASS | PASS ✅ |
| 23 | test_ensures_consistency | Both ensures agree (uppertree ↔ subtree) | PASS | PASS ✅ |
| 24 | test_ensures_true_implies_containment | ret=true ⟹ both containments hold | PASS | PASS ✅ |
| 25 | test_ensures_false_implies_non_containment | ret=false ⟹ neither containment holds | PASS | PASS ✅ |
| 26-30 | (spec definition verifications) | Definitions compile correctly | PASS | PASS ✅ |

---

## Completeness Results

### Round 1: Precondition Violations — 5 verified, 5 errors

All tests **FAIL** as expected. Missing preconditions are correctly rejected.

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_no_perms_wf_assert_init | Assert is_init without perms_wf | FAIL | FAIL ✅ |
| 2 | test_no_perms_wf_assert_addr | Assert addr match without perms_wf | FAIL | FAIL ✅ |
| 3 | test_no_perms_wf_assert_children_wf | Assert children.wf() without perms_wf | FAIL | FAIL ✅ |
| 4 | test_no_perms_wf_assert_depth_eq_len | Assert depth==len without perms_wf | FAIL | FAIL ✅ |
| 5 | test_no_dom_contains_assert_not_self_child | Assert not-self-child without dom.contains | FAIL | FAIL ✅ |

### Round 2: Overly Strong Postconditions — 5 verified, 5 errors

All tests **FAIL** as expected. Specs don't over-promise.

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_perms_wf_implies_has_parent | perms_wf does NOT imply parent.is_Some() | FAIL | FAIL ✅ |
| 2 | test_perms_wf_implies_depth_zero | perms_wf does NOT imply depth == 0 | FAIL | FAIL ✅ |
| 3 | test_perms_wf_implies_children_nonempty | perms_wf does NOT imply children non-empty | FAIL | FAIL ✅ |
| 4 | test_perms_wf_implies_subtree_nonempty | perms_wf does NOT imply subtree non-empty | FAIL | FAIL ✅ |
| 5 | test_perms_wf_implies_uppertree_nonempty | perms_wf does NOT imply uppertree non-empty | FAIL | FAIL ✅ |

### Round 3: Negated/Contradicted Postconditions — 5 verified, 5 errors

All tests **FAIL** as expected. Negating spec properties is correctly rejected.

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_perms_wf_negated_init | Assert !is_init despite perms_wf | FAIL | FAIL ✅ |
| 2 | test_perms_wf_negated_addr | Assert addr != c_ptr despite perms_wf | FAIL | FAIL ✅ |
| 3 | test_perms_wf_negated_children_wf | Assert !children.wf() despite perms_wf | FAIL | FAIL ✅ |
| 4 | test_perms_wf_negated_unique | Assert !unique() despite perms_wf | FAIL | FAIL ✅ |
| 5 | test_root_wf_negated_root_depth | Assert root depth != 0 despite root_wf | FAIL | FAIL ✅ |

### Round 4: Wrong Specific Values — 5 verified, 5 errors

All tests **FAIL** as expected. Incorrect concrete values are rejected.

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_seq_push_wrong_not_contains | Assert pushed element NOT contained | FAIL | FAIL ✅ |
| 2 | test_seq_push_wrong_removes_existing | Assert existing element disappears after push | FAIL | FAIL ✅ |
| 3 | test_perms_wf_wrong_addr_plus_one | Assert addr == c_ptr + 1 | FAIL | FAIL ✅ |
| 4 | test_perms_wf_wrong_depth_off_by_one | Assert depth == uppertree.len() + 1 | FAIL | FAIL ✅ |
| 5 | test_perms_wf_wrong_self_child | Assert container IS its own child | FAIL | FAIL ✅ |

### Round 5: Cross-function Misuse & Edge Cases — 5 verified, 5 errors

All tests **FAIL** as expected. Cross-function misuse is rejected.

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_perms_wf_does_not_imply_tree_wf | perms_wf alone doesn't imply tree_wf | FAIL | FAIL ✅ |
| 2 | test_tree_wf_wrong_root | tree_wf with different root fails | FAIL | FAIL ✅ |
| 3 | test_swapped_containment | Child can't be ancestor of parent | FAIL | FAIL ✅ |
| 4 | test_equal_depth_containment | Equal-depth nodes can't be ancestor/descendant | FAIL | FAIL ✅ |
| 5 | test_outside_domain_in_subtree | Out-of-domain node can't be in subtree | FAIL | FAIL ✅ |

---

## Overall Assessment

### Correctness: ✅ PASS
All 30 correctness tests verify successfully. The specifications correctly describe:
- Well-formedness properties of container permissions
- Tree structure invariants (root, parent-child, depth, subtree/uppertree relationships)
- The seq_push_lemma helper properties
- Consistency of the function's two ensures clauses

### Completeness: ✅ PASS
All 25 completeness tests fail as expected across 5 rounds. The specifications are tight enough to reject:
- Assertions made without sufficient preconditions
- Overly strong claims not guaranteed by the specs
- Negations of guaranteed properties
- Incorrect specific values
- Cross-function misuse and edge cases

### Spec Gaps Found: None
No unexpected passes were observed in completeness tests. The specifications appear both correct and complete for the tested properties.
