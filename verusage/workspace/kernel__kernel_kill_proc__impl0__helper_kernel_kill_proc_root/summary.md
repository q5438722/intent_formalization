# Test Summary: `helper_kernel_kill_proc_root`

## Target Function
`Kernel::helper_kernel_kill_proc_root(&mut self, proc_ptr: ProcPtr)` — kills a root process (depth == 0) by removing it from the process domain, freeing its page table, and returning a freed page.

## Results Overview

| Test Category | Total | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary Tests | 8 | 8 ✅ | 0 |
| Behavioral Mutation Tests | 8 | 8 ✅ | 0 |
| Logical Tests | 8 | 8 ✅ | 0 |
| **Total** | **24** | **24 ✅** | **0** |

All 24 adversarial tests were correctly **rejected** by Verus, meaning the specification does not entail any of the tested undesirable properties.

---

## Boundary Tests (`boundary_tests.rs`)

All 8 tests violate preconditions and correctly fail:

| # | Test | Violated Precondition |
|---|---|---|
| 1 | `test_boundary_proc_not_in_domain` | `proc_dom().contains(proc_ptr)` |
| 2 | `test_boundary_depth_nonzero` | `depth == 0` |
| 3 | `test_boundary_children_not_empty` | `children@ == Seq::empty()` |
| 4 | `test_boundary_threads_not_empty` | `owned_threads@ == Seq::empty()` |
| 5 | `test_boundary_ioid_is_some` | `ioid.is_None()` |
| 6 | `test_boundary_pagetable_not_empty` | Pagetable `is_empty()` |
| 7 | `test_boundary_empty_proc_dom` | Empty domain edge case |
| 8 | `test_boundary_depth_max` | `depth == usize::MAX` (not 0) |

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

All 8 tests mutate expected postconditions and correctly fail:

| # | Test | Mutated Postcondition |
|---|---|---|
| 1 | `test_mutation_proc_dom_unchanged` | proc_dom should shrink, not stay same |
| 2 | `test_mutation_thread_dom_shrinks` | thread_dom should stay same, not shrink |
| 3 | `test_mutation_container_dom_changed` | container_dom should stay same |
| 4 | `test_mutation_pcid_changed` | pcid should be preserved for remaining procs |
| 5 | `test_mutation_children_gains_child` | children should be preserved, not grow |
| 6 | `test_mutation_uppertree_seq_changed` | uppertree_seq should be preserved |
| 7 | `test_mutation_subtree_set_unchanged` | subtree_set should remove proc_ptr |
| 8 | `test_mutation_depth_changed` | depth should be preserved for remaining procs |

---

## Logical Tests (`logical_tests.rs`)

All 8 tests encode properties NOT guaranteed by the spec and correctly fail:

| # | Test | Unintended Property Tested |
|---|---|---|
| 1 | `test_logical_page_closure_size_unchanged` | Page closure cardinality preserved (not guaranteed) |
| 2 | `test_logical_proc_reuse_after_kill` | Killed proc_ptr reusable (set size unchanged after insert) |
| 3 | `test_logical_container_owned_procs_unchanged` | containers_tree_unchanged covers owned_procs (it doesn't) |
| 4 | `test_logical_deterministic_freed_page` | Freed page is deterministically 0 |
| 5 | `test_logical_thread_value_determinism` | Thread values are universally equal |
| 6 | `test_logical_proc_dom_empty_after_kill` | proc_dom becomes empty after kill |
| 7 | `test_logical_killed_pcid_still_active` | Killed pcid remains active |
| 8 | `test_logical_root_parent_must_be_none` | Root process parent must be None |

---

## Conclusion

The specification for `helper_kernel_kill_proc_root` correctly rejects all 24 adversarial queries across boundary violations, behavioral mutations, and logical over-claims. No weaknesses were detected in the tested semantic space.
