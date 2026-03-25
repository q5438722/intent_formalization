# Specification Testing Summary: `create_iommu_table_entry`

## File Under Test

`kernel__mem_util__impl0__create_iommu_table_entry.rs` — Defines `Kernel::create_iommu_table_entry`, which creates IOMMU page table entries (L4/L3/L2) for a process's I/O address space. The function allocates up to 3 4K pages and returns `(pages_allocated, l2_entry_addr)`. Also defines numerous helper spec functions for bit-field extraction, VA/PA manipulation, page table index computation, and quota management.

---

## Correctness Results

All 139 verification items passed (including definitions + test functions).

| Test Name | Description | Expected | Actual |
|-----------|-------------|----------|--------|
| `test_usize2present_set` | `usize2present(0x1)` is true | PASS | PASS |
| `test_usize2present_clear` | `usize2present(0x0)` is false | PASS | PASS |
| `test_usize2write_set` | `usize2write(0x2)` is true | PASS | PASS |
| `test_usize2write_clear` | `usize2write(0x1)` is false | PASS | PASS |
| `test_usize2user_set` | `usize2user(0x4)` is true | PASS | PASS |
| `test_usize2user_clear` | `usize2user(0x0)` is false | PASS | PASS |
| `test_usize2ps_set` | `usize2ps(0x80)` is true | PASS | PASS |
| `test_usize2ps_clear` | `usize2ps(0x0)` is false | PASS | PASS |
| `test_usize2execute_disable_set` | Execute disable flag (arch-dependent) | PASS | PASS |
| `test_usize2execute_disable_clear` | `usize2execute_disable(0x0)` is false | PASS | PASS |
| `test_usize2present_combined_flags` | Multiple flags set simultaneously | PASS | PASS |
| `test_usize2pa_aligned_addr` | PA extraction: `0x1001` -> `0x1000` | PASS | PASS |
| `test_usize2pa_zero` | PA of zero is zero | PASS | PASS |
| `test_usize2pa_strips_low_bits` | PA masks off low 12 bits | PASS | PASS |
| `test_mem_valid_zero` | `MEM_valid(0x0)` is true | PASS | PASS |
| `test_mem_valid_page_aligned` | `MEM_valid(0x1000)` is true | PASS | PASS |
| `test_page_ptr_valid_zero` | `page_ptr_valid(0x0)` | PASS | PASS |
| `test_page_ptr_valid_first_page` | `page_ptr_valid(0x1000)` | PASS | PASS |
| `test_page_index_valid_zero` | `page_index_valid(0)` | PASS | PASS |
| `test_page_index_valid_last` | `page_index_valid(NUM_PAGES-1)` | PASS | PASS |
| `test_page_ptr2index_zero` | `ptr2index(0x0) == 0` | PASS | PASS |
| `test_page_ptr2index_first_page` | `ptr2index(0x1000) == 1` | PASS | PASS |
| `test_page_index2ptr_zero` | `index2ptr(0) == 0x0` | PASS | PASS |
| `test_page_index2ptr_one` | `index2ptr(1) == 0x1000` | PASS | PASS |
| `test_v2l1index_basic` | L1 index of `0x1000` is 1 | PASS | PASS |
| `test_v2l1index_zero` | L1 index of 0 is 0 | PASS | PASS |
| `test_v2l1index_max_bits` | L1 index with all 9 bits set | PASS | PASS |
| `test_v2l2index_basic` | L2 index of `0x200000` is 1 | PASS | PASS |
| `test_v2l2index_max_bits` | L2 index with all 9 bits set | PASS | PASS |
| `test_v2l3index_basic` | L3 index of `0x40000000` is 1 | PASS | PASS |
| `test_v2l4index_zero` | L4 index of 0 is 0 | PASS | PASS |
| `test_page_ptr_2m_valid_zero` | 2M-valid ptr at 0 | PASS | PASS |
| `test_page_ptr_2m_valid_aligned` | 2M-valid ptr at 0x200000 | PASS | PASS |
| `test_page_ptr_1g_valid_zero` | 1G-valid ptr at 0 | PASS | PASS |
| `test_page_entry_is_empty` | All-zero PageEntry is empty | PASS | PASS |
| `test_page_entry_not_empty_present` | Present flag => not empty | PASS | PASS |
| `test_page_entry_not_empty_addr` | Non-zero addr => not empty | PASS | PASS |
| `test_page_entry_not_empty_ps` | PS flag => not empty | PASS | PASS |
| `test_page_entry_not_empty_write` | Write flag => not empty | PASS | PASS |
| `test_page_entry_not_empty_execute_disable` | Execute disable => not empty | PASS | PASS |
| `test_page_entry_not_empty_user` | User flag => not empty | PASS | PASS |
| `test_quota_subtract_mem_4k_zero` | Subtract 0 from quota | PASS | PASS |
| `test_quota_subtract_mem_4k_three` | Subtract 3 from quota | PASS | PASS |
| `test_quota_subtract_preserves_other_fields` | Other fields preserved | PASS | PASS |
| `test_quota_set_mem_4k` | Set mem_4k preserves others | PASS | PASS |
| `test_va_lemma_4k_valid_indices_bounded` | 4k-valid VA => all indices < 512 | PASS | PASS |
| `test_va_lemma_2m_valid_l1_zero` | 2m-valid VA => l1 index = 0 | PASS | PASS |
| `test_va_lemma_1g_valid_l1_l2_zero` | 1g-valid VA => l1,l2 = 0 | PASS | PASS |
| `test_va_lemma_valid_indices_construct_valid_va` | Valid indices => valid VA | PASS | PASS |
| `test_va_lemma_construct_2m_valid_va` | Valid L4/L3/L2 indices => 2m-valid VA | PASS | PASS |
| `test_l1_index_range` | forall va: l1_index(va) < 512 | PASS | PASS |
| `test_l2_index_range` | forall va: l2_index(va) < 512 | PASS | PASS |
| `test_l3_index_range` | forall va: l3_index(va) < 512 | PASS | PASS |
| `test_l4_index_range` | forall va: l4_index(va) < 512 | PASS | PASS |
| `test_param_usize2pa_preserves_alignment` | forall v: PA is page-aligned | PASS | PASS |
| `test_param_quota_subtract_reflexive` | Subtract 0 is reflexive | PASS | PASS |
| `test_create_iommu_ret_bounded` | ret.0 <= 3 implication | PASS | PASS |
| `test_create_iommu_quota_subtract_consistency` | Quota math consistent with ret | PASS | PASS |
| `test_create_iommu_free_pages_decrease` | Free pages decrease by ret.0 | PASS | PASS |
| `test_usize2pa_masks_low_12` | PA of 0xFFF is 0x0 | PASS | PASS |
| `test_spec_usize2page_entry_perm_all_clear` | All flags clear at 0x0 | PASS | PASS |
| `test_spec_usize2page_entry_perm_selective` | Selective flags at 0x81 | PASS | PASS |
| `test_page_ptr_roundtrip` | ptr->index->ptr roundtrip | PASS | PASS |
| `test_page_index_roundtrip` | index->ptr->index roundtrip | PASS | PASS |
| `test_page_index_2m_valid_basic` | 2M-valid indices: 0, 512 | PASS | PASS |
| `test_page_index_1g_valid_basic` | 1G-valid index: 0 | PASS | PASS |
| `test_va2index_orthogonality` | Different VA bits map to different indices | PASS | PASS |
| `test_param_l1_index_bounded` | Parameterized: l1 < 512 | PASS | PASS |
| `test_param_l2_index_bounded` | Parameterized: l2 < 512 | PASS | PASS |
| `test_param_l3_index_bounded` | Parameterized: l3 < 512 | PASS | PASS |
| `test_param_l4_index_bounded` | Parameterized: l4 < 512 | PASS | PASS |
| `test_param_page_entry_empty_implies_zero_addr` | Empty => addr=0 | PASS | PASS |
| `test_va_index_mixed_pattern` | Mixed bit pattern VA | PASS | PASS |
| `test_va_index_all_ones_in_range` | All-max indices VA | PASS | PASS |
| `test_spec_va_4k_valid_alignment_check` | Non-aligned VA fails check | PASS | PASS |

