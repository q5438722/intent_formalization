# Specification Testing Summary

## File Under Test

`process_manager__container_tree__new_container_preserve_tree_inv.rs`

This file defines a container tree data structure for a process manager. The main proof function `new_container_preserve_tree_inv` proves that adding a new container to the tree preserves the tree well-formedness invariant (`container_tree_wf`). The tree invariant is decomposed into 7 sub-properties, each proven by a separate `#[verifier::external_body]` sub-lemma (1–7).

### Key Specs
- **`new_container_ensures`** (open spec): Encapsulates all preconditions for adding a new container — old/new permissions well-formedness, domain changes, parent-child relationships, depth, uppertree sequence, subtree sets.
- **`container_tree_wf`** (open spec): Conjunction of 7 closed sub-properties (root wf, children-parent wf, linkedlist wf, depth wf, subtree set wf, uppertree seq wf, subtree set exclusivity).
- **`new_container_preserve_tree_inv`**: Main lemma — given `new_container_ensures`, proves `container_tree_wf` on the new state.
- **Sub-lemmas 1–7** (`external_body`): Each proves one component of `container_tree_wf` from `new_container_ensures`.

---

## Correctness Results (should all PASS)

**Command**: `./verus/verus workspace/.../correctness_tests.rs`
**Result**: ✅ **21 verified, 0 errors**

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_main_lemma_preserves_tree_wf` | Call main lemma, assert `container_tree_wf` on new perms | PASS | ✅ PASS |
| 2 | `test_sub_lemma_1_root_wf` | Sub-lemma 1 ensures `container_root_wf` | PASS | ✅ PASS |
| 3 | `test_sub_lemma_2_children_parent_wf` | Sub-lemma 2 ensures `container_childern_parent_wf` | PASS | ✅ PASS |
| 4 | `test_sub_lemma_3_children_depth_wf` | Sub-lemma 3 ensures `container_childern_depth_wf` | PASS | ✅ PASS |
| 5 | `test_sub_lemma_4_subtree_set_wf` | Sub-lemma 4 ensures `container_subtree_set_wf` | PASS | ✅ PASS |
| 6 | `test_sub_lemma_5_uppertree_seq_wf` | Sub-lemma 5 ensures `container_uppertree_seq_wf` | PASS | ✅ PASS |
| 7 | `test_sub_lemma_6_subtree_exclusive` | Sub-lemma 6 ensures `container_subtree_set_exclusive` | PASS | ✅ PASS |
| 8 | `test_sub_lemma_7_linkedlist_wf` | Sub-lemma 7 ensures `containers_linkedlist_wf` | PASS | ✅ PASS |
| 9 | `test_ensures_implies_old_perms_wf` | `new_container_ensures` implies `container_perms_wf(old)` | PASS | ✅ PASS |
| 10 | `test_ensures_implies_new_perms_wf` | `new_container_ensures` implies `container_perms_wf(new)` | PASS | ✅ PASS |
| 11 | `test_ensures_implies_old_tree_wf` | `new_container_ensures` implies `container_tree_wf(root, old)` | PASS | ✅ PASS |
| 12 | `test_ensures_domain_relationship` | Domain contains container_ptr, excludes new_container_ptr, new = old ∪ {new_ptr} | PASS | ✅ PASS |
| 13 | `test_ensures_new_container_structure` | New container: parent=Some(container_ptr), empty children, empty subtree_set | PASS | ✅ PASS |
| 14 | `test_ensures_parent_children_updated` | Parent's children list = old.push(new_container_ptr), len grows by 1 | PASS | ✅ PASS |

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

**Command**: `./verus/verus workspace/.../completeness_round1.rs`
**Result**: ✅ **7 verified, 5 errors** (all 5 tests fail as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_no_precondition_sub_lemma_1` | Call sub-lemma 1 without `new_container_ensures` | FAIL | ✅ FAIL |
| 2 | `test_no_precondition_sub_lemma_2` | Call sub-lemma 2 without `new_container_ensures` | FAIL | ✅ FAIL |
| 3 | `test_no_precondition_main_lemma` | Call main lemma without any preconditions | FAIL | ✅ FAIL |
| 4 | `test_no_precondition_sub_lemma_4` | Call sub-lemma 4 without `new_container_ensures` | FAIL | ✅ FAIL |
| 5 | `test_no_precondition_sub_lemma_7` | Call sub-lemma 7 without `new_container_ensures` | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions

