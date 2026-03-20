# Test Summary: kernel__syscall_io_mmap__impl0__syscall_io_mmap.rs

## File Under Test

This file implements the `syscall_io_mmap` system call for the Atmosphere verified kernel. It defines the complete type hierarchy: `Kernel`, `ProcessManager`, `MemoryManager`, `PageAllocator`, `Container`, `Process`, `Thread`, `PageTable`, `PageEntry`, `VaRange4K`, and numerous spec functions for page pointer/index conversions, bit-field extraction, and well-formedness predicates.

The main function `syscall_io_mmap` takes a thread pointer and a VA range, checks preconditions (IOMMU table exists, sufficient quota, VA range free), then allocates and maps IO pages. It has `requires` but no explicit `ensures`.

## Correctness Results (should all PASS)

| Test Name | Description | Expected | Actual |
|-----------|-------------|----------|--------|
| test_page_ptr2index_concrete_0 | ptr=0 → index=0 | PASS | ✅ PASS |
| test_page_ptr2index_concrete_4096 | ptr=4096 → index=1 | PASS | ✅ PASS |
| test_page_ptr2index_concrete_8192 | ptr=8192 → index=2 | PASS | ✅ PASS |
| test_page_index2ptr_concrete_0 | index=0 → ptr=0 | PASS | ✅ PASS |
| test_page_index2ptr_concrete_1 | index=1 → ptr=4096 | PASS | ✅ PASS |
| test_page_index2ptr_concrete_2 | index=2 → ptr=8192 | PASS | ✅ PASS |
| test_roundtrip_index_to_ptr_to_index | i→ptr→i roundtrip | PASS | ✅ PASS |
| test_roundtrip_ptr_to_index_to_ptr | ptr→i→ptr roundtrip | PASS | ✅ PASS |
| test_page_ptr_valid_zero | 0 is a valid page ptr | PASS | ✅ PASS |
| test_page_index_valid_zero | 0 is a valid page index | PASS | ✅ PASS |
| test_page_index_valid_max | NUM_PAGES-1 is valid | PASS | ✅ PASS |
| test_page_ptr_valid_large | 4194304 is valid page ptr | PASS | ✅ PASS |
| test_page_index_2m_valid_0 | 0 is 2m-valid | PASS | ✅ PASS |
| test_page_index_2m_valid_512 | 512 is 2m-valid | PASS | ✅ PASS |
| test_page_index_2m_valid_1024 | 1024 is 2m-valid | PASS | ✅ PASS |
| test_page_index_1g_valid_0 | 0 is 1g-valid | PASS | ✅ PASS |
| test_page_ptr_2m_valid_0 | ptr=0 is 2m-valid | PASS | ✅ PASS |
| test_page_ptr_2m_valid_2m | ptr=0x200000 is 2m-valid | PASS | ✅ PASS |
| test_page_ptr_1g_valid_0 | ptr=0 is 1g-valid | PASS | ✅ PASS |
| test_page_index_truncate_2m_aligned | truncate(512)=512 | PASS | ✅ PASS |
| test_page_index_truncate_2m_unaligned | truncate(513)=512 | PASS | ✅ PASS |
| test_page_index_truncate_2m_zero | truncate(0)=0 | PASS | ✅ PASS |
| test_page_index_truncate_1g_zero | truncate_1g(0)=0 | PASS | ✅ PASS |
| test_merge_2m_valid | 0<1<0+0x200 is valid | PASS | ✅ PASS |
| test_merge_1g_valid | 0<1<0+0x40000 is valid | PASS | ✅ PASS |
| test_bitwise_present_mask | 0&1=0, 1&1=1 | PASS | ✅ PASS |
| test_bitwise_write_mask | 0&2=0, 2&2≠0 | PASS | ✅ PASS |
| test_bitwise_user_mask | 0&4=0, 4&4≠0 | PASS | ✅ PASS |
| test_bitwise_ps_mask | 0&0x80=0, 0x80&0x80≠0 | PASS | ✅ PASS |
| test_bitwise_mem_mask | MEM_MASK address extraction | PASS | ✅ PASS |
| test_page_entry_is_empty | Zero entry is empty | PASS | ✅ PASS |
| test_page_entry_not_empty_present | Present entry not empty | PASS | ✅ PASS |
| test_page_entry_not_empty_addr | Non-zero addr not empty | PASS | ✅ PASS |
| test_num_pages_value | NUM_PAGES == 2097152 | PASS | ✅ PASS |
| test_param_page_ptr_valid_alignment | Valid ptr implies aligned | PASS | ✅ PASS |
| test_param_2m_implies_valid | 2m-valid implies valid | PASS | ✅ PASS |
| test_param_1g_implies_valid | 1g-valid implies valid | PASS | ✅ PASS |
| test_param_ptr_2m_implies_alignment | 2m-valid ptr aligned | PASS | ✅ PASS |
| test_return_struct_switch_decision_values | SwitchDecision variants distinct | PASS | ✅ PASS |
| test_ret_value_type_error | RetValueType variants exist | PASS | ✅ PASS |
| test_page_state_variants | PageState variants exist | PASS | ✅ PASS |
| test_param_merge_2m_ordering | merge_2m implies i<j | PASS | ✅ PASS |
| test_param_merge_1g_ordering | merge_1g implies i<j | PASS | ✅ PASS |

