# Test Summary: kernel__mem_util__impl0__create_entry.rs

## File Under Test

Defines `Kernel::create_entry`, which creates page table entries (L2/L3/L4) for a process's virtual address space. The file contains:
- Page entry parsing spec functions (`usize2present`, `usize2write`, `usize2user`, `usize2ps`, `usize2execute_disable`, `spec_usize2pa`, `spec_usize2page_entry`)
- Virtual address conversion functions (`spec_v2l1index` through `spec_v2l4index`, `spec_va2index`, `spec_index2va`)
- VA validity predicates (`spec_va_4k_valid`, `spec_va_2m_valid`, `spec_va_1g_valid`)
- Page pointer/index conversion (`spec_page_ptr2page_index`, `spec_page_index2page_ptr`)
- Page validity predicates and truncation functions
- `Quota` struct with `spec_set_mem_4k` and `spec_subtract_mem_4k`
- `va_lemma` (external_body proof fn establishing VA index bounds)
- `create_entry` exec fn with extensive requires/ensures contract

**Note**: `create_entry` is an exec fn and cannot be called from proof tests directly. Tests focus on the spec helper functions that define its contract interface.

---

## Correctness Results

**File**: `correctness_tests.rs`  
**Result**: ✅ ALL 84 tests PASS (175 verified, 0 errors)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | test_quota_set_mem_4k_preserves_other_fields | set_mem_4k only changes mem_4k | PASS | ✅ PASS |
| 2 | test_quota_subtract_mem_4k_correct | 10-3=7 with matching fields | PASS | ✅ PASS |
| 3 | test_quota_subtract_zero | Subtracting 0 is identity | PASS | ✅ PASS |
| 4 | test_quota_set_then_subtract | set then subtract consistency | PASS | ✅ PASS |
| 5 | test_usize2present_zero | 0 has bit 0 clear → false | PASS | ✅ PASS |
| 6 | test_usize2present_one | 1 has bit 0 set → true | PASS | ✅ PASS |
| 7 | test_usize2present_two | 2 has bit 0 clear → false | PASS | ✅ PASS |
| 8 | test_usize2write_zero | 0 has bit 1 clear → false | PASS | ✅ PASS |
| 9 | test_usize2write_two | 2 has bit 1 set → true | PASS | ✅ PASS |
| 10 | test_usize2write_one | 1 has bit 1 clear → false | PASS | ✅ PASS |
| 11 | test_usize2user_zero | 0 has bit 2 clear → false | PASS | ✅ PASS |
| 12 | test_usize2user_four | 4 has bit 2 set → true | PASS | ✅ PASS |
| 13 | test_usize2ps_zero | 0 has bit 7 clear → false | PASS | ✅ PASS |
| 14 | test_usize2ps_0x80 | 0x80 has bit 7 set → true | PASS | ✅ PASS |
| 15 | test_usize2execute_disable_zero | 0 has bit 63 clear → false | PASS | ✅ PASS |
| 16 | test_usize2present_all_low_bits | 0xFF has bit 0 set → true | PASS | ✅ PASS |
| 17 | test_usize2write_0xff | 0xFF has bit 1 set → true | PASS | ✅ PASS |
| 18 | test_usize2user_0xff | 0xFF has bit 2 set → true | PASS | ✅ PASS |
| 19 | test_usize2ps_0xff | 0xFF has bit 7 set → true | PASS | ✅ PASS |
| 20 | test_spec_usize2pa_zero | PA of 0 is 0 | PASS | ✅ PASS |
| 21 | test_spec_usize2pa_page_aligned | PA of 0x1000 is 0x1000 | PASS | ✅ PASS |
| 22 | test_spec_usize2pa_masks_low_bits | PA of 0x1001 is 0x1000 | PASS | ✅ PASS |
| 23 | test_spec_usize2pa_preserves_mem_valid | PA output always MEM_valid | PASS | ✅ PASS |
| 24 | test_mem_valid_zero | 0 is MEM_valid | PASS | ✅ PASS |
| 25 | test_mem_valid_page_aligned | 0x1000 is MEM_valid | PASS | ✅ PASS |
| 26 | test_mem_valid_invalid_low_bit | 1 is not MEM_valid | PASS | ✅ PASS |
| 27 | test_mem_valid_invalid_low_bits | 0xFFF is not MEM_valid | PASS | ✅ PASS |
| 28 | test_page_entry_is_empty_for_zero | Entry from 0 is empty | PASS | ✅ PASS |
| 29 | test_v2l1index_zero | L1 index of 0 is 0 | PASS | ✅ PASS |
| 30 | test_v2l1index_0x1000 | L1 index of 0x1000 is 1 | PASS | ✅ PASS |
| 31 | test_v2l1index_max_l1 | L1 index of 0x1FF000 is 511 | PASS | ✅ PASS |
| 32 | test_v2l2index_zero | L2 index of 0 is 0 | PASS | ✅ PASS |
| 33 | test_v2l2index_0x200000 | L2 index of 0x200000 is 1 | PASS | ✅ PASS |
| 34 | test_v2l3index_zero | L3 index of 0 is 0 | PASS | ✅ PASS |
| 35 | test_v2l3index_0x40000000 | L3 index of 0x40000000 is 1 | PASS | ✅ PASS |
| 36 | test_v2l4index_zero | L4 index of 0 is 0 | PASS | ✅ PASS |
| 37 | test_v2l1index_bound | ∀va. L1 index < 512 | PASS | ✅ PASS |
| 38 | test_v2l2index_bound | ∀va. L2 index < 512 | PASS | ✅ PASS |
| 39 | test_v2l3index_bound | ∀va. L3 index < 512 | PASS | ✅ PASS |
| 40 | test_v2l4index_bound | ∀va. L4 index < 512 | PASS | ✅ PASS |
| 41 | test_va2index_zero | All indices of VA=0 are 0 | PASS | ✅ PASS |
| 42 | test_va_4k_valid_implies_l4_ge_1 | 4k-valid VA has L4≥1 | PASS | ✅ PASS |
| 43 | test_va_2m_valid_implies_4k_valid | 2m-valid ⟹ 4k-valid | PASS | ✅ PASS |
| 44 | test_page_ptr2index_zero | Index of ptr 0 is 0 | PASS | ✅ PASS |
| 45 | test_page_ptr2index_4096 | Index of ptr 4096 is 1 | PASS | ✅ PASS |
| 46 | test_page_ptr2index_8192 | Index of ptr 8192 is 2 | PASS | ✅ PASS |
| 47 | test_page_index2ptr_zero | Ptr of index 0 is 0 | PASS | ✅ PASS |
| 48 | test_page_index2ptr_one | Ptr of index 1 is 4096 | PASS | ✅ PASS |
| 49 | test_page_index2ptr_two | Ptr of index 2 is 8192 | PASS | ✅ PASS |
| 50 | test_page_ptr_valid_zero | ptr 0 is valid | PASS | ✅ PASS |
| 51 | test_page_ptr_valid_4096 | ptr 4096 is valid | PASS | ✅ PASS |
| 52 | test_page_ptr_valid_invalid_alignment | ptr 1 is invalid | PASS | ✅ PASS |
| 53 | test_page_index_valid_zero | index 0 is valid | PASS | ✅ PASS |
| 54 | test_page_index_valid_max_minus_1 | index NUM_PAGES-1 valid | PASS | ✅ PASS |
| 55 | test_page_index_invalid_at_max | index NUM_PAGES invalid | PASS | ✅ PASS |
| 56 | test_truncate_2m_aligned | truncate(512) == 512 | PASS | ✅ PASS |
| 57 | test_truncate_2m_unaligned | truncate(513) == 512 | PASS | ✅ PASS |
| 58 | test_truncate_2m_zero | truncate(0) == 0 | PASS | ✅ PASS |
| 59 | test_truncate_2m_below_boundary | truncate(511) == 0 | PASS | ✅ PASS |
| 60 | test_truncate_1g_aligned | truncate(262144) == 262144 | PASS | ✅ PASS |
| 61 | test_truncate_1g_unaligned | truncate(262145) == 262144 | PASS | ✅ PASS |
| 62 | test_truncate_1g_zero | truncate(0) == 0 | PASS | ✅ PASS |
| 63 | test_page_index_2m_valid_zero | 0 is 2m-valid index | PASS | ✅ PASS |
| 64 | test_page_index_2m_valid_512 | 512 is 2m-valid index | PASS | ✅ PASS |
| 65 | test_page_index_2m_valid_not_aligned | 1 is not 2m-valid | PASS | ✅ PASS |
| 66 | test_page_index_1g_valid_zero | 0 is 1g-valid index | PASS | ✅ PASS |
| 67 | test_page_index_1g_valid_zero_is_valid | 0 is 1g-valid (redundant) | PASS | ✅ PASS |
| 68 | test_page_ptr_2m_valid_zero | ptr 0 is 2m-valid | PASS | ✅ PASS |
| 69 | test_page_ptr_2m_valid_2m | ptr 0x200000 is 2m-valid | PASS | ✅ PASS |
| 70 | test_va_lemma_4k_index_bounds | va_lemma: 4k VA indices < 512 | PASS | ✅ PASS |
| 71 | test_va_lemma_2m_l1_is_zero | va_lemma: 2m VA has L1=0 | PASS | ✅ PASS |
| 72 | test_va_lemma_1g_l1_l2_are_zero | va_lemma: 1g VA L1=L2=0 | PASS | ✅ PASS |
| 73 | test_param_present_implies_nonzero | present(v) ⟹ v≠0 | PASS | ✅ PASS |
| 74 | test_param_all_false_means_zero_low_bits | ¬present∧¬write∧¬user ⟹ low 3 bits clear | PASS | ✅ PASS |
| 75 | test_roundtrip_index_to_ptr | index→ptr conversion | PASS | ✅ PASS |
| 76 | test_merge_2m_valid_in_range | merge_2m(0,1) true | PASS | ✅ PASS |
| 77 | test_merge_2m_valid_upper_bound | merge_2m(0,0x1FF) true | PASS | ✅ PASS |
| 78 | test_merge_1g_valid_in_range | merge_1g(0,1) true | PASS | ✅ PASS |
| 79 | test_merge_1g_valid_upper_bound | merge_1g(0,0x3FFFF) true | PASS | ✅ PASS |
| 80 | test_page_entry_perm_present_write | v=3: present+write, no others | PASS | ✅ PASS |
| 81 | test_page_entry_perm_present_write_user_ps | v=0x87: 4 flags set | PASS | ✅ PASS |
| 82 | test_quota_subtract_bounds | ∀k≤3. subtract coherent | PASS | ✅ PASS |
| 83 | test_page_entry_not_empty_with_addr | Non-zero addr ⟹ not empty | PASS | ✅ PASS |
| 84 | test_page_entry_not_empty_with_present | present=true ⟹ not empty | PASS | ✅ PASS |

