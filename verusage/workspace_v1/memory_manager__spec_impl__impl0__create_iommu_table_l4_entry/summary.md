# Adversarial Test Summary: `create_iommu_table_l4_entry`

## Target
`memory_manager__spec_impl__impl0__create_iommu_table_l4_entry.rs`

## Results

All 15 adversarial tests **FAILED verification** as expected, confirming the specification correctly rejects these invalid properties.

| Category | Tests | All Failed? |
|----------|-------|-------------|
| Boundary | 5 | ✅ Yes (5/5) |
| Behavioral Mutation | 5 | ✅ Yes (5/5) |
| Logical | 5 | ✅ Yes (5/5) |

### Boundary Tests (precondition violations)
| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_boundary_unaligned_ptr` | `page_ptr_valid(0x1001)` — non-4K-aligned rejected | ✅ FAIL |
| 2 | `test_boundary_ptr_beyond_num_pages` | `page_ptr_valid(NUM_PAGES*0x1000)` — beyond limit rejected | ✅ FAIL |
| 3 | `test_boundary_write_entry_not_empty` | `is_empty` with `write=true` rejected | ✅ FAIL |
| 4 | `test_boundary_user_entry_not_empty` | `is_empty` with `user=true` rejected | ✅ FAIL |
| 5 | `test_boundary_mem_valid_low_bits` | `MEM_valid(0xFFF)` — low bits set rejected | ✅ FAIL |

### Behavioral Mutation Tests (mutated outputs)
| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_behav_pa_extraction_nonzero` | `spec_usize2pa(0x2000) == 0` — wrong extraction rejected | ✅ FAIL |
| 2 | `test_behav_zero_not_present` | `usize2present(0) == true` — false present bit rejected | ✅ FAIL |
| 3 | `test_behav_ps_bit_set` | `usize2ps(0x80) == false` — PS bit inversion rejected | ✅ FAIL |
| 4 | `test_behav_user_bit_set` | `usize2user(4) == false` — user bit inversion rejected | ✅ FAIL |
| 5 | `test_behav_present_entry_is_not_empty` | `spec_usize2page_entry(1).is_empty()` — present entry not empty | ✅ FAIL |

### Logical Tests (unintended properties)
| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_logic_zero_is_valid_ptr` | `!page_ptr_valid(0)` — zero IS valid (spec allows it) | ✅ FAIL |
| 2 | `test_logic_kernel_l4_end_not_zero` | `KERNEL_MEM_END_L4INDEX == 0` — constant is 1, not 0 | ✅ FAIL |
| 3 | `test_logic_entry_not_injective` | `usize2page_entry(0) != usize2page_entry(8)` — not injective | ✅ FAIL |
| 4 | `test_logic_write_implies_present` | `write ==> present` — independent bits, no implication | ✅ FAIL |
| 5 | `test_logic_valid_ptrs_not_unique` | `0 == 0x1000` — distinct valid ptrs exist | ✅ FAIL |

## Conclusion

The specification is **consistent** with respect to all 15 queried properties:
- **Boundary**: Invalid inputs (unaligned pointers, out-of-range pointers, non-empty entry fields) are correctly rejected.
- **Behavioral**: Mutated output relations (inverted bits, wrong PA extraction) are correctly rejected.
- **Logical**: Unintended properties (injectivity, bit implications, uniqueness) are correctly rejected. Notably, `page_ptr_valid(0)` is accepted by the spec — zero is a valid page pointer, which is an intentional design choice.

No spec weaknesses were detected by these tests.
