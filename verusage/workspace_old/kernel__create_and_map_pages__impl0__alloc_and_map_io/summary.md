# Test Summary: kernel__create_and_map_pages__impl0__alloc_and_map_io

## File Under Test

This file defines kernel memory management infrastructure for the Atmosphere verified OS kernel:
- **`alloc_and_map_io`**: Main exec function that allocates a 4k page and maps it for IO in IOMMU tables
- **`va_lemma`**: Proof function establishing VA↔index conversion properties (bijection, bounds, validity)
- **`alloc_and_map_io_4k`**: External body for page allocator IO page allocation
- **`iommu_table_map_4k_page`**: External body for IOMMU page table mapping
- Various spec helper functions for VA manipulation, page pointer validity, and quota management

## Correctness Results

**File**: `correctness_tests.rs` — **74 verified, 0 errors** ✅

| Test Name | Description | Expected | Actual |
|-----------|-------------|----------|--------|
| test_va_lemma_4k_index_bounds | Valid 4k VA ⟹ all 4 indices ∈ [0,512) | PASS | ✅ PASS |
| test_va_lemma_2m_index_bounds | Valid 2m VA ⟹ indices bounded, l1=0 | PASS | ✅ PASS |
| test_va_lemma_1g_index_bounds | Valid 1g VA ⟹ indices bounded, l2=l1=0 | PASS | ✅ PASS |
| test_va_lemma_index2va_valid_4k | Valid indices ⟹ spec_index2va produces valid 4k VA | PASS | ✅ PASS |
| test_va_lemma_index_equality | Equal index tuples ⟹ equal VAs | PASS | ✅ PASS |
| test_va_lemma_index_inequality | Unequal index tuples ⟹ unequal VAs | PASS | ✅ PASS |
| test_va_lemma_index2va_2m_valid | Valid indices with l1=0 ⟹ valid 2m VA | PASS | ✅ PASS |
| test_va_lemma_roundtrip | va_4k_valid(va) ∧ spec_va2index(va)=(l4,l3,l2,l1) ⟹ spec_index2va(...)=va | PASS | ✅ PASS |
| test_page_ptr_valid_zero | page_ptr_valid(0) is true | PASS | ✅ PASS |
| test_page_ptr_valid_aligned | 0x1000 is 4k-aligned | PASS | ✅ PASS |
| test_page_index_valid_bounds | 0, 1, NUM_PAGES-1 are valid page indices | PASS | ✅ PASS |
| test_quota_set_mem_4k | set_mem_4k updates mem_4k, preserves others | PASS | ✅ PASS |
| test_quota_subtract_mem_4k | subtract_mem_4k with correct values | PASS | ✅ PASS |
| test_quota_subtract_mem_4k_preserves_fields | subtract preserves non-mem_4k fields | PASS | ✅ PASS |
| test_quota_set_mem_4k_idempotent | set_mem_4k with same value is identity | PASS | ✅ PASS |
| test_page_entry_is_empty | Zero PageEntry is empty | PASS | ✅ PASS |
| test_page_entry_not_empty_when_present | PageEntry with present=true is not empty | PASS | ✅ PASS |
| test_page_entry_not_empty_when_addr_nonzero | PageEntry with addr≠0 is not empty | PASS | ✅ PASS |
| test_page_index_2m_valid | 512 is valid 2m page index | PASS | ✅ PASS |
| test_page_index_2m_valid_zero | 0 is valid 2m page index | PASS | ✅ PASS |
| test_spec_usize2pa_zero | spec_usize2pa(0) == 0 | PASS | ✅ PASS |
| test_kernel_mem_end_l4index | KERNEL_MEM_END_L4INDEX == 1 | PASS | ✅ PASS |
| test_num_pages | NUM_PAGES == 2*1024*1024 | PASS | ✅ PASS |
| test_pcid_max | PCID_MAX == 4096 | PASS | ✅ PASS |
| test_ioid_max | IOID_MAX == 4096 | PASS | ✅ PASS |
| test_page_ptr_valid_definition | Bit-level alignment verification | PASS | ✅ PASS |

## Completeness Results

