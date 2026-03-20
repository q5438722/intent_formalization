# Test Summary: helper_kernel_kill_proc_non_root

## File Under Test
`kernel__kernel_kill_proc__impl0__helper_kernel_kill_proc_non_root.rs`

Defines `Kernel::helper_kernel_kill_proc_non_root(&mut self, proc_ptr: ProcPtr)` — kills a non-root process by removing it from the process tree, freeing its page table page, and releasing its pcid. Requires the process to have no children, no threads, no IO id, an empty page table, and non-zero depth.

## Correctness Results (all should PASS ✅)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | test_containers_tree_unchanged_reflexive | Reflexivity of containers_tree_unchanged | PASS | ✅ PASS |
| 2 | test_processes_fields_unchanged_reflexive | Reflexivity of processes_fields_unchanged | PASS | ✅ PASS |
| 3 | test_threads_unchanged_except_empty_reflexive | Reflexivity with empty exception set | PASS | ✅ PASS |
| 4 | test_threads_unchanged_reflexive | Reflexivity of threads_unchanged | PASS | ✅ PASS |
| 5 | test_threads_unchanged_except_singleton_reflexive | Reflexivity with singleton exception set | PASS | ✅ PASS |
| 6 | test_postcond_proc_removed | proc_ptr removed from domain | PASS | ✅ PASS |
| 7 | test_postcond_thread_dom_preserved | Thread domain preserved | PASS | ✅ PASS |
| 8 | test_postcond_container_dom_preserved | Container domain preserved | PASS | ✅ PASS |
| 9 | test_threads_unchanged_except_empty | All threads preserved with empty exception | PASS | ✅ PASS |
| 10 | test_postcond_remaining_proc_fields | Remaining procs preserve pcid, ioid, depth | PASS | ✅ PASS |
| 11 | test_containers_tree_unchanged_properties | Container parent/children/depth preserved | PASS | ✅ PASS |
| 12 | test_postcond_parent_children_count | Parent children count decreased | PASS | ✅ PASS |
| 13 | test_page_ptr2page_index_zero | page_ptr2page_index(0) == 0 | PASS | ✅ PASS |
| 14 | test_page_ptr2page_index_4096 | page_ptr2page_index(4096) == 1 | PASS | ✅ PASS |
| 15 | test_page_index2page_ptr_zero | page_index2page_ptr(0) == 0 | PASS | ✅ PASS |
| 16 | test_page_index2page_ptr_one | page_index2page_ptr(1) == 4096 | PASS | ✅ PASS |
| 17 | test_page_ptr_valid_zero | page_ptr_valid(0) is true | PASS | ✅ PASS |
| 18 | test_page_ptr_valid_4096 | page_ptr_valid(4096) is true | PASS | ✅ PASS |
| 19 | test_page_index_valid_zero | page_index_valid(0) is true | PASS | ✅ PASS |
| 20 | test_page_index_valid_max_minus_one | page_index_valid(NUM_PAGES-1) is true | PASS | ✅ PASS |
| 21 | test_page_index_invalid_num_pages | page_index_valid(NUM_PAGES) is false | PASS | ✅ PASS |
| 22 | test_postcond_non_parent_children_unchanged | Non-parent procs' children unchanged | PASS | ✅ PASS |
| 23 | test_postcond_upper_tree_subtree_update | Ancestors' subtree_set updated | PASS | ✅ PASS |
| 24 | test_postcond_upper_tree_seq_preserved | Upper tree sequences preserved | PASS | ✅ PASS |
| 25 | test_postcond_pagetable_preserved | Page table mappings preserved for remaining | PASS | ✅ PASS |
| 26 | test_postcond_parent_children_list_updated | Parent children list updated correctly | PASS | ✅ PASS |
| 27 | test_page_entry_is_empty | Zero PageEntry is empty | PASS | ✅ PASS |
| 28 | test_page_entry_not_empty_present | PageEntry with present=true not empty | PASS | ✅ PASS |
| 29 | test_page_index_2m_valid_zero | page_index_2m_valid(0) is true | PASS | ✅ PASS |
| 30 | test_page_index_2m_valid_512 | page_index_2m_valid(512) is true | PASS | ✅ PASS |
| 31 | test_page_index_2m_invalid_1 | page_index_2m_valid(1) is false | PASS | ✅ PASS |
| 32 | test_page_index_truncate_2m | truncate_2m(513) == 512 | PASS | ✅ PASS |
| 33 | test_page_index_truncate_2m_zero | truncate_2m(0) == 0 | PASS | ✅ PASS |
| 34 | test_processes_fields_unchanged_preserves | fields_unchanged preserves pcid/parent/threads | PASS | ✅ PASS |

