# Adversarial Test Results: `helper_kernel_kill_proc_non_root`

## Target Function
`Kernel::helper_kernel_kill_proc_non_root(&mut self, proc_ptr: ProcPtr)` — kills a non-root process, removing it from the process domain, updating the parent's children list, and freeing associated pages/page tables.

---

## Summary

| Test Category          | Total | Failed (Expected) | Passed (Unexpected) |
|------------------------|-------|--------------------|---------------------|
| Boundary Tests         | 8     | 8 ✅               | 0                   |
| Behavioral Mutation    | 10    | 10 ✅              | 0                   |
| Logical Tests          | 10    | 10 ✅              | 0                   |
| **Total**              | **28**| **28 ✅**          | **0**               |

**All 28 adversarial tests failed verification as expected.** The specification correctly rejects all invalid inputs, incorrect behaviors, and unintended reasoning tested.

---

## Boundary Tests (8/8 failed ✅)

| # | Test | Violated Precondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_boundary_proc_not_in_domain` | `proc_dom().contains(proc_ptr)` | FAIL ✅ |
| 2 | `test_boundary_depth_is_zero` | `depth != 0` | FAIL ✅ |
| 3 | `test_boundary_children_not_empty` | `children@ == Seq::empty()` | FAIL ✅ |
| 4 | `test_boundary_threads_not_empty` | `owned_threads@ == Seq::empty()` | FAIL ✅ |
| 5 | `test_boundary_ioid_is_some` | `ioid.is_None()` | FAIL ✅ |
| 6 | `test_boundary_parent_is_none` | Implicit: `parent.is_Some()` for non-root | FAIL ✅ |
| 7 | `test_boundary_parent_not_in_domain` | Parent must be in proc_dom | FAIL ✅ |
| 8 | `test_boundary_page_ptr_not_aligned` | `ptr % 0x1000 == 0` | FAIL ✅ |

## Behavioral Mutation Tests (10/10 failed ✅)

| # | Test | Mutated Postcondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_mutation_proc_dom_unchanged` | proc_dom should shrink by 1 | FAIL ✅ |
| 2 | `test_mutation_thread_dom_changed` | thread_dom should be unchanged | FAIL ✅ |
| 3 | `test_mutation_container_dom_changed` | container_dom should be unchanged | FAIL ✅ |
| 4 | `test_mutation_parent_children_still_has_proc` | Parent's children should remove proc_ptr | FAIL ✅ |
| 5 | `test_mutation_parent_children_len_unchanged` | Parent's children.len() should decrease by 1 | FAIL ✅ |
| 6 | `test_mutation_subtree_set_unchanged` | Uppertree subtree_set should remove proc_ptr | FAIL ✅ |
| 7 | `test_mutation_uppertree_seq_changed` | uppertree_seq should be preserved | FAIL ✅ |
| 8 | `test_mutation_non_parent_children_changed` | Non-parent procs' children should be unchanged | FAIL ✅ |
| 9 | `test_mutation_wf_false_after_kill` | wf() should hold after kill | FAIL ✅ |
| 10 | `test_mutation_pcid_changed` | pcid should be preserved for remaining procs | FAIL ✅ |

## Logical Tests (10/10 failed ✅)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_page_closure_size_unchanged` | page_closure size unchanged (wrong) | FAIL ✅ |
| 2 | `test_logical_proc_reuse_after_kill` | proc_ptr reusable immediately | FAIL ✅ |
| 3 | `test_logical_container_owned_procs_unchanged` | containers_tree_unchanged covers owned_procs | FAIL ✅ |
| 4 | `test_logical_free_page_determinism` | Freed page is deterministic | FAIL ✅ |
| 5 | `test_logical_proc_dom_empty_after_kill` | proc_dom empties after one kill | FAIL ✅ |
| 6 | `test_logical_all_pagetables_none` | All pagetables deactivated | FAIL ✅ |
| 7 | `test_logical_proc_dom_ordering` | proc_dom has ordering properties | FAIL ✅ |
| 8 | `test_logical_freed_page_always_zero` | Freed page always at address 0 | FAIL ✅ |
| 9 | `test_logical_thread_count_zero` | Thread count becomes 0 | FAIL ✅ |
| 10 | `test_logical_depth_bounded` | Depth upper-bounded by 1 | FAIL ✅ |

---

## Conclusion

The specification for `helper_kernel_kill_proc_non_root` is **consistent** with respect to all 28 adversarial queries tested. The spec correctly:

1. **Rejects invalid inputs** — all precondition violations are caught
2. **Rejects incorrect behaviors** — mutated postconditions are not entailed
3. **Rejects unintended reasoning** — non-guaranteed properties (determinism, stronger bounds, cross-function assumptions) are not derivable

No specification weaknesses were identified in this test suite.
