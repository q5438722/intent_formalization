# Test Summary: `syscall_new_container_with_endpoint`

## Overview

30 adversarial proof tests were generated across three categories to probe the semantic boundary of the `syscall_new_container_with_endpoint` specification. **All 30 tests correctly FAILED verification**, confirming the spec rejects the tested invalid properties.

## Results

| Category | Tests | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary Tests | 10 | 10 ✅ | 0 |
| Behavioral Mutation Tests | 10 | 10 ✅ | 0 |
| Logical Tests | 10 | 10 ✅ | 0 |
| **Total** | **30** | **30** | **0** |

## Boundary Tests (10/10 failed ✅)

| # | Test | Property Violated |
|---|---|---|
| 1 | `test_boundary_thread_ptr_not_in_dom` | `thread_ptr` not in `thread_dom` |
| 2 | `test_boundary_endpoint_index_out_of_range` | `endpoint_index >= MAX_NUM_ENDPOINT_DESCRIPTORS` |
| 3 | `test_boundary_container_depth_max` | Container depth == `usize::MAX` |
| 4 | `test_boundary_children_list_full` | Children list at capacity (`>= CONTAINER_CHILD_LIST_LEN`) |
| 5 | `test_boundary_insufficient_mem_4k_quota` | `mem_4k < 3 + init_quota.mem_4k` |
| 6 | `test_boundary_insufficient_pcid_quota` | `pcid < 1 + init_quota.pcid` |
| 7 | `test_boundary_no_free_pages` | Zero free pages available |
| 8 | `test_boundary_page_ptrs_not_distinct` | `page_ptr_1 == page_ptr_2` |
| 9 | `test_boundary_init_quota_less_than_va_range_cost` | `init_quota.mem_4k < 3 * va_range.len` |
| 10 | `test_boundary_pcid_exhausted` | PCID pool exhausted (`free_pcids.len() == 0`) |

## Behavioral Mutation Tests (10/10 failed ✅)

| # | Test | Mutated Property |
|---|---|---|
| 1 | `test_mutation_new_container_children_not_empty` | New container has non-empty children |
| 2 | `test_mutation_new_container_no_owned_proc` | New container's `owned_procs` is empty |
| 3 | `test_mutation_proc_wrong_owning_container` | New process's `owning_container` is wrong |
| 4 | `test_mutation_parent_children_not_updated` | Parent's children list unchanged after insert |
| 5 | `test_mutation_thread_wrong_owning_container` | New thread assigned to wrong container |
| 6 | `test_mutation_proc_wrong_pcid` | New process has incorrect `pcid` |
| 7 | `test_mutation_new_container_wrong_quota` | New container's `quota.mem_4k` is wrong |
| 8 | `test_mutation_wrong_mem_4k_deduction` | Parent `mem_4k` deduction off by 1 (2 vs 3) |
| 9 | `test_mutation_proc_ioid_not_none` | New process's `ioid` is `Some` (should be `None`) |
| 10 | `test_mutation_endpoint_dom_changed` | Endpoint domain gained spurious entry |

## Logical Tests (10/10 failed ✅)

| # | Test | Unwarranted Property |
|---|---|---|
| 1 | `test_logical_determinism_of_allocation` | Page allocation is deterministic |
| 2 | `test_logical_depth_bounded_small` | Container depth bounded by small constant |
| 3 | `test_logical_container_already_in_subtree` | New container pre-exists in parent's subtree |
| 4 | `test_logical_container_proc_dom_disjoint` | Container and process domains are always disjoint |
| 5 | `test_logical_parent_container_unchanged` | Parent container is unchanged after operation |
| 6 | `test_logical_new_thread_in_old_dom` | New thread was already in old thread domain |
| 7 | `test_logical_new_proc_multiple_threads` | New process has more than 1 thread |
| 8 | `test_logical_mem_2m_extra_deduction` | `mem_2m` deduction includes extra 3 (like `mem_4k`) |
| 9 | `test_logical_all_endpoint_descriptors_filled` | All endpoint descriptor slots are filled |
| 10 | `test_logical_new_container_has_cpus` | New container has non-empty `owned_cpus` |

## Conclusion

The specification for `syscall_new_container_with_endpoint` correctly rejects all 30 adversarial properties. The spec demonstrates adequate boundary enforcement (precondition checks), behavioral precision (correct postcondition semantics), and resistance to unwarranted logical inferences.