**Result: 96 verified, 0 errors** — All correctness tests pass.

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| test_violate_unaligned_ptr_valid | Assert page_ptr_valid(1) — not 4k-aligned | FAIL | ✅ FAIL |
| test_violate_index_out_of_range | Assert page_index_valid(NUM_PAGES) — out of range | FAIL | ✅ FAIL |
| test_violate_roundtrip_no_valid_ptr | Roundtrip without page_ptr_valid precondition | FAIL | ✅ FAIL |
| test_violate_roundtrip_no_valid_index | Roundtrip without page_index_valid precondition | FAIL | ✅ FAIL |
| test_violate_merge_2m_equal | merge_2m with i==j (needs i<j) | FAIL | ✅ FAIL |
| test_violate_merge_2m_out_of_range | merge_2m with j==i+0x200 (needs j<i+0x200) | FAIL | ✅ FAIL |
| test_violate_merge_1g_equal | merge_1g with i==j | FAIL | ✅ FAIL |
| test_violate_ptr_too_large | Assert page_ptr_valid(7) — not aligned | FAIL | ✅ FAIL |

**Result: 43 verified, 8 errors** — All precondition violation tests correctly fail.

### Round 2: Overly Strong Postconditions

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| test_overly_strong_ptr_bound | page_ptr_valid → ptr<4096 (too tight) | FAIL | ✅ FAIL |
| test_overly_strong_index_bound | page_index_valid → i<1024 (too tight) | FAIL | ✅ FAIL |
| test_overly_strong_ptr2index_zero | ptr2index always == 0 (wrong) | FAIL | ✅ FAIL |
| test_overly_strong_2m_zero | 2m_valid → i==0 (wrong) | FAIL | ✅ FAIL |
| test_overly_strong_truncate_2m | truncate always == 0 (wrong) | FAIL | ✅ FAIL |
| test_overly_strong_merge_exact | merge_2m → j==i+1 (too specific) | FAIL | ✅ FAIL |
| test_overly_strong_2m_implies_1g | page_ptr_2m_valid → page_ptr_1g_valid | FAIL | ✅ FAIL |
| test_overly_strong_2m_index_implies_1g | page_index_2m_valid → page_index_1g_valid | FAIL | ✅ FAIL |

**Result: 43 verified, 8 errors** — All overly-strong tests correctly fail.

### Round 3: Negated/Contradicted Postconditions

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| test_negate_is_empty | Assert non-empty for zero entry | FAIL | ✅ FAIL |
| test_negate_not_empty | Assert empty for non-zero addr entry | FAIL | ✅ FAIL |
| test_negate_page_ptr_valid | Assert !page_ptr_valid(0) | FAIL | ✅ FAIL |
| test_negate_page_index_valid | Assert !page_index_valid(0) | FAIL | ✅ FAIL |
| test_negate_2m_valid | Assert !page_index_2m_valid(0) | FAIL | ✅ FAIL |
| test_negate_merge_2m | Assert !merge_2m(0,1) | FAIL | ✅ FAIL |
| test_negate_ptr_2m_valid | Assert !page_ptr_2m_valid(0) | FAIL | ✅ FAIL |
| test_negate_ptr_1g_valid | Assert !page_ptr_1g_valid(0) | FAIL | ✅ FAIL |

