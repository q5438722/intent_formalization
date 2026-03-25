# Adversarial Proof Test Summary

## Target: `create_entry_and_share` (kernel page sharing)

This function creates a page table entry for a target process and shares a mapping from a source process's address space into the target's address space.

---

## Results Overview

| Test Category | Total | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary Tests | 12 | 12 ✅ | 0 |
| Behavioral Mutation Tests | 12 | 12 ✅ | 0 |
| Logical Tests | 12 | 12 ✅ | 0 |
| **Total** | **36** | **36** | **0** |

**All 36 tests failed verification as expected.** No specification weaknesses were detected.

---

## Boundary Tests (`boundary_tests.rs`)

All 12 tests violate preconditions of `create_entry_and_share` and correctly fail:

| # | Test | Precondition Violated |
|---|---|---|
| 1 | `test_boundary_zero_free_pages` | `free_pages >= 3` (free_pages=0) |
| 2 | `test_boundary_two_free_pages` | `free_pages >= 3` (free_pages=2) |
| 3 | `test_boundary_zero_quota` | `quota.mem_4k >= 3` (mem_4k=0) |
| 4 | `test_boundary_quota_two` | `quota.mem_4k >= 3` (mem_4k=2) |
| 5 | `test_boundary_src_proc_not_in_domain` | `proc_dom.contains(src_proc_ptr)` |
| 6 | `test_boundary_target_proc_not_in_domain` | `proc_dom.contains(target_proc_ptr)` |
| 7 | `test_boundary_src_va_zero_not_valid` | `va_4k_valid(src_va)` (va=0) |
| 8 | `test_boundary_target_va_not_aligned` | `va_4k_valid(target_va)` (va=1) |
| 9 | `test_boundary_target_va_already_mapped` | `target_va NOT in address space` |
| 10 | `test_boundary_src_va_not_mapped` | `src_va IS in address space` |
| 11 | `test_boundary_ref_counter_at_max` | `ref_counter <= usize::MAX - 1` |
| 12 | `test_boundary_ret_exceeds_three` | `ret <= 3` (postcondition boundary) |

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

All 12 tests mutate correct postconditions and correctly fail:

| # | Test | Mutation Applied |
|---|---|---|
| 1 | `test_mutation_free_pages_unchanged` | Free pages stays same instead of decreasing |
| 2 | `test_mutation_target_va_not_in_new_space` | target_va absent from new address space |
| 3 | `test_mutation_shared_entry_addr_differs` | Shared entry addr differs from source |
| 4 | `test_mutation_quota_unchanged_after_share` | Quota mem_4k unchanged after subtraction |
| 5 | `test_mutation_other_proc_space_changes` | Non-target proc's address space changes |
| 6 | `test_mutation_ref_counter_increases_by_two` | Ref counter +2 instead of +1 |
| 7 | `test_mutation_other_page_ref_counter_changes` | Other page ref counters change |
| 8 | `test_mutation_page_mapping_unchanged` | page_mapping for src page unchanged |
| 9 | `test_mutation_page_mapping_domain_grows` | page_mapping domain gains new entry |
| 10 | `test_mutation_proc_dom_grows` | proc_dom gains new process |
| 11 | `test_mutation_page_mapped_status_flips` | page_is_mapped status flips |
| 12 | `test_mutation_container_owned_pages_change` | Container owned pages change |

---

## Logical Tests (`logical_tests.rs`)

All 12 tests assert unguaranteed properties and correctly fail:

| # | Test | Unguaranteed Property |
|---|---|---|
| 1 | `test_logical_ret_always_positive` | ret >= 1 (spec allows ret=0) |
| 2 | `test_logical_ret_always_exactly_three` | ret == 3 (spec only says ret <= 3) |
| 3 | `test_logical_determinism` | Same inputs → same ret |
| 4 | `test_logical_root_process_preserved` | Container root_process unchanged |
| 5 | `test_logical_src_target_always_different` | src_proc != target_proc always |
| 6 | `test_logical_va_always_different` | src_va != target_va always |
| 7 | `test_logical_quota_positive_after` | quota.mem_4k > 0 after call |
| 8 | `test_logical_io_mapping_domain_unchanged` | page_io_mapping domain unchanged |
| 9 | `test_logical_explicit_mapped_count_unchanged` | Explicit count of mapped pages unchanged |
| 10 | `test_logical_can_have_children_preserved` | Container can_have_children preserved |
| 11 | `test_logical_va_valid_implies_page_ptr_valid` | VA validity ⇒ PA validity |
| 12 | `test_logical_src_page_mapping_size_exactly_two` | Src page mapping has exactly 2 entries |

---

## Conclusion

The specification for `create_entry_and_share` is **well-formed** against all 36 adversarial queries:

- **Boundary constraints** are tight — invalid inputs (insufficient pages, quotas, unmapped VAs, overflow ref counters) are all rejected.
- **Behavioral mutations** are caught — incorrect output relations (unchanged free pages, wrong ref counter delta, missing VA insertions) are all rejected.
- **Logical overreach** is avoided — the spec does not inadvertently entail determinism, stronger bounds, cross-domain validity, or unspecified field preservation.

No specification weaknesses were identified in this testing round.