---

## Completeness Results

### Round 1: Precondition Violations
**File**: `completeness_round1.rs`  
**Result**: ✅ ALL 8 tests FAIL as expected

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | test_fail_quota_subtract_too_much | Underflow: mem_4k=2, k=3 | FAIL | ❌ FAIL |
| 2 | test_fail_quota_subtract_mismatched_fields | mem_2m changed (20→30) | FAIL | ❌ FAIL |
| 3 | test_fail_page_ptr_valid_unaligned | ptr=1 (not 4k-aligned) | FAIL | ❌ FAIL |
| 4 | test_fail_page_index_valid_out_of_range | index=NUM_PAGES (≥ max) | FAIL | ❌ FAIL |
| 5 | test_fail_mem_valid_with_low_bits | 0xFFF has low bits set | FAIL | ❌ FAIL |
| 6 | test_fail_present_when_bit0_clear | Assert present(2) (bit 0 clear) | FAIL | ❌ FAIL |
| 7 | test_fail_write_when_bit1_clear | Assert write(1) (bit 1 clear) | FAIL | ❌ FAIL |
| 8 | test_fail_user_when_bit2_clear | Assert user(1) (bit 2 clear) | FAIL | ❌ FAIL |

### Round 2: Overly Strong Postconditions
**File**: `completeness_round2.rs`  
**Result**: ✅ ALL 8 tests FAIL as expected

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | test_fail_l1index_too_tight_bound | L1 < 256 (spec says < 512) | FAIL | ❌ FAIL |
| 2 | test_fail_l2index_too_tight_bound | L2 < 256 (spec says < 512) | FAIL | ❌ FAIL |
| 3 | test_fail_l3index_too_tight_bound | L3 < 256 (spec says < 512) | FAIL | ❌ FAIL |
| 4 | test_fail_l4index_too_tight_bound | L4 < 256 (spec says < 512) | FAIL | ❌ FAIL |
| 5 | test_fail_pa_equals_input | PA == input (it masks bits) | FAIL | ❌ FAIL |
| 6 | test_fail_aligned_implies_valid | Aligned ⟹ valid (needs size check) | FAIL | ❌ FAIL |
| 7 | test_fail_quota_subtract_wrong_k | k=2 when should be k=3 | FAIL | ❌ FAIL |
| 8 | test_fail_truncate_2m_identity | truncate == identity (it truncates) | FAIL | ❌ FAIL |

