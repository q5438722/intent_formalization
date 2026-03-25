# Adversarial Proof Test Summary

**Target**: `kernel__create_and_map_pages__impl0__alloc_and_map_io.rs` — `Kernel::alloc_and_map_io`

## Results Overview

| Test File | Total Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| `boundary_tests.rs` | 7 | 7 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 7 | 7 ✅ | 0 |
| `logical_tests.rs` | 7 | 7 ✅ | 0 |
| **Total** | **21** | **21** | **0** |

All 21 tests correctly **failed verification**, indicating the specification rejects all tested invalid inputs, incorrect behaviors, and unintended logical properties.

---

## Boundary Tests (7/7 FAIL ✅)

| # | Test | Violated Precondition |
|---|---|---|
| 1 | `test_boundary_no_wf` | Missing `kernel.wf()` |
| 2 | `test_boundary_proc_not_in_dom` | `proc_ptr ∉ proc_dom` |
| 3 | `test_boundary_no_iommu_table` | `ioid.is_None()` (no IOMMU table) |
| 4 | `test_boundary_zero_quota` | `mem_4k == 0` (insufficient quota) |
| 5 | `test_boundary_no_free_pages` | `free_pages == 0` |
| 6 | `test_boundary_va_zero_not_4k_valid` | `va=0` not 4k-valid |
| 7 | `test_boundary_va_already_in_io_space` | VA already mapped in IO space |

## Behavioral Mutation Tests (7/7 FAIL ✅)

| # | Test | Mutated Behavior |
|---|---|---|
| 1 | `test_mutation_free_pages_unchanged` | Free pages count unchanged (should decrease by 1) |
| 2 | `test_mutation_io_space_unchanged` | IO space unchanged (should gain new VA) |
| 3 | `test_mutation_ret_write_false` | Return `write=false` (should be `true`) |
| 4 | `test_mutation_ret_execute_disable_true` | Return `execute_disable=true` (should be `false`) |
| 5 | `test_mutation_proc_dom_changes` | Proc removed from domain (should be preserved) |
| 6 | `test_mutation_quota_unchanged` | Container quota unchanged (should decrease by 1) |
| 7 | `test_mutation_address_space_changed` | Address space changed (should be preserved) |

## Logical Tests (7/7 FAIL ✅)

| # | Test | Unentailed Property |
|---|---|---|
| 1 | `test_logical_determinism` | Two calls produce same page address (determinism) |
| 2 | `test_logical_free_pages_decrease_by_two` | Free pages decrease by 2 (stronger inequality) |
| 3 | `test_logical_other_proc_io_space_changes` | Other proc's IO space changed (isolation violation) |
| 4 | `test_logical_endpoint_dom_changes` | Endpoint domain changed (structural assumption) |
| 5 | `test_logical_container_pages_unchanged` | Container owned pages unchanged (should grow) |
| 6 | `test_logical_thread_dom_changes` | Thread domain changed (global assumption) |
| 7 | `test_logical_zero_page_ptr_invalid` | `page_ptr_valid(0)` is false (null pointer) |

---

## Conclusion

The specification for `alloc_and_map_io` is **consistent** across all tested dimensions:
- **Boundary**: All 7 preconditions are enforced — invalid inputs are rejected.
- **Behavioral**: All 7 postcondition mutations are rejected — incorrect behaviors are caught.
- **Logical**: All 7 unintended properties are rejected — the spec does not over-entail.

**Notable finding**: `page_ptr_valid(0)` evaluates to `true` (Test Logical-7 fails when asserting it's false). This means the specification permits a null/zero page pointer as valid. Whether this is intentional or a spec gap depends on the system's memory layout (page 0 at physical address 0 may be reserved).
