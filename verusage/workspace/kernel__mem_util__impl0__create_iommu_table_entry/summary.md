# Adversarial Proof Test Summary: `create_iommu_table_entry`

## Target Function
`Kernel::create_iommu_table_entry(&mut self, proc_ptr: ProcPtr, va: VAddr) -> (usize, PageMapPtr)`

Creates intermediate IOMMU page table levels (L4, L3, L2) for a given process and virtual address. Returns the number of newly allocated pages (0–3) and the L2 page map pointer.

---

## Results Overview

| Test Category         | Total | Failed (Expected) | Passed (Unexpected) |
|----------------------|-------|--------------------|---------------------|
| Boundary Tests       | 10    | 10                 | 0                   |
| Behavioral Mutation  | 10    | 10                 | 0                   |
| Logical Tests        | 10    | 10                 | 0                   |
| **Total**            | **30**| **30**             | **0**               |

**All 30 tests failed verification as expected.** The specification correctly rejects all adversarial queries.

---

## Boundary Tests (`boundary_tests.rs`)

All tests violate preconditions of `create_iommu_table_entry`. All failed ✅

| # | Test | Violated Precondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_boundary_proc_not_in_domain` | `proc_dom().contains(proc_ptr)` | FAIL ✅ |
| 2 | `test_boundary_no_iommu_table` | `get_proc_has_iommu_table(proc_ptr)` (ioid.is_Some) | FAIL ✅ |
| 3 | `test_boundary_insufficient_quota` | `quota.mem_4k >= 3` (quota=2) | FAIL ✅ |
| 4 | `test_boundary_insufficient_free_pages` | `get_num_of_free_pages() >= 3` (free=2) | FAIL ✅ |
| 5 | `test_boundary_va_zero_not_valid` | `va_4k_valid(va)` (va=0, kernel space) | FAIL ✅ |
| 6 | `test_boundary_va_already_in_io_space` | `get_io_space().dom().contains(va) == false` | FAIL ✅ |
| 7 | `test_boundary_zero_quota` | `quota.mem_4k >= 3` (quota=0) | FAIL ✅ |
| 8 | `test_boundary_zero_free_pages` | `get_num_of_free_pages() >= 3` (free=0) | FAIL ✅ |
| 9 | `test_boundary_va_misaligned` | `va_4k_valid(va)` (va=1, not 4k-aligned) | FAIL ✅ |
| 10 | `test_boundary_quota_one` | `quota.mem_4k >= 3` (quota=1) | FAIL ✅ |

---

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)

All tests start from valid postcondition values and mutate expected relations. All failed ✅

| # | Test | Mutated Postcondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_behavioral_ret_exceeds_upper_bound` | `ret.0 <= 3` → claim ret.0 == 4 | FAIL ✅ |
| 2 | `test_behavioral_wrong_free_page_decrease` | free pages decrease by ret.0+1 instead of ret.0 | FAIL ✅ |
| 3 | `test_behavioral_proc_dom_changed` | proc_dom preserved → claim new proc added | FAIL ✅ |
| 4 | `test_behavioral_thread_dom_changed` | thread_dom preserved → claim thread removed | FAIL ✅ |
| 5 | `test_behavioral_container_dom_changed` | container_dom preserved → claim container added | FAIL ✅ |
| 6 | `test_behavioral_endpoint_dom_changed` | endpoint_dom preserved → claim endpoint removed | FAIL ✅ |
| 7 | `test_behavioral_quota_wrong_subtraction` | quota subtract by ret.0 → subtract by ret.0+1 | FAIL ✅ |
| 8 | `test_behavioral_page_mapping_changed` | page_mapping preserved → claim new entry added | FAIL ✅ |
| 9 | `test_behavioral_io_space_changed` | IO space preserved → claim new va in IO space | FAIL ✅ |
| 10 | `test_behavioral_container_owned_pages_changed` | owned pages preserved → claim new page added | FAIL ✅ |

---

## Logical Tests (`logical_tests.rs`)

All tests encode properties NOT explicitly guaranteed by the specification. All failed ✅

| # | Test | Unguaranteed Property | Result |
|---|------|-----------------------|--------|
| 1 | `test_logical_always_allocates_at_least_one` | ret.0 >= 1 (spec only says <= 3) | FAIL ✅ |
| 2 | `test_logical_always_allocates_exactly_three` | ret.0 == 3 always | FAIL ✅ |
| 3 | `test_logical_determinism` | Same constraints → same ret.0 | FAIL ✅ |
| 4 | `test_logical_free_pages_always_decrease_by_3` | Free pages always decrease by exactly 3 | FAIL ✅ |
| 5 | `test_logical_ret_strictly_less_than_3` | ret.0 < 3 always (never 3) | FAIL ✅ |
| 6 | `test_logical_va_added_to_io_space` | VA gets added to IO space after call | FAIL ✅ |
| 7 | `test_logical_owned_pages_grow` | Container owned pages grow | FAIL ✅ |
| 8 | `test_logical_quota_always_decreases` | Quota always strictly decreases (ret.0 > 0) | FAIL ✅ |
| 9 | `test_logical_different_va_different_ret1` | Different VAs → different returned PageMapPtr | FAIL ✅ |
| 10 | `test_logical_ret_is_even` | ret.0 is always even | FAIL ✅ |

---

## Conclusion

The specification for `create_iommu_table_entry` correctly rejects all 30 adversarial queries across three categories:
- **Boundary**: All invalid inputs are properly rejected by preconditions.
- **Behavioral**: All mutated output relations are properly rejected by postconditions.
- **Logical**: All unguaranteed stronger properties are correctly not entailed.

The specification appears **consistent** with respect to the tested semantic boundary — it neither admits invalid inputs, nor allows incorrect behavioral inferences, nor entails unintended logical properties.