**Result: 43 verified, 8 errors** — All negated-postcondition tests correctly fail.

### Round 4: Wrong Specific Values

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| test_wrong_ptr2index_value | ptr2index(4096)==0 (should be 1) | FAIL | ✅ FAIL |
| test_wrong_index2ptr_value | index2ptr(1)==1 (should be 4096) | FAIL | ✅ FAIL |
| test_wrong_truncate_2m_value | truncate(513)==0 (should be 512) | FAIL | ✅ FAIL |
| test_wrong_truncate_2m_value_2 | truncate(1023)==1024 (should be 512) | FAIL | ✅ FAIL |
| test_wrong_ptr2index_8192 | ptr2index(8192)==3 (should be 2) | FAIL | ✅ FAIL |
| test_wrong_index2ptr_2 | index2ptr(2)==8193 (should be 8192) | FAIL | ✅ FAIL |
| test_wrong_num_pages | NUM_PAGES==1048576 (should be 2097152) | FAIL | ✅ FAIL |
| test_wrong_ptr2index_zero | ptr2index(0)==1 (should be 0) | FAIL | ✅ FAIL |

**Result: 43 verified, 8 errors** — All wrong-value tests correctly fail.

### Round 5: Cross-Function Misuse & Edge Cases

| Test Name | What It Tests | Expected | Actual |
|-----------|--------------|----------|--------|
| test_misuse_valid_implies_2m | page_index_valid → 2m_valid (wrong) | FAIL | ✅ FAIL |
| test_misuse_valid_implies_1g | page_index_valid → 1g_valid (wrong) | FAIL | ✅ FAIL |
| test_misuse_ptr_valid_implies_2m | page_ptr_valid → 2m_valid (wrong) | FAIL | ✅ FAIL |
| test_misuse_truncate_preserves | truncate(1)==1 (wrong) | FAIL | ✅ FAIL |
| test_misuse_different_ptrs_same_index | ptr2index(0)==ptr2index(4096) (wrong) | FAIL | ✅ FAIL |
| test_misuse_1g_implies_nonzero | 1g_valid → i>0 (wrong, 0 is valid) | FAIL | ✅ FAIL |
| test_misuse_merge_1g_implies_2m | merge_1g → merge_2m (wrong direction) | FAIL | ✅ FAIL |
| test_misuse_index2ptr_identity | index2ptr(1)==1 (wrong, should be 4096) | FAIL | ✅ FAIL |

**Result: 43 verified, 8 errors** — All cross-function misuse tests correctly fail.

## Overall Assessment

### Correctness: ✅ CONFIRMED
All 43 correctness tests pass, verifying that the spec functions produce correct results for valid inputs including boundary values, concrete values, and parameterized inputs.

### Completeness: ✅ CONFIRMED
All 40 completeness tests (8 per round × 5 rounds) correctly fail, confirming that the specs are tight enough to reject:
- Invalid inputs (precondition violations)
- Overly strong claims (tighter bounds than guaranteed)
- Contradicted postconditions (negated assertions)
- Wrong concrete values
- Incorrect cross-function relationships

### Notes
- The main `syscall_io_mmap` function has `requires` but no explicit `ensures` clause, so we tested the spec functions it relies on.
- Complex types (`Kernel`, `ProcessManager`, etc.) use `closed spec fn wf()` with `external_body`, making it impossible to construct valid instances in proof context. Testing focused on standalone spec functions operating on primitive types.
- Bitwise spec functions (`usize2present`, `MEM_valid`, etc.) use `usize` bitwise operations that require `by(bit_vector)` reasoning. Since `by(bit_vector)` in Verus only works on fixed-width types (`u64`, etc.), correctness tests for these used `u64` bit_vector assertions on the underlying mask constants instead.
- No spec gaps were found — all specs correctly accept valid inputs and reject invalid claims.
