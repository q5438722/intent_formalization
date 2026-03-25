# Test Execution Summary: `alloc_page_table`

**Target**: `memory_manager__spec_impl__impl0__alloc_page_table.rs`

## Results: All 15 adversarial tests correctly FAILED verification ✅

The specification correctly rejects all unintended properties.

---

### Boundary Tests (5/5 failed as expected)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_present_entry_not_empty` | `PageEntry` with `present=true` asserted as `is_empty()` | ✅ FAIL |
| 2 | `test_boundary_pcid_out_of_range` | `pcid_active(PCID_MAX)` — off-by-one at upper bound | ✅ FAIL |
| 3 | `test_boundary_nonzero_addr_entry_not_empty` | `PageEntry` with `addr=4096` asserted as `is_empty()` | ✅ FAIL |
| 4 | `test_boundary_ioid_out_of_range` | `ioid_active(IOID_MAX)` — off-by-one at upper bound | ✅ FAIL |
| 5 | `test_boundary_write_entry_not_empty` | `PageEntry` with `write=true` asserted as `is_empty()` | ✅ FAIL |

### Behavioral Mutation Tests (5/5 failed as expected)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_mutation_wrong_proc_ptr` | Asserted `ret` maps to wrong `ProcPtr` | ✅ FAIL |
| 2 | `test_mutation_ret_not_active` | Asserted returned PCID is NOT active after alloc | ✅ FAIL |
| 3 | `test_mutation_ret_was_active_before` | Asserted returned PCID WAS active before alloc | ✅ FAIL |
| 4 | `test_mutation_ret_mapping_nonempty` | Asserted new PCID's mapping domain is non-empty | ✅ FAIL |
| 5 | `test_mutation_iommu_mapping_changed` | Asserted IOMMU mapping changed after alloc | ✅ FAIL |

### Logical Tests (5/5 failed as expected)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_logical_ret_always_zero` | Asserted `ret == 0` always (no such guarantee) | ✅ FAIL |
| 2 | `test_logical_ret_bounded_small` | Asserted `ret < 10` (spec allows up to `PCID_MAX`) | ✅ FAIL |
| 3 | `test_logical_free_pcids_unchanged` | Asserted free PCID set unchanged (ret became active) | ✅ FAIL |
| 4 | `test_logical_free_pcid_count_same` | Asserted free set still contains `ret` | ✅ FAIL |
| 5 | `test_logical_proc_ptr_injective` | Asserted `pcid_to_proc_ptr` is injective (not guaranteed) | ✅ FAIL |

---

## Conclusion

The `alloc_page_table` specification is **consistent** across all three test categories:
- **Boundary**: Spec predicates correctly reject out-of-range PCIDs/IOIDs and non-empty page entries.
- **Behavioral**: Postconditions correctly constrain the returned PCID's state, mapping, and proc_ptr association while preserving IOMMU state.
- **Logical**: The spec does not over-constrain (no false determinism, no unwarranted bounds, no injectivity). The free PCID set correctly changes to reflect the allocation.

No spec weaknesses were detected.
