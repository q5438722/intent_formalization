# Test Execution Summary: `create_iommu_table_l2_entry`

## Target
`memory_manager__spec_impl__impl0__create_iommu_table_l2_entry.rs`

## Results: All 15 adversarial tests FAILED verification ✅

This means the specification correctly rejects all tested invalid inputs, incorrect behaviors, and unwarranted logical claims.

---

### Boundary Tests (5/5 failed as expected)
| # | Test | Property Tested | Result |
|---|------|----------------|--------|
| 1 | `test_boundary_misaligned_page_ptr` | `page_ptr_valid(1)` — rejects non-aligned ptr | ✅ FAIL |
| 2 | `test_boundary_page_ptr_at_limit` | `page_ptr_valid(NUM_PAGES*0x1000)` — rejects at boundary | ✅ FAIL |
| 3 | `test_boundary_present_entry_not_empty` | `PageEntry{present:true}.is_empty()` — present entry not empty | ✅ FAIL |
| 4 | `test_boundary_zero_not_present` | `usize2present(0)` — zero has no present bit | ✅ FAIL |
| 5 | `test_boundary_mem_valid_low_bits` | `MEM_valid(1)` — low bits violate mask | ✅ FAIL |

### Behavioral Mutation Tests (5/5 failed as expected)
| # | Test | Property Tested | Result |
|---|------|----------------|--------|
| 1 | `test_mutation_page_entry_nonzero_addr` | `PageEntry{addr:4096}.is_empty()` — nonzero addr not empty | ✅ FAIL |
| 2 | `test_mutation_usize2pa_preserves_low_bits` | `spec_usize2pa(1)==1` — low bits are masked out | ✅ FAIL |
| 3 | `test_mutation_write_bit_false` | `perm(WRITE_MASK).write==false` — write bit correctly detected | ✅ FAIL |
| 4 | `test_mutation_write_entry_not_empty` | `PageEntry{write:true}.is_empty()` — write flag not empty | ✅ FAIL |
| 5 | `test_mutation_present_bit_false` | `perm(PRESENT_MASK).present==false` — present bit detected | ✅ FAIL |

### Logical Tests (5/5 failed as expected)
| # | Test | Property Tested | Result |
|---|------|----------------|--------|
| 1 | `test_logical_valid_ptr_not_unique` | `page_ptr_valid(ptr) ⊬ ptr==0` — no uniqueness | ✅ FAIL |
| 2 | `test_logical_mem_valid_not_zero` | `MEM_valid(v) ⊬ v==0` — not forced to zero | ✅ FAIL |
| 3 | `test_logical_usize2page_entry_deterministic` | `e1.addr != e2.addr` for same input — determinism holds | ✅ FAIL |
| 4 | `test_logical_valid_ptr_can_be_large` | `page_ptr_valid(ptr) ⊬ ptr<0x1000` — no false upper bound | ✅ FAIL |
| 5 | `test_logical_empty_map_has_no_elements` | `empty().dom().contains(0)` — empty map has no elements | ✅ FAIL |

---

## Conclusion

The specification for `create_iommu_table_l2_entry` and its supporting definitions correctly:
- **Rejects invalid inputs** at precondition boundaries (alignment, ranges, validity masks)
- **Rejects mutated behaviors** (wrong field values, incorrect bit extraction, spurious emptiness)
- **Rejects unwarranted reasoning** (uniqueness claims, false bounds, non-determinism assertions)

No spec weakness was detected across all 15 adversarial queries.
