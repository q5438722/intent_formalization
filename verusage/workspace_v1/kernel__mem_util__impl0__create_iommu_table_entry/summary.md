# Adversarial Test Summary: `create_iommu_table_entry`

## Target
`Kernel::create_iommu_table_entry` — allocates up to 3 IOMMU page table pages (L4/L3/L2) for a process with an IOMMU table, ensuring the L2 entry resolves for the given virtual address.

## Results

| Test File | Tests | All Failed (as expected) |
|-----------|-------|------------------------|
| `boundary_tests.rs` | 7 | ✅ Yes (7/7 errors) |
| `behavioral_mutation_tests.rs` | 7 | ✅ Yes (7/7 errors) |
| `logical_tests.rs` | 6 | ✅ Yes (6/6 errors) |
| **Total** | **20** | **✅ 20/20** |

## Boundary Tests (7/7 FAIL ✅)

| # | Test | Property Violated | Result |
|---|------|-------------------|--------|
| 1 | `test_boundary_va_zero_not_valid_4k` | VA=0 has L4 index 0 < KERNEL_MEM_END_L4INDEX | FAIL ✅ |
| 2 | `test_boundary_va_unaligned_not_valid` | VA=1 not 4k-aligned | FAIL ✅ |
| 3 | `test_boundary_page_ptr_max_invalid` | usize::MAX fails alignment and range | FAIL ✅ |
| 4 | `test_boundary_page_index_at_num_pages` | Off-by-one: index == NUM_PAGES out of range | FAIL ✅ |
| 5 | `test_boundary_quota_insufficient` | mem_4k=2 < 3 required | FAIL ✅ |
| 6 | `test_boundary_quota_zero` | mem_4k=0 < 3 required | FAIL ✅ |
| 7 | `test_boundary_present_entry_not_empty` | present=true entry is not empty | FAIL ✅ |

## Behavioral Mutation Tests (7/7 FAIL ✅)

| # | Test | Mutated Postcondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_mutation_ret_exceeds_3` | Assert ret.0 > 3 (spec says ≤3) | FAIL ✅ |
| 2 | `test_mutation_free_pages_wrong_amount` | Free pages reduced by ret.0+1 | FAIL ✅ |
| 3 | `test_mutation_io_space_changed` | IO space changed (spec says preserved) | FAIL ✅ |
| 4 | `test_mutation_address_space_changed` | Address space changed (spec says preserved) | FAIL ✅ |
| 5 | `test_mutation_proc_dom_changed` | proc_dom changed (spec says preserved) | FAIL ✅ |
| 6 | `test_mutation_quota_wrong_subtraction` | Quota subtracted by wrong amount | FAIL ✅ |
| 7 | `test_mutation_page_mapping_changed` | page_mapping changed (spec says preserved) | FAIL ✅ |

## Logical Tests (6/6 FAIL ✅)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_determinism` | Same inputs → same ret.0 (not guaranteed) | FAIL ✅ |
| 2 | `test_logical_strict_less_than_3` | ret.0 < 3 strictly (spec allows =3) | FAIL ✅ |
| 3 | `test_logical_always_allocates` | ret.0 > 0 always (spec allows =0) | FAIL ✅ |
| 4 | `test_logical_ioid_active_flipped` | ioid_active changed (spec preserves) | FAIL ✅ |
| 5 | `test_logical_container_pages_changed` | Container pages changed (spec preserves) | FAIL ✅ |
| 6 | `test_logical_subtract_zero` | Quota subtract with k=0 when mem_4k changed | FAIL ✅ |

## Conclusion

All 20 adversarial tests correctly **fail** verification, confirming that the specification of `create_iommu_table_entry`:

1. **Rejects invalid inputs**: VA=0, unaligned VA, max pointers, insufficient quota — all properly excluded by preconditions.
2. **Rejects incorrect behaviors**: Mutated postconditions (wrong return value bounds, wrong free page counts, changed IO/address spaces, wrong quota subtraction) are all caught.
3. **Rejects unintended reasoning**: The spec does not over-promise determinism, strict bounds, mandatory allocation, or allow unsound cross-function inferences.

The specification appears **consistent** with respect to the tested semantic boundaries — no unintended properties were found to be entailed.
