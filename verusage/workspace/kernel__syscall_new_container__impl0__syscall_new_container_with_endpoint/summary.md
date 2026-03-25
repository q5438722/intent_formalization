# Adversarial Test Summary: `syscall_new_container_with_endpoint`

## Target
`kernel__syscall_new_container__impl0__syscall_new_container_with_endpoint.rs`

Functions tested: `syscall_new_container_with_endpoint`, `new_container_with_endpoint`, and related helpers.

---

## Results Overview

| Test File | Tests | Passed (verified) | Failed (verification error) | Expected Failures |
|-----------|-------|--------------------|-----------------------------|-------------------|
| `boundary_tests.rs` | 10 | 0 | 10 | 10 |
| `behavioral_mutation_tests.rs` | 10 | 0 | 10 | 10 |
| `logical_tests.rs` | 10 | 0 | 10 | 10 |
| **Total** | **30** | **0** | **30** | **30** |

✅ **All 30 adversarial tests correctly FAILED verification**, meaning the specification properly rejects all tested invalid properties.

---

## Boundary Tests (10/10 failed ✅)

| # | Test | Violated Precondition |
|---|------|-----------------------|
| 1 | `test_boundary_thread_not_in_domain` | `thread_dom().contains(thread_ptr)` |
| 2 | `test_boundary_endpoint_index_at_max` | `0 <= endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS` |
| 3 | `test_boundary_insufficient_mem_4k_quota` | `quota.mem_4k >= 3 + init_quota.mem_4k` |
| 4 | `test_boundary_depth_at_max` | `depth != usize::MAX` |
| 5 | `test_boundary_children_list_full` | `children.len() < CONTAINER_CHILD_LIST_LEN` |
| 6 | `test_boundary_duplicate_page_ptrs` | `page_ptr_1 != page_ptr_2` |
| 7 | `test_boundary_mem_4k_overflow` | `3 + init_quota.mem_4k < usize::MAX` |
| 8 | `test_boundary_zero_pcid_quota` | `quota.pcid >= 1 + init_quota.pcid` |
| 9 | `test_boundary_va_range_len_overflow` | `va_range.len * 3 < usize::MAX` |
| 10 | `test_boundary_init_quota_less_than_3x_va_range` | `init_quota.mem_4k >= 3 * va_range.len` |

---

## Behavioral Mutation Tests (10/10 failed ✅)

| # | Test | Mutated Behavior |
|---|------|-----------------|
| 1 | `test_mutation_page_closure_missing_one` | Claimed only 2 pages added (should be 3) |
| 2 | `test_mutation_wrong_proc_ptr_added` | Claimed wrong pointer added to proc_dom |
| 3 | `test_mutation_new_container_nonempty_children` | Claimed new container has non-empty children |
| 4 | `test_mutation_wrong_quota_deduction` | Claimed only 2 mem_4k deducted (should be 3 + init) |
| 5 | `test_mutation_children_prepend_instead_of_append` | Claimed child was prepended not appended |
| 6 | `test_mutation_endpoint_at_wrong_index` | Claimed endpoint at index 1 (should be 0) |
| 7 | `test_mutation_proc_wrong_owning_container` | Claimed proc belongs to parent container |
| 8 | `test_mutation_endpoint_dom_changed` | Claimed endpoint_dom grew (should be unchanged) |
| 9 | `test_mutation_wrong_depth` | Claimed depth = parent + 2 (should be parent + 1) |
| 10 | `test_mutation_old_thread_changed` | Claimed old thread's owning_container changed |

---

## Logical Tests (10/10 failed ✅)

| # | Test | Unintended Property Tested |
|---|------|---------------------------|
| 1 | `test_logical_determinism_container_ptr` | Determinism of allocated container pointer |
| 2 | `test_logical_stronger_quota_inequality` | New container mem_4k strictly greater than init |
| 3 | `test_logical_new_proc_same_pcid_as_caller` | New process reuses caller's pcid |
| 4 | `test_logical_ptr_ordering` | Ordering among allocated page pointers |
| 5 | `test_logical_new_container_multiple_threads` | New container has ≥2 threads |
| 6 | `test_logical_parent_container_fully_preserved` | Parent container fully unchanged |
| 7 | `test_logical_endpoint_owning_threads_unchanged` | Endpoint's owning_threads set unchanged |
| 8 | `test_logical_page_ptr_alignment` | Allocated page pointer is page-aligned |
| 9 | `test_logical_all_containers_no_cpus` | All containers have empty owned_cpus |
| 10 | `test_logical_requirement_false_but_success` | Syscall succeeds when requirement is false |

---

## Conclusion

The specification for `syscall_new_container_with_endpoint` correctly rejects:
- **Invalid inputs** (boundary violations of all preconditions)
- **Incorrect behaviors** (mutated postcondition relations)
- **Unintended reasoning** (determinism, stronger inequalities, structural assumptions, cross-function misuse)

No specification weaknesses were detected in this test campaign.
