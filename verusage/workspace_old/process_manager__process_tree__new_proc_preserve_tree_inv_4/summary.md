# Specification Testing Summary

## File Under Test
`process_manager__process_tree__new_proc_preserve_tree_inv_4.rs`

Defines process tree invariants for a verified process manager. The main proof function `new_proc_preserve_tree_inv_4` proves that adding a new child process to an existing process in the tree preserves the **subtree set well-formedness** invariant (`proc_subtree_set_wf`) on the expanded domain.

### Key Specs
- `new_proc_ensures` (open): Describes the complete postcondition of adding a new process (parent/child relationships, depth, uppertree_seq, subtree_set updates, children list update, domain expansion)
- `proc_subtree_set_wf` (closed): Every member of a process's subtree set is in the tree domain and has the process at the correct position in its uppertree sequence
- `proc_tree_wf` (open): Conjunction of 7 closed invariants (root_wf, children_parent_wf, linkedlist_wf, children_depth_wf, subtree_set_wf, uppertree_seq_wf, subtree_set_exclusive)
- `seq_push_lemma`, `seq_push_unique_lemma` (external_body): Helper lemmas for sequence operations

---

## Correctness Results (should PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_postcondition_holds` | Call lemma, assert `proc_subtree_set_wf` on new domain | PASS | ✅ PASS |
| 2 | `test_old_tree_wf` | Assert old tree was well-formed (from precondition) | PASS | ✅ PASS |
| 3 | `test_domain_membership` | Verify proc_ptr in domain, new_proc_ptr not in domain | PASS | ✅ PASS |
| 4 | `test_new_proc_parent` | Assert new process parent is proc_ptr | PASS | ✅ PASS |
| 5 | `test_new_proc_empty_subtree_children` | Assert new process has empty subtree and children | PASS | ✅ PASS |
| 6 | `test_new_proc_uppertree` | Assert new process uppertree_seq is parent's pushed with proc_ptr | PASS | ✅ PASS |
| 7 | `test_parent_children_updated` | Assert parent's children list has new_proc_ptr pushed | PASS | ✅ PASS |
| 8 | `test_new_perms_domain` | Assert new_proc_perms domain is old domain + new_proc_ptr | PASS | ✅ PASS |
| 9 | `test_combined_pre_post` | Combined: check old tree wf, call lemma, check postcondition | PASS | ✅ PASS |
| 10 | `test_new_proc_depth` | Assert new process depth is parent's depth + 1 | PASS | ✅ PASS |

**Result: 17 verified, 0 errors** (10 test functions + 7 base definitions)

---

## Completeness Results (should FAIL)

### Round 1: Precondition Violations

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_no_requires` | Call lemma with no precondition at all | FAIL | ✅ FAIL (precondition not satisfied) |
| 2 | `test_only_tree_wf` | Only assume proc_tree_wf, not full new_proc_ensures | FAIL | ✅ FAIL (precondition not satisfied) |
| 3 | `test_only_perms_wf` | Only assume proc_perms_wf, not full new_proc_ensures | FAIL | ✅ FAIL (precondition not satisfied) |

**Result: 7 verified, 3 errors** ✅

### Round 2: Overly Strong Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_full_tree_wf` | Assert full proc_tree_wf (all 7 invariants, not just subtree_set_wf) | FAIL | ✅ FAIL (assertion failed) |
| 2 | `test_all_subtrees_unchanged` | Assert ALL subtree sets unchanged (wrong: ancestors gain new_proc_ptr) | FAIL | ✅ FAIL (assertion failed) |
| 3 | `test_uppertree_wf_preserved` | Assert proc_uppertree_seq_wf on new domain (not proven by this lemma) | FAIL | ✅ FAIL (assertion failed) |

**Result: 7 verified, 3 errors** ✅

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_negated_subtree_set_wf` | Assert NOT proc_subtree_set_wf after calling lemma | FAIL | ✅ FAIL (assertion failed) |
| 2 | `test_negated_old_tree_wf` | Assert old tree was NOT well-formed (contradicts precondition) | FAIL | ✅ FAIL (assertion failed) |
| 3 | `test_negated_new_proc_not_in_dom` | Assert new_proc_ptr was already in old domain (contradicts precondition) | FAIL | ✅ FAIL (assertion failed) |

**Result: 7 verified, 3 errors** ✅

### Round 4: Wrong Specific Values

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_new_proc_no_parent` | Assert new process has parent == None (should be Some(proc_ptr)) | FAIL | ✅ FAIL (assertion failed) |
| 2 | `test_parent_children_unchanged` | Assert parent's children list is unchanged (should have push) | FAIL | ✅ FAIL (assertion failed) |
| 3 | `test_new_proc_nonempty_subtree` | Assert new process subtree contains proc_ptr (should be empty) | FAIL | ✅ FAIL (assertion failed) |

**Result: 7 verified, 3 errors** ✅

### Round 5: Cross-function Misuse & Edge Cases

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_domain_old` | Assert subtree_set_wf on old domain (not inserting new_proc_ptr) | FAIL | ✅ FAIL (assertion failed) |
| 2 | `test_wrong_perms_old` | Assert subtree_set_wf with old_proc_perms (should be new_proc_perms) | FAIL | ✅ FAIL (assertion failed) |
| 3 | `test_swapped_proc_ptrs` | Call lemma with proc_ptr and new_proc_ptr swapped | FAIL | ✅ FAIL (precondition not satisfied) |

**Result: 7 verified, 3 errors** ✅

---

## Overall Assessment

### Correctness: ✅ PASS
All 10 correctness tests verified successfully. The lemma correctly proves `proc_subtree_set_wf` on the expanded domain given `new_proc_ensures`. Properties from the precondition (old tree well-formedness, domain membership, parent relationships, depth, uppertree sequence) are all consistent and correctly specified.

### Completeness: ✅ PASS
All 15 completeness tests across 5 rounds failed as expected. The specs correctly reject:
- Missing or partial preconditions
- Overly strong postconditions (full tree wf, unchanged subtree sets, uppertree seq wf)
- Negated postconditions and precondition properties
- Wrong specific values (wrong parent, unchanged children, non-empty subtree)
- Wrong domain, wrong permissions, and swapped arguments

### Spec Gaps Found
**None.** The specifications appear both correct and complete:
- `new_proc_ensures` is tight enough to reject partial preconditions
- `proc_subtree_set_wf` is the precise postcondition — neither too strong nor too weak
- The lemma specifically proves variant 4 (subtree set) without accidentally proving other invariants (uppertree_seq_wf, proc_tree_wf as a whole)

### Note
`proc_root_wf` on the new domain IS derivable from `new_proc_ensures` alone (without the lemma), since the root's properties are preserved and the new process has depth > 0 and parent = Some. This is expected behavior, not a spec gap — it just means `proc_root_wf` preservation is a simpler property than `proc_subtree_set_wf`.
