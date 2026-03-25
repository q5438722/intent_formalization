# Adversarial Test Summary: `syscall_mmap`

## Target
`kernel__syscall_mmap__impl0__syscall_mmap.rs` — the `syscall_mmap` function and its specification (`syscall_mmap_spec`, `syscall_mmap_return_value`).

## Results

| Test File | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| `boundary_tests.rs` | 10 | 10 | 0 |
| `behavioral_mutation_tests.rs` | 12 | 12 | 0 |
| `logical_tests.rs` | 10 | 10 | 0 |
| **Total** | **32** | **32** | **0** |

All 32 adversarial tests **failed verification** as expected, meaning the specification correctly rejects all tested invalid inputs, incorrect behaviors, and unintended logical inferences.

---

## Boundary Tests (10 tests)

All tests violate preconditions or use edge-case values. All rejected.

| # | Test | Failure Mode |
|---|---|---|
| 1 | `thread_not_in_domain` | thread_ptr not in thread_dom |
| 2 | `va_range_len_overflow` | va_range.len * 4 ≥ usize::MAX |
| 3 | `va_range_start_overflow` | start + len * 4096 exceeds usize::MAX |
| 4 | `zero_len_va_range_with_zero_quota` | quota insufficient for allocation |
| 5 | `unaligned_page_ptr` | page pointer not 4K-aligned |
| 6 | `page_index_at_num_pages` | page index at NUM_PAGES (out of bounds) |
| 7 | `quota_exactly_insufficient` | quota = len*4 - 1 (just under threshold) |
| 8 | `non_4k_aligned_va` | VA fails 4K alignment mask check |
| 9 | `duplicate_va_in_range` | va_range contains duplicate addresses |
| 10 | `free_pages_less_than_quota` | free pages ≠ total quota (total_wf violation) |

## Behavioral Mutation Tests (12 tests)

All tests mutate postcondition relationships. All rejected.

| # | Test | Mutation |
|---|---|---|
| 1 | `error_path_kernel_changed` | On error, claim kernel state changed |
| 2 | `thread_dom_changed` | On success, claim new thread appeared |
| 3 | `proc_dom_shrank` | On success, claim a process disappeared |
| 4 | `endpoint_dom_changed` | On success, claim endpoint appeared |
| 5 | `other_proc_address_space_changed` | Claim unrelated proc's addr space changed |
| 6 | `mapped_va_not_in_domain` | Claim mapped VA not in new addr space |
| 7 | `new_pages_were_already_mapped` | Claim allocated pages were previously mapped |
| 8 | `outside_va_mapping_changed` | Claim VA outside range had mapping change |
| 9 | `return_has_pcid` | Claim return struct has Some pcid (not NoSwitch) |
| 10 | `other_container_changed` | Claim unrelated container changed |
| 11 | `page_mapping_dom_unchanged` | Claim page mapping domain didn't grow |
| 12 | `page_mapping_is_empty` | Claim new page maps to empty set |

## Logical Tests (10 tests)

All tests assert properties not guaranteed by the spec. All rejected.

| # | Test | Unwarranted Property |
|---|---|---|
| 1 | `determinism_of_mapped_pages` | Same inputs → same physical pages |
| 2 | `free_pages_decrease_exact` | Free pages decrease by exactly va_range.len |
| 3 | `allocated_pages_unique` | Allocated page pointers are distinct |
| 4 | `pages_contiguous` | Physical pages are contiguous |
| 5 | `quota_decrease_equals_va_range_len` | Quota decreases by exactly va_range.len |
| 6 | `ptr_index_roundtrip_without_bounds` | page_ptr / 4096 < NUM_PAGES without bounds check |
| 7 | `address_space_exact_composition` | New addr space is exactly old + va_range |
| 8 | `identify_conflicting_va` | Can identify which specific VA conflicts |
| 9 | `new_pages_zeroed` | Newly mapped pages have zero content |
| 10 | `pages_consumed_upper_bound` | num_page ≤ va_range_len (no page table overhead) |

## Conclusion

The `syscall_mmap` specification is **consistent** with respect to all 32 tested adversarial properties. The spec correctly:
- Rejects invalid inputs (boundary violations)
- Rejects incorrect output mutations (behavioral soundness)
- Does not entail unwarranted logical properties (no over-specification)