### Round 1: Precondition Violations — **5 errors** ✅

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| test_no_va_lemma_4k_bounds | Assert index bounds without calling va_lemma | FAIL | ✅ FAIL |
| test_no_va_lemma_2m_bounds | Assert l1=0 without calling va_lemma | FAIL | ✅ FAIL |
| test_no_va_lemma_index2va | Assert index2va validity without va_lemma | FAIL | ✅ FAIL |
| test_no_va_lemma_index_inequality | Assert index inequality without va_lemma | FAIL | ✅ FAIL |
| test_no_va_lemma_roundtrip | Assert roundtrip property without va_lemma | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions — **5 errors** ✅

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| test_overly_strong_l4_bound | l4 index < 256 (spec says < 512) | FAIL | ✅ FAIL |
| test_overly_strong_l3_bound | l3 index < 128 (spec says < 512) | FAIL | ✅ FAIL |
| test_overly_strong_quota_subtract | Quota subtract with wrong mem_2m | FAIL | ✅ FAIL |
| test_overly_strong_page_ptr | page_ptr_valid ⟹ page_ptr_2m_valid | FAIL | ✅ FAIL |
| test_overly_strong_page_index | page_index_valid ⟹ page_index_2m_valid | FAIL | ✅ FAIL |

### Round 3: Negated/Contradicted Postconditions — **5 errors** ✅

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| test_negate_l4_bound | l4 index ≥ 512 (spec says < 512) | FAIL | ✅ FAIL |
| test_negate_index_equality | Same tuple ⟹ different VAs (spec says equal) | FAIL | ✅ FAIL |
| test_negate_quota_set | set_mem_4k(100) ⟹ mem_4k ≠ 100 | FAIL | ✅ FAIL |
| test_negate_page_entry_empty | Zero PageEntry is NOT empty (spec says empty) | FAIL | ✅ FAIL |
| test_negate_2m_l1_zero | 2m VA has l1 ≠ 0 (spec says l1 = 0) | FAIL | ✅ FAIL |

### Round 4: Wrong Specific Values — **5 errors** ✅

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| test_wrong_quota_subtract | mem_4k=8 after subtracting 1 from 10 (should be 9) | FAIL | ✅ FAIL |
| test_wrong_set_mem_4k | set_mem_4k(100) ⟹ mem_4k=200 | FAIL | ✅ FAIL |
| test_wrong_page_ptr_valid | page_ptr_valid(0x1001) (not aligned) | FAIL | ✅ FAIL |
| test_wrong_num_pages | NUM_PAGES == 1M (it's 2M) | FAIL | ✅ FAIL |
| test_wrong_kernel_mem_end | KERNEL_MEM_END_L4INDEX == 0 (it's 1) | FAIL | ✅ FAIL |

### Round 5: Cross-function Misuse & Edge Cases — **5 errors** ✅

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| test_cross_function_no_lemma | Roundtrip without va_lemma | FAIL | ✅ FAIL |
| test_quota_cross_field_change | set_mem_4k changes mem_2m | FAIL | ✅ FAIL |
| test_1g_nonzero_l2 | 1g VA has l2 > 0 (spec says l2 = 0) | FAIL | ✅ FAIL |
| test_wrong_1g_alignment | page_index_1g_valid(1) (not 512*512 aligned) | FAIL | ✅ FAIL |
| test_quota_subtract_wrong_field | Subtract changes ioid instead of mem_4k | FAIL | ✅ FAIL |

## Overall Assessment

- **Correctness**: ✅ All 26 correctness tests pass (74 total verifications including original file definitions). The specs correctly describe valid behaviors.
- **Completeness**: ✅ All 25 completeness tests fail as expected. The specs are tight enough to reject invalid claims across all categories:
  - Precondition violations are properly detected
  - Overly strong postconditions are rejected
  - Negated postconditions are rejected
  - Wrong concrete values are rejected
  - Cross-function misuse is detected
- **Spec Gaps Found**: None. The tested specs (va_lemma, Quota operations, PageEntry, page pointer/index validity) are both correct and complete within the scope of what can be tested without constructing complex Kernel state.
- **Note**: The `alloc_and_map_io` function (exec fn) and its sub-functions (`alloc_and_map_io_4k`, `iommu_table_map_4k_page`, `set_container_mem_quota_mem_4k`) require complex mutable Kernel state that cannot be easily constructed in proof tests. Their specs are tested indirectly through the verified body of `alloc_and_map_io`, which Verus successfully verifies.