**Total: 34 tests, 34 passed (74 verified items including definitions)**

## Completeness Results (all should FAIL ❌)

### Round 1: Precondition Violations

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_precond_proc_not_in_dom | proc_ptr not in domain | FAIL | ❌ FAIL |
| 2 | test_precond_no_wf | Missing kernel wf() | FAIL | ❌ FAIL |
| 3 | test_precond_root_process | depth == 0 (root) | FAIL | ❌ FAIL |
| 4 | test_precond_nonempty_children | Non-empty children | FAIL | ❌ FAIL |
| 5 | test_precond_nonempty_threads | Non-empty threads | FAIL | ❌ FAIL |

### Round 2: Overly Strong Postconditions

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_too_strong_proc_dom_unchanged | proc_dom unchanged (wrong) | FAIL | ❌ FAIL |
| 2 | test_too_strong_parent_children_same_count | Children count same (wrong) | FAIL | ❌ FAIL |
| 3 | test_too_strong_proc_still_in_dom | proc_ptr still in domain (wrong) | FAIL | ❌ FAIL |
| 4 | test_too_strong_thread_dom_changed | thread_dom changed (wrong) | FAIL | ❌ FAIL |
| 5 | test_too_strong_parent_children_decreased_by_two | Children -2 instead of -1 | FAIL | ❌ FAIL |

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_negate_wf_preserved | Negate kernel wf after op | FAIL | ❌ FAIL |
| 2 | test_negate_container_dom_preserved | Negate container_dom preserved | FAIL | ❌ FAIL |
| 3 | test_negate_thread_dom_preserved | Negate thread_dom preserved | FAIL | ❌ FAIL |
| 4 | test_negate_containers_tree_unchanged | Negate container tree preserved | FAIL | ❌ FAIL |
| 5 | test_negate_threads_unchanged | Negate threads preserved | FAIL | ❌ FAIL |

### Round 4: Wrong Specific Values

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_wrong_page_index2ptr_1 | page_index2ptr(1)==0 (wrong) | FAIL | ❌ FAIL |
| 2 | test_wrong_page_ptr2index_4096 | page_ptr2index(4096)==0 (wrong) | FAIL | ❌ FAIL |
| 3 | test_wrong_page_ptr_valid_unaligned | page_ptr_valid(1) (wrong) | FAIL | ❌ FAIL |
| 4 | test_wrong_page_index_valid_num_pages | index_valid(NUM_PAGES) (wrong) | FAIL | ❌ FAIL |
| 5 | test_wrong_page_index_2m_valid_1 | 2m_valid(1) (wrong) | FAIL | ❌ FAIL |
| 6 | test_wrong_truncate_2m | truncate_2m(513)==513 (wrong) | FAIL | ❌ FAIL |
| 7 | test_wrong_page_entry_empty_with_addr | Entry with addr=4096 empty (wrong) | FAIL | ❌ FAIL |
| 8 | test_wrong_roundtrip | ptr/index roundtrip 4096→8192 (wrong) | FAIL | ❌ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_removed_proc_in_domain | Removed proc still in domain + other not | FAIL | ❌ FAIL |
| 2 | test_fields_unchanged_implies_children | fields_unchanged implies children same | FAIL | ❌ FAIL |
| 3 | test_tree_unchanged_implies_owned_procs | tree_unchanged implies owned_procs same | FAIL | ❌ FAIL |
| 4 | test_tree_unchanged_implies_scheduler | tree_unchanged implies scheduler same | FAIL | ❌ FAIL |
| 5 | test_two_procs_removed | Two procs removed (only one should be) | FAIL | ❌ FAIL |

**Total: 28 completeness tests, all 28 correctly failed**

## Overall Assessment

- **Correctness**: ✅ All 34 correctness tests pass. The specs correctly describe valid behavior.
- **Completeness**: ✅ All 28 completeness tests fail as expected. The specs are tight enough to reject:
  - Precondition violations (missing wf, wrong proc state)
  - Overly strong claims (unchanged when should change, stronger bounds)
  - Negated postconditions
  - Wrong concrete values
  - Cross-function misuse (spec predicates don't cover unrelated fields)
- **Spec Gaps Found**: None. The specifications appear both correct and complete for the tested properties.