---

## Completeness Results

### Round 1: Precondition Violations (8 tests, 8 errors)

| Test Name | What it Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| `test_fail_page_ptr_valid_unaligned` | Non-aligned ptr (0x1001) | FAIL | FAIL |
| `test_fail_page_index_valid_out_of_range` | Index = NUM_PAGES (out of range) | FAIL | FAIL |
| `test_fail_quota_subtract_wrong_k` | Subtract 5 from quota of 3 | FAIL | FAIL |
| `test_fail_quota_subtract_mismatched_mem2m` | Different mem_2m values | FAIL | FAIL |
| `test_fail_page_index_2m_unaligned` | Non-512-aligned 2M index | FAIL | FAIL |
| `test_fail_page_ptr_2m_unaligned` | Non-2M-aligned ptr | FAIL | FAIL |
| `test_fail_page_ptr_1g_unaligned` | Non-1G-aligned ptr | FAIL | FAIL |
| `test_fail_precondition_quota_insufficient` | Quota < 3 | FAIL | FAIL |

### Round 2: Overly Strong Postconditions (8 tests, 12 errors)

| Test Name | What it Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| `test_fail_l1_index_too_tight` | l1 < 256 (spec says < 512) | FAIL | FAIL |
| `test_fail_l2_index_too_tight` | l2 < 256 (spec says < 512) | FAIL | FAIL |
| `test_fail_l3_index_too_tight` | l3 < 256 (spec says < 512) | FAIL | FAIL |
| `test_fail_l4_index_too_tight` | l4 < 256 (spec says < 512) | FAIL | FAIL |
| `test_fail_usize2pa_equals_input` | PA equals input (no masking) | FAIL | FAIL |
| `test_fail_ret_too_tight` | ret.0 <= 2 (spec says <= 3) | FAIL | FAIL |
| `test_fail_quota_subtract_is_identity` | Subtract 3 but expect no change | FAIL | FAIL |
| `test_fail_mem_valid_universal` | All values are MEM_valid | FAIL | FAIL |