**Command**: `./verus/verus workspace/.../completeness_round2.rs`
**Result**: ✅ **7 verified, 5 errors** (all 5 tests fail as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_sub_lemma_1_too_strong` | Sub-lemma 1 → assert full `container_tree_wf` (only ensures `root_wf`) | FAIL | ✅ FAIL |
| 2 | `test_domain_too_strong` | Assert `new_perms.dom() =~= old_perms.dom()` (should be `.insert(new_ptr)`) | FAIL | ✅ FAIL |
| 3 | `test_perms_equality_too_strong` | Assert `old_perms =~= new_perms` (they differ) | FAIL | ✅ FAIL |
| 4 | `test_children_unchanged_too_strong` | Assert parent's children unchanged (should have new_ptr appended) | FAIL | ✅ FAIL |
| 5 | `test_new_container_children_too_strong` | Assert new container has children (should be empty) | FAIL | ✅ FAIL |

### Round 3: Negated/Contradicted Postconditions

**Command**: `./verus/verus workspace/.../completeness_round3.rs`
**Result**: ✅ **7 verified, 5 errors** (all 5 tests fail as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_negate_tree_wf` | Assert `!container_tree_wf(root, new_perms)` after main lemma | FAIL | ✅ FAIL |
| 2 | `test_negate_root_wf` | Assert `!container_root_wf(root, new_perms)` after sub-lemma 1 | FAIL | ✅ FAIL |
| 3 | `test_negate_old_perms_wf` | Assert `!container_perms_wf(old)` (contradicts `new_container_ensures`) | FAIL | ✅ FAIL |
| 4 | `test_negate_parent` | Assert new container parent is `None` (should be `Some(container_ptr)`) | FAIL | ✅ FAIL |
| 5 | `test_negate_subtree_exclusive` | Assert `!container_subtree_set_exclusive` after sub-lemma 6 | FAIL | ✅ FAIL |

### Round 4: Wrong Specific Values

**Command**: `./verus/verus workspace/.../completeness_round4.rs`
**Result**: ✅ **7 verified, 5 errors** (all 5 tests fail as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_depth` | Assert new container depth == 0 (should be parent depth + 1) | FAIL | ✅ FAIL |
| 2 | `test_wrong_children_len` | Assert parent children len unchanged (should grow by 1) | FAIL | ✅ FAIL |
| 3 | `test_wrong_domain_membership` | Assert new_container_ptr in old domain (should be absent) | FAIL | ✅ FAIL |
| 4 | `test_wrong_uppertree_seq` | Assert new container uppertree_seq empty (should be non-empty) | FAIL | ✅ FAIL |
| 5 | `test_wrong_subtree_set` | Assert new container subtree_set contains parent (should be empty) | FAIL | ✅ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases

**Command**: `./verus/verus workspace/.../completeness_round5.rs`
**Result**: ✅ **7 verified, 5 errors** (all 5 tests fail as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_swap_old_new_perms` | Call main lemma with old/new perms swapped | FAIL | ✅ FAIL |
| 2 | `test_swap_container_ptrs` | Call main lemma with container_ptr/new_container_ptr swapped | FAIL | ✅ FAIL |
| 3 | `test_wrong_root` | Assert `container_tree_wf(new_container_ptr, new_perms)` (wrong root) | FAIL | ✅ FAIL |
| 4 | `test_parent_rev_ptr_none` | Assert new container's `parent_rev_ptr` is `None` (should be `Some`) | FAIL | ✅ FAIL |
| 5 | `test_misuse_sub_lemma_on_empty` | After sub-lemma 1, assert `container_root_wf(root, Map::empty())` | FAIL | ✅ FAIL |

---

## Overall Assessment

### Correctness: ✅ PASS
All 14 correctness tests verify successfully. The specs correctly describe the behavior of adding a new container to the tree:
- The main lemma correctly preserves `container_tree_wf`.
- Each sub-lemma correctly proves its assigned component.
- The `new_container_ensures` spec correctly captures all structural properties of the new/old state.

### Completeness: ✅ PASS
All 25 completeness tests (5 rounds × 5 tests) fail with verification errors as expected. The specs are tight enough to reject:
- Calls without preconditions (Round 1)
- Overly strong postcondition claims (Round 2)
- Negated/contradicted postconditions (Round 3)
- Wrong specific values (Round 4)
- Argument misuse and cross-function errors (Round 5)

### Spec Gaps Found: None
No spec gaps were identified. The specifications are both correct and complete for the tested properties.