### Round 3: Negated/Contradicted Postconditions
**File**: `completeness_round3.rs`  
**Result**: ✅ ALL 8 tests FAIL as expected

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | test_fail_negate_present | ¬present(1) (bit 0 IS set) | FAIL | ❌ FAIL |
| 2 | test_fail_negate_write | ¬write(2) (bit 1 IS set) | FAIL | ❌ FAIL |
| 3 | test_fail_negate_user | ¬user(4) (bit 2 IS set) | FAIL | ❌ FAIL |
| 4 | test_fail_negate_ps | ¬ps(0x80) (bit 7 IS set) | FAIL | ❌ FAIL |
| 5 | test_fail_negate_mem_valid | ¬MEM_valid(0x1000) (it IS valid) | FAIL | ❌ FAIL |
| 6 | test_fail_negate_is_empty | ¬is_empty(entry_from_0) (it IS empty) | FAIL | ❌ FAIL |
| 7 | test_fail_negate_l1index_zero | L1(0)≠0 (it IS 0) | FAIL | ❌ FAIL |
| 8 | test_fail_negate_quota_subtract | ¬subtract(valid) (it DOES hold) | FAIL | ❌ FAIL |

### Round 4: Wrong Specific Values
**File**: `completeness_round4.rs`  
**Result**: ✅ ALL 8 tests FAIL as expected

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | test_fail_wrong_l1index_for_0x1000 | L1(0x1000)==2 (should be 1) | FAIL | ❌ FAIL |
| 2 | test_fail_wrong_l2index_for_0x200000 | L2(0x200000)==0 (should be 1) | FAIL | ❌ FAIL |
| 3 | test_fail_wrong_l3index_for_0x40000000 | L3(0x40000000)==2 (should be 1) | FAIL | ❌ FAIL |
| 4 | test_fail_wrong_pa_for_0x1001 | PA(0x1001)==0x1001 (should be 0x1000) | FAIL | ❌ FAIL |
| 5 | test_fail_wrong_page_index_for_4096 | index(4096)==0 (should be 1) | FAIL | ❌ FAIL |
| 6 | test_fail_wrong_page_ptr_for_1 | ptr(1)==8192 (should be 4096) | FAIL | ❌ FAIL |
| 7 | test_fail_wrong_truncate_2m | trunc(513)==513 (should be 512) | FAIL | ❌ FAIL |
| 8 | test_fail_wrong_quota_subtract | 10-3=8 (should be 7) | FAIL | ❌ FAIL |

