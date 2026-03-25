# Adversarial Proof Test Summary

**Target**: `kernel__create_and_share_pages__impl0__range_create_and_share_mapping.rs`

**Functions tested**: `create_entry_and_share`, `range_create_and_share_mapping`

---

## Results Overview

| Category | Total | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary Tests | 12 | 12 ✅ | 0 |
| Behavioral Mutation Tests | 12 | 12 ✅ | 0 |
| Logical Tests | 12 | 12 ✅ | 0 |
| **Total** | **36** | **36** | **0** |

All 36 adversarial tests correctly **failed verification**, indicating the specification correctly rejects:
- Invalid inputs (boundary violations)
- Incorrect behaviors (mutated postconditions)
- Unintended logical inferences

---

## Boundary Tests (12/12 failed ✅)

| # | Test | Violated Precondition |
|---|---|---|
| 1 | `test_boundary_src_proc_not_in_domain` | `proc_dom().contains(src_proc_ptr)` |
| 2 | `test_boundary_target_proc_not_in_domain` | `proc_dom().contains(target_proc_ptr)` |
| 3 | `test_boundary_same_src_and_target_proc` | `src_proc_ptr != target_proc_ptr` |
| 4 | `test_boundary_insufficient_quota` | `quota.mem_4k >= 3 * range.len` |
| 5 | `test_boundary_insufficient_free_pages` | `free_pages >= 3 * range.len` |
| 6 | `test_boundary_range_length_mismatch` | `src_va_range.len == target_va_range.len` |
| 7 | `test_boundary_src_va_zero` | `va_4k_valid(src_va)` |
| 8 | `test_boundary_target_va_not_aligned` | `va_4k_valid(target_va)` |
| 9 | `test_boundary_target_va_already_mapped` | `addr_space.contains(target_va) == false` |
| 10 | `test_boundary_src_va_not_in_address_space` | `addr_space.contains(src_va) == true` |
| 11 | `test_boundary_ref_counter_at_max` | `ref_counter <= usize::MAX - 1` |
| 12 | `test_boundary_ref_counter_overflow_for_range` | `ref_counter <= usize::MAX - range.len` |

## Behavioral Mutation Tests (12/12 failed ✅)

| # | Test | Mutated Postcondition |
|---|---|---|
| 1 | `test_mutation_proc_dom_shrinks` | proc_dom should be unchanged |
| 2 | `test_mutation_thread_dom_changes` | thread_dom should be unchanged |
| 3 | `test_mutation_free_pages_increase` | free_pages should decrease |
| 4 | `test_mutation_ret_exceeds_bound` | ret should be ≤ 3 |
| 5 | `test_mutation_target_addr_space_loses_old_entry` | old entries should be preserved |
| 6 | `test_mutation_other_proc_addr_space_changed` | non-target proc space unchanged |
| 7 | `test_mutation_endpoint_dom_changes` | endpoint_dom should be unchanged |
| 8 | `test_mutation_ref_counter_wrong_increment` | ref counter should increment by 1 |
| 9 | `test_mutation_shared_entry_addr_differs` | shared entry should match source |
| 10 | `test_mutation_container_dom_changes` | container_dom should be unchanged |
| 11 | `test_mutation_page_mapping_dom_changes` | page_mapping domain unchanged |
| 12 | `test_mutation_quota_unchanged` | quota should subtract by ret |

## Logical Tests (12/12 failed ✅)

| # | Test | Unentailed Property |
|---|---|---|
| 1 | `test_logical_ret_is_deterministic` | determinism of return value |
| 2 | `test_logical_ret_always_positive` | ret > 0 (could be 0) |
| 3 | `test_logical_ret_equals_max` | ret == 3 exactly |
| 4 | `test_logical_free_pages_decrease_exact` | exact 3-page decrease per entry |
| 5 | `test_logical_zero_length_forces_ret_zero` | zero-length ⇒ ret nonzero |
| 6 | `test_logical_new_page_becomes_mapped` | new page becomes mapped |
| 7 | `test_logical_src_page_mapping_unchanged` | src page mapping unchanged |
| 8 | `test_logical_target_space_exact_size` | target space size == range_len |
| 9 | `test_logical_unrelated_container_quota_unchanged` | quota unchanged after call |
| 10 | `test_logical_partial_mapping_implies_full` | partial mapping ⇒ full mapping |
| 11 | `test_logical_all_src_pages_shared` | all src pages shared to target |
| 12 | `test_logical_all_page_mapping_values_unchanged` | all mapping values unchanged |

---

## Conclusion

The specification for `range_create_and_share_mapping` and `create_entry_and_share` appears **well-formed and sufficiently strong**: it correctly rejects all 36 adversarial queries across boundary, behavioral, and logical categories. No spec weaknesses were detected.