### Round 3: Negated/Contradicted Postconditions (10 tests, 12 errors)

| Test Name | What it Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| `test_fail_negate_present` | not present(0x1) | FAIL | FAIL |
| `test_fail_negate_write` | not write(0x2) | FAIL | FAIL |
| `test_fail_negate_user` | not user(0x4) | FAIL | FAIL |
| `test_fail_negate_ps` | not ps(0x80) | FAIL | FAIL |
| `test_fail_negate_pa_masking` | PA != masked value | FAIL | FAIL |
| `test_fail_negate_page_ptr_valid` | not page_ptr_valid(0) | FAIL | FAIL |
| `test_fail_negate_page_index_valid` | not page_index_valid(0) | FAIL | FAIL |
| `test_fail_negate_page_entry_empty` | not is_empty for all-zero entry | FAIL | FAIL |
| `test_fail_negate_quota_subtract` | not subtract_mem_4k correct result | FAIL | FAIL |
| `test_fail_l1_index_can_exceed` | exists va: l1 >= 512 | FAIL | FAIL |

### Round 4: Wrong Specific Values (10 tests, 11 errors)

| Test Name | What it Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| `test_fail_wrong_l1_index` | l1(0x1000) == 0 (should be 1) | FAIL | FAIL |
| `test_fail_wrong_l2_index` | l2(0x200000) == 2 (should be 1) | FAIL | FAIL |
| `test_fail_wrong_l3_index` | l3(0x40000000) == 0 (should be 1) | FAIL | FAIL |
| `test_fail_wrong_pa` | PA(0x1001) == 0x1001 (should be 0x1000) | FAIL | FAIL |
| `test_fail_wrong_ptr2index` | ptr2idx(0x2000) == 3 (should be 2) | FAIL | FAIL |
| `test_fail_wrong_index2ptr` | idx2ptr(2) == 0x3000 (should be 0x2000) | FAIL | FAIL |
| `test_fail_wrong_quota_result` | 10-3 == 8 (should be 7) | FAIL | FAIL |
| `test_fail_wrong_set_mem_4k` | set_mem_4k(7) gives 10 | FAIL | FAIL |
| `test_fail_wrong_l1_for_0x2000` | l1(0x2000) == 1 (should be 2) | FAIL | FAIL |
| `test_fail_wrong_present` | present(0x0) is true | FAIL | FAIL |

### Round 5: Cross-Function Misuse & Edge Cases (10 tests, 14 errors)

| Test Name | What it Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| `test_fail_4k_implies_2m` | 4K-valid ptr => 2M-valid ptr | FAIL | FAIL |
| `test_fail_2m_implies_1g` | 2M-valid ptr => 1G-valid ptr | FAIL | FAIL |
| `test_fail_4k_valid_implies_2m_valid` | 4K-valid VA => 2M-valid VA | FAIL | FAIL |
| `test_fail_pa_preserves_low_bits` | PA == input (no masking) | FAIL | FAIL |
| `test_fail_all_mem_valid` | MEM_valid(0x1) | FAIL | FAIL |
| `test_fail_l1_equals_l2` | l1 always equals l2 | FAIL | FAIL |
| `test_fail_index_2m_not_multiple` | 2M-valid index at 256 | FAIL | FAIL |
| `test_fail_all_indices_zero` | All l1 indices are 0 | FAIL | FAIL |
| `test_fail_empty_with_nonzero_addr` | Non-zero addr PageEntry is empty | FAIL | FAIL |
| `test_fail_quota_subtract_different_ioid` | Different ioid passes subtract | FAIL | FAIL |

---

## Overall Assessment

### Correctness: All specs are correct
All test functions verified successfully. The spec functions (bit-field extraction, VA index computation, page validity checks, quota management) produce correct results for both concrete values and universally-quantified inputs. The `va_lemma` proof function's ensures clauses correctly capture the relationships between VA validity and index bounds.

### Completeness: All specs are tight
All 46 completeness test functions failed verification as expected. The specs properly reject:
- Invalid inputs (precondition violations)
- Overly strong claims (tighter bounds than guaranteed)
- Negated postconditions
- Wrong concrete values
- Cross-function misuse (improper implications between validity levels)

### Notes
- `create_iommu_table_entry` is an `exec fn` (not `proof fn`), so it cannot be called from proof context. Testing focused on its helper spec functions and the logical implications of its ensures clauses.
- Error counts per round may exceed test function counts because Verus checks `by (bit_vector)` assertions on both 32-bit and 64-bit architectures independently.
- The `usize2execute_disable` test for the set case was modified to be architecture-safe since the execute_disable bit is bit 63, which only exists on 64-bit architectures.
- No spec gaps were identified.