### Round 5: Cross-function Misuse & Edge Cases
**File**: `completeness_round5.rs`  
**Result**: ✅ ALL 8 tests FAIL as expected

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | test_fail_all_va_valid | ∀va. va_4k_valid(va) (false) | FAIL | ❌ FAIL |
| 2 | test_fail_present_implies_write | present ⟹ write (independent bits) | FAIL | ❌ FAIL |
| 3 | test_fail_l1index_injective | Same L1 ⟹ same VA (false) | FAIL | ❌ FAIL |
| 4 | test_fail_ptr_valid_implies_index_valid | ptr_valid ⟹ index_valid(ptr) | FAIL | ❌ FAIL |
| 5 | test_fail_2m_valid_implies_l2_zero | 2m-valid ⟹ L2=0 (only L1=0) | FAIL | ❌ FAIL |
| 6 | test_fail_set_mem_4k_changes_mem_2m | set_mem_4k changes mem_2m (no) | FAIL | ❌ FAIL |
| 7 | test_fail_merge_2m_at_boundary | merge(0, 0x200) (j must be < i+0x200) | FAIL | ❌ FAIL |
| 8 | test_fail_truncate_preserves_non_aligned | trunc(511)==511 (should be 0) | FAIL | ❌ FAIL |

---

## Overall Assessment

### Correctness: ✅ PASS
All 84 correctness tests verify successfully. The spec functions correctly define:
- Bit-level page entry parsing with proper mask application
- VA-to-index extraction with 9-bit field isolation
- Page pointer/index conversion via multiplication/division by 4096
- Quota management with proper field isolation
- `va_lemma` postconditions (index bounds for valid VAs)

### Completeness: ✅ PASS
All 40 completeness tests (8 per round × 5 rounds) fail as expected. The specs correctly reject:
- Precondition violations (underflow, mismatched fields, invalid values)
- Overly strong claims (tighter bounds than guaranteed)
- Contradictions of known results
- Wrong concrete values
- Invalid cross-function relationships

### Spec Gaps: None found
No completeness test passed unexpectedly. The spec functions are both correct and sufficiently tight for the tested properties.

### Limitations
- `create_entry` is an exec fn and cannot be tested directly from proof functions
- `usize` in Verus is architecture-independent, so values > 2^32 cannot be expressed as literals; parameterized tests with preconditions are used instead
- `usize2execute_disable` (bit 63) could not be tested with concrete large values due to architecture-independent usize
