# Adversarial Test Summary: `syscall_mmap_to_iommu_table`

## Target
`kernel__syscall_io_mmap__impl0__syscall_mmap_to_iommu_table.rs` — the `syscall_io_mmap` function and its supporting specifications for IO memory mapping in the Atmosphere kernel.

## Results

| Test Category          | Tests | Failed (expected) | Passed (spec weakness) |
|------------------------|-------|--------------------|------------------------|
| Boundary Tests         | 8     | 8                  | 0                      |
| Behavioral Mutation    | 8     | 8                  | 0                      |
| Logical Tests          | 8     | 8                  | 0                      |
| **Total**              | **24**| **24**             | **0**                  |

All 24 adversarial tests **correctly failed verification**, meaning the specification rejects all tested invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (8/8 failed ✓)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_boundary_va_zero_is_valid` | VA 0 rejected: L4 index < KERNEL_MEM_END_L4INDEX |
| 2 | `test_boundary_unaligned_page_ptr` | ptr=1 rejected: not 4096-aligned |
| 3 | `test_boundary_page_index_at_limit` | index=NUM_PAGES rejected: off-by-one boundary |
| 4 | `test_boundary_mem_valid_low_bits` | addr=1 rejected: low bits set violates MEM_MASK |
| 5 | `test_boundary_present_entry_not_empty` | present=true rejected by is_empty() |
| 6 | `test_boundary_page_ptr_max_value` | usize::MAX rejected: not aligned |
| 7 | `test_boundary_2m_index_not_aligned` | index=1 rejected: not 512-aligned for 2M pages |
| 8 | `test_boundary_va_kernel_space_low` | VA 0x1000 rejected: in kernel space |

## Behavioral Mutation Tests (8/8 failed ✓)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_mutation_page_entry_nonzero_addr` | addr=4096 mutated; is_empty requires addr==0 |
| 2 | `test_mutation_page_entry_write_set` | write=true mutated; is_empty requires write==false |
| 3 | `test_mutation_page_entry_user_set` | user=true mutated; is_empty requires user==false |
| 4 | `test_mutation_page_entry_ps_set` | ps=true mutated; is_empty requires ps==false |
| 5 | `test_mutation_page_entry_execute_disable_set` | exec_disable=true mutated; is_empty rejects |
| 6 | `test_mutation_io_space_not_free` | IO space with mapped VA correctly detected as not free |
| 7 | `test_mutation_check_io_contradicts_spec` | check_io_space ensures correctly links to spec |
| 8 | `test_mutation_range_alloc_breaks_wf` | Cannot assert !wf() when wf() is given |

## Logical Tests (8/8 failed ✓)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_logical_valid_ptr_not_unique` | Valid ptr not forced to be 0; multiple valid ptrs exist |
| 2 | `test_logical_roundtrip_breaks` | index↔ptr roundtrip is sound; cannot assert breakage |
| 3 | `test_logical_index2ptr_injective` | index2ptr is injective (i*4096 ≠ j*4096 when i≠j) |
| 4 | `test_logical_quota_exceeds_fold` | fold_mem_4k_lemma ensures fold ≥ individual quota |
| 5 | `test_logical_empty_range_not_free` | Empty range (len=0) is vacuously free; cannot assert ¬free |
| 6 | `test_logical_ioid_not_unique` | pcid_ioid_wf establishes IOid→proc bijection; same ioid ⇒ same proc |
| 7 | `test_logical_4k_implies_2m` | 4K-valid ptr not always 2M-valid (alignment difference) |
| 8 | `test_logical_syscall_preserves_total_wf` | syscall_io_mmap has NO ensures clause; cannot derive total_wf |

## Key Observations

1. **Missing `ensures` on `syscall_io_mmap`**: The main function has **no postcondition**, which means callers cannot reason about the post-state. Test 8 (logical) confirms this is a spec gap — `total_wf()` cannot be derived after the call. However, since Verus still verifies the function body, internal consistency is maintained.

2. **Spec completeness for `io_space_range_free`**: The spec correctly handles edge cases — a range with len=0 is vacuously free, and occupied addresses are correctly detected as not free.

3. **IOid uniqueness is enforced**: The `pcid_ioid_wf` specification correctly establishes a bijection between IOids and process pointers, preventing two processes from sharing an IOid.

4. **All PageEntry fields are individually guarded**: Each field in `PageEntryPerm` is independently checked by `is_empty()`, preventing any single-field mutation from passing.
