# Test Execution Summary: `syscall_io_mmap`

## Target Function
`Kernel::syscall_io_mmap(&mut self, thread_ptr: ThreadPtr, va_range: VaRange4K) -> SyscallReturnStruct`

**Key observation:** This function has **NO `ensures` clause** — a significant specification gap. The function body checks ioid existence, container quota, IO space availability, then allocates/maps IO pages, but none of these outcomes are formally guaranteed as postconditions.

---

## Results Overview

| Test File | Tests | Failed (Expected) | Passed (Unexpected) |
|-----------|-------|--------------------|---------------------|
| `boundary_tests.rs` | 10 | 10 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 10 | 10 ✅ | 0 |
| `logical_tests.rs` | 10 | 10 ✅ | 0 |
| **Total** | **30** | **30** | **0** |

All 30 adversarial tests were **correctly rejected** by Verus verification.

---

## Boundary Tests (10/10 failed ✅)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_boundary_thread_not_in_domain` | thread_ptr not in thread_dom |
| 2 | `test_boundary_va_range_overflow` | va_range.len * 4 overflows usize::MAX |
| 3 | `test_boundary_invalid_va_alignment` | VA not 4K-aligned (MEM_4k_MASK check) |
| 4 | `test_boundary_unaligned_page_ptr` | page_ptr not 0x1000-aligned |
| 5 | `test_boundary_page_index_at_num_pages` | page index == NUM_PAGES (off-by-one) |
| 6 | `test_boundary_zero_quota_nonzero_len` | quota=0 with len>0 |
| 7 | `test_boundary_quota_off_by_one` | quota = len*4 - 1 (strictly less) |
| 8 | `test_boundary_zero_free_pages` | zero free pages with nonzero len |
| 9 | `test_boundary_page_index_usize_max` | page index = usize::MAX |
| 10 | `test_boundary_multiple_violations` | simultaneous violations |

**Conclusion:** Preconditions correctly reject all boundary/edge-case inputs.

---

## Behavioral Mutation Tests (10/10 failed ✅)

| # | Test | Mutation |
|---|------|---------|
| 1 | `test_mutation_no_ioid_returns_success` | ioid=None but claim success |
| 2 | `test_mutation_insufficient_quota_returns_success` | insufficient quota but claim success |
| 3 | `test_mutation_va_in_use_returns_success` | VA occupied but claim success |
| 4 | `test_mutation_no_switch_is_switch` | NoSwitchNew but claim Switch |
| 5 | `test_mutation_no_switch_pcid_is_some` | NoSwitchNew but claim pcid=Some |
| 6 | `test_mutation_no_switch_cr3_is_some` | NoSwitchNew but claim cr3=Some |
| 7 | `test_mutation_io_space_check_negated` | negate io_space_range_free result |
| 8 | `test_mutation_range_alloc_breaks_wf` | claim wf broken after alloc |
| 9 | `test_mutation_success_path_returns_error` | all checks pass but claim error |
| 10 | `test_mutation_occupied_va_is_free` | VA in domain but claim free |

**Conclusion:** Behavioral mutations are correctly rejected.

---

## Logical Tests (10/10 failed ✅)

| # | Test | Unentailed Property |
|---|------|-------------------|
| 1 | `test_logical_always_returns_error` | function always returns error |
| 2 | `test_logical_syscall_preserves_total_wf` | total_wf preserved (no ensures!) |
| 3 | `test_logical_determinism` | deterministic results |
| 4 | `test_logical_proc_dom_preserved` | proc_dom unchanged |
| 5 | `test_logical_container_dom_preserved` | container_dom unchanged |
| 6 | `test_logical_free_pages_exact_decrease` | free pages decrease by len*4 |
| 7 | `test_logical_io_space_contains_new_mappings` | new VAs in IO space after success |
| 8 | `test_logical_quota_decreases` | quota decreases after alloc |
| 9 | `test_logical_page_ptr_index_inverse` | stronger inverse claim (ptr+4096) |
| 10 | `test_logical_thread_dom_unchanged` | thread_dom preserved |

**Conclusion:** Logical properties not stated in the spec are correctly unentailed. However, tests 2, 4–8, 10 highlight a **major spec weakness**: `syscall_io_mmap` has **no ensures clause**, meaning callers cannot reason about its effects. Properties like wf-preservation, domain stability, quota decrease, and IO mapping addition are reasonable expectations that the spec should guarantee but currently does not.

---

## Specification Weakness Identified

The most critical finding is that `syscall_io_mmap` lacks an `ensures` clause entirely. While the internal helper `range_alloc_and_map_io` guarantees `self.wf()`, the outer syscall function does not propagate this or any other guarantee to callers. Recommended additions:

1. `ensures self.wf()` — well-formedness preservation
2. Postconditions specifying which error code is returned for each failure path
3. Domain preservation guarantees (proc_dom, container_dom, thread_dom unchanged)
4. On success: IO space updated with new mappings, quota decreased
