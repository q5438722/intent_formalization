# Test Summary: `process_manager__impl_new_container__impl0__new_container_with_endpoint`

## Target Function
`ProcessManager::new_container_with_endpoint` — Creates a new child container with a process, thread, and endpoint reference, consuming 3 page pointers and quota from the parent container.

## Results Overview

| Test Category          | Total Tests | Failed (Expected) | Passed (Unexpected) |
|------------------------|:-----------:|:------------------:|:-------------------:|
| Boundary Tests         |     10      |        10          |         0           |
| Behavioral Mutation    |     12      |        12          |         0           |
| Logical Tests          |     12      |        12          |         0           |
| **Total**              |   **34**    |      **34**        |       **0**         |

All 34 adversarial tests correctly **FAIL** verification, indicating the specification rejects each invalid property as intended.

---

## Boundary Tests (10/10 FAIL ✓)

| # | Test Name | Targeted Precondition |
|---|-----------|----------------------|
| 1 | `test_boundary_thread_ptr_not_in_dom` | `thread_dom().contains(thread_ptr)` |
| 2 | `test_boundary_endpoint_index_out_of_range` | `endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS` |
| 3 | `test_boundary_container_depth_max` | `depth != usize::MAX` |
| 4 | `test_boundary_children_list_full` | `children.len() < CONTAINER_CHILD_LIST_LEN` |
| 5 | `test_boundary_insufficient_mem_4k_quota` | `quota.mem_4k - 3 >= new_quota.mem_4k` |
| 6 | `test_boundary_page_ptrs_not_distinct` | `page_ptr_1 != page_ptr_2` |
| 7 | `test_boundary_page_ptr_already_in_closure` | `page_closure.contains(page_ptr_1) == false` |
| 8 | `test_boundary_quota_not_greater` | `parent.quota.spec_greater(new_quota)` |
| 9 | `test_boundary_pcid_collision` | `forall p_ptr: get_proc(p_ptr).pcid != new_pcid` |
| 10 | `test_boundary_page_ptr_2_equals_3` | `page_ptr_2 != page_ptr_3` |

## Behavioral Mutation Tests (12/12 FAIL ✓)

| # | Test Name | Mutated Postcondition |
|---|-----------|----------------------|
| 1 | `test_mutation_new_container_children_not_empty` | New container children = empty |
| 2 | `test_mutation_new_container_no_owned_proc` | New container owns page_ptr_2 |
| 3 | `test_mutation_proc_wrong_owning_container` | New proc's owning_container = page_ptr_1 |
| 4 | `test_mutation_parent_children_not_updated` | Parent children grows by push(page_ptr_1) |
| 5 | `test_mutation_thread_wrong_owning_container` | New thread's owning_container = page_ptr_1 |
| 6 | `test_mutation_proc_wrong_pcid` | New proc's pcid = new_pcid |
| 7 | `test_mutation_wrong_mem_4k_deduction` | mem_4k deduction = old - 3 - new_quota |
| 8 | `test_mutation_proc_ioid_not_none` | New proc's ioid is None |
| 9 | `test_mutation_endpoint_dom_changed` | endpoint_dom unchanged |
| 10 | `test_mutation_new_container_wrong_quota` | New container quota = new_quota |
| 11 | `test_mutation_scheduler_empty` | New container scheduler has page_ptr_3 |
| 12 | `test_mutation_endpoint_owning_threads_unchanged` | Endpoint owning_threads updated with (page_ptr_3, 0) |

## Logical Tests (12/12 FAIL ✓)

| # | Test Name | Unwarranted Property |
|---|-----------|---------------------|
| 1 | `test_logical_depth_bounded_small` | Depth ≤ 10 (not bounded by spec) |
| 2 | `test_logical_container_already_in_subtree` | page_ptr_1 already in parent subtree |
| 3 | `test_logical_container_proc_dom_overlap` | Arbitrary container/proc dom disjointness |
| 4 | `test_logical_new_thread_in_old_dom` | page_ptr_3 in old thread_dom |
| 5 | `test_logical_new_proc_multiple_threads` | New proc has > 1 thread |
| 6 | `test_logical_mem_2m_extra_deduction` | mem_2m deducted by 3 (like mem_4k) |
| 7 | `test_logical_all_endpoint_descriptors_filled` | All endpoint descriptor slots filled |
| 8 | `test_logical_new_container_has_cpus` | New container has CPUs |
| 9 | `test_logical_parent_owned_procs_changed` | Parent's owned_procs changed |
| 10 | `test_logical_new_container_subtree_not_empty` | New container subtree non-empty |
| 11 | `test_logical_new_container_parent_none` | New container parent is None |
| 12 | `test_logical_new_container_equals_parent` | page_ptr_1 == parent_container |

## Conclusion

The specification for `new_container_with_endpoint` is **consistent** with respect to all 34 adversarial queries:
- **Boundary**: All preconditions correctly reject invalid inputs.
- **Behavioral**: All postconditions correctly reject mutated/incorrect behaviors.
- **Logical**: No unintended properties are entailed by the specification.

No specification weaknesses were detected in these tests.
