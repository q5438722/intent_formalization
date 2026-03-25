# Summary: Specification Testing for `new_proc_preserve_tree_inv_2`

## File Under Test

**File**: `process_manager__process_tree__new_proc_preserve_tree_inv_2.rs`

Defines a process tree data structure for a verified OS process manager. The main proof function `new_proc_preserve_tree_inv_2` proves that adding a new child process to an existing process in the tree preserves the **children-parent well-formedness invariant** (`proc_childern_parent_wf`).

### Key Components
- `new_proc_ensures` (open spec): Rich precondition describing how old/new process permissions relate when a new process is added
- `proc_tree_wf` (open spec): Full tree well-formedness (conjunction of 7 closed invariants)
- `proc_childern_parent_wf` (closed spec): Parent-child consistency invariant
- `seq_push_lemma` / `seq_push_unique_lemma`: Helper lemmas for sequence push operations

---

## Correctness Results (should all PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_main_lemma_postcondition` | Call lemma, assert `proc_childern_parent_wf` on extended domain | PASS | PASS |
| 2 | `test_seq_push_contains_new` | `seq_push_lemma`: pushed element is contained | PASS | PASS |
| 3 | `test_seq_push_preserves_existing` | `seq_push_lemma`: existing elements preserved | PASS | PASS |
| 4 | `test_seq_push_non_contained` | `seq_push_lemma`: non-contained element stays non-contained | PASS | PASS |
| 5 | `test_seq_push_unique_no_dup` | `seq_push_unique_lemma`: push preserves no_duplicates | PASS | PASS |
| 6 | `test_seq_push_unique_index_of_new` | `seq_push_unique_lemma`: new element gets last index | PASS | PASS |
| 7 | `test_seq_push_unique_index_of_existing` | `seq_push_unique_lemma`: existing indices unchanged | PASS | PASS |
| 8 | `test_new_proc_ensures_structure` | Precondition implies new proc parent/children/subtree | PASS | PASS |
| 9 | `test_new_proc_ensures_dom_extended` | Precondition implies domain extended by new_proc_ptr | PASS | PASS |
| 10 | `test_new_proc_ensures_depth` | Precondition implies depth = parent depth + 1 | PASS | PASS |
| 11 | `test_new_proc_ensures_old_tree_wf` | Precondition implies old tree was well-formed | PASS | PASS |
| 12 | `test_new_proc_ensures_unchanged_nodes` | Non-parent nodes unchanged in new perms | PASS | PASS |
| 13 | `test_new_proc_ensures_uppertree` | New proc uppertree is parent uppertree + parent | PASS | PASS |
| 14 | `test_new_proc_ensures_children_len` | Parent children length increased by 1 | PASS | PASS |
| 15 | `test_main_lemma_idempotent` | Calling lemma twice yields same result | PASS | PASS |

**Result**: 22 verified, 0 errors

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_no_precondition` | Call lemma with no requires at all | FAIL | FAIL |
| 2 | `test_new_proc_already_in_dom` | Call with new_proc_ptr already in domain | FAIL | FAIL |
| 3 | `test_proc_ptr_not_in_dom` | Call with proc_ptr not in domain | FAIL | FAIL |
| 4 | `test_old_tree_not_wf` | Call without old tree well-formedness | FAIL | FAIL |
| 5 | `test_children_list_full` | Call when parent children list is full | FAIL | FAIL |

**Result**: 7 verified, 5 errors (all tests rejected)

### Round 2: Overly Strong Postconditions

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_assert_on_larger_domain` | Assert postcondition on domain with extra element | FAIL | FAIL |
| 2 | `test_assert_perms_unchanged` | Assert old_proc_perms == new_proc_perms | FAIL | FAIL |
| 3 | `test_assert_subtree_set_wf` | Assert `proc_subtree_set_wf` on new domain | FAIL | FAIL |
| 4 | `test_assert_uppertree_seq_wf` | Assert `proc_uppertree_seq_wf` on new domain | FAIL | FAIL |
| 5 | `test_assert_full_tree_wf` | Assert full `proc_tree_wf` on new domain | FAIL | FAIL |

**Result**: 7 verified, 5 errors (all tests rejected)

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_negate_postcondition` | Assert NOT proc_childern_parent_wf after lemma | FAIL | FAIL |
| 2 | `test_negate_new_proc_in_dom` | Assert new_proc_ptr NOT in extended domain | FAIL | FAIL |
| 3 | `test_negate_parent` | Assert new proc has no parent (None) | FAIL | FAIL |
| 4 | `test_negate_empty_children` | Assert new proc has non-empty children | FAIL | FAIL |
| 5 | `test_negate_empty_subtree` | Assert new proc has proc_ptr in subtree | FAIL | FAIL |

**Result**: 7 verified, 5 errors (all tests rejected)

### Round 4: Wrong Specific Values

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_depth_same_as_parent` | Assert new proc depth == parent depth (should be +1) | FAIL | FAIL |
| 2 | `test_wrong_children_len_unchanged` | Assert parent children.len() unchanged | FAIL | FAIL |
| 3 | `test_wrong_parent_self` | Assert new proc parent is itself | FAIL | FAIL |
| 4 | `test_wrong_depth_zero` | Assert new proc has depth 0 | FAIL | FAIL |
| 5 | `test_wrong_dom_unchanged` | Assert new_proc_perms.dom() == old_proc_perms.dom() | FAIL | FAIL |

**Result**: 7 verified, 5 errors (all tests rejected)

### Round 5: Cross-function Misuse & Edge Cases

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_domain_old` | Assert postcondition with old domain (not inserted) | FAIL | FAIL |
| 2 | `test_wrong_perms_old` | Assert postcondition with old_proc_perms | FAIL | FAIL |
| 3 | `test_old_perms_cover_new_domain` | Assert old perms cover extended domain | FAIL | FAIL |
| 4 | `test_seq_push_not_contains_wrong` | Negate seq_push_lemma: pushed element NOT contained | FAIL | FAIL |
| 5 | `test_new_proc_was_in_old_domain` | Assert new_proc_ptr was in original domain | FAIL | FAIL |

**Result**: 7 verified, 5 errors (all tests rejected)

---

## Overall Assessment

### Correctness: PASS
All 15 correctness tests verify successfully. The specs correctly describe the lemma postcondition, precondition sub-properties, and helper lemma behaviors.

### Completeness: PASS
All 25 completeness tests are rejected by the verifier. The specs reject precondition violations, overly strong claims, negated postconditions, wrong values, and cross-function misuse.

### Spec Gaps Found: None
The specifications are both correct and complete for their stated purpose. The lemma precisely proves `proc_childern_parent_wf` preservation on the extended domain without over-claiming (e.g., it does not prove full `proc_tree_wf`, which requires separate lemmas for each sub-invariant).
