# Summary: Specification Testing for `syscall_new_proc_with_endpoint_iommu`

## File Under Test

`kernel__syscall_new_proc_with_iommu__impl0__syscall_new_proc_with_endpoint_iommu.rs`

This file defines a kernel syscall (`syscall_new_proc_with_endpoint_iommu`) for creating a new process with an endpoint and IOMMU table. It includes:

- **Kernel struct** with `wf()` invariant composing memory manager, page allocator, and process manager well-formedness
- **ProcessManager** methods: `thread_inv`, `process_inv`, `new_proc_with_endpoint_iommu` — proof functions establishing invariants on threads, processes, containers, and endpoints
- **MemoryManager** methods: `alloc_page_table`, `alloc_iommu_table` — allocate PCID/IOID resources
- **PageAllocator** methods: `alloc_page_4k` — allocate 4K pages
- **Helper specs**: `seq_push_lemma`, `SyscallReturnStruct::NoSwitchNew`, `Quota::spec_subtract_mem_4k`, page utility functions
- **Note**: `syscall_new_proc_with_endpoint_iommu` has **empty ensures** — it validates inputs and delegates to internal methods but makes no guarantees in its postcondition

---

## Correctness Results

**File**: `correctness_tests.rs`
**Result**: **68 verified, 0 errors** (25 test functions + 43 base file items)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_seq_push_contains_pushed_value` | Pushed value is contained in result sequence | PASS | PASS |
| 2 | `test_seq_push_preserves_existing` | Existing elements preserved after push | PASS | PASS |
| 3 | `test_seq_push_non_member_stays_absent` | Non-member remains absent after push of different value | PASS | PASS |
| 4 | `test_seq_push_empty_seq` | Push onto empty seq contains the pushed value | PASS | PASS |
| 5 | `test_seq_push_lemma_with_i32` | seq_push_lemma works with i32 type parameter | PASS | PASS |
| 6 | `test_page_ptr_valid_aligned` | 0x1000 is a valid page pointer | PASS | PASS |
| 7 | `test_page_ptr_valid_larger` | 0x2000 is a valid page pointer | PASS | PASS |
| 8 | `test_page_ptr_valid_zero` | 0 is a valid page pointer | PASS | PASS |
| 9 | `test_page_index_valid_zero` | Index 0 is valid | PASS | PASS |
| 10 | `test_page_index_valid_max_minus_1` | Index NUM_PAGES-1 (2097151) is valid | PASS | PASS |
| 11 | `test_page_index_valid_mid` | Index 1024 is valid | PASS | PASS |
| 12 | `test_page_ptr_to_index_basic` | ptr/4096 conversion for 0x1000, 0x2000, 0 | PASS | PASS |
| 13 | `test_page_index_to_ptr_basic` | i*4096 conversion for 0, 1, 2 | PASS | PASS |
| 14 | `test_page_ptr_index_roundtrip` | Parameterized: index-ptr-index roundtrip | PASS | PASS |
| 15 | `test_page_entry_is_empty_true` | PageEntry with all-zero fields is empty | PASS | PASS |
| 16 | `test_page_entry_not_empty_present` | PageEntry with present=true is not empty | PASS | PASS |
| 17 | `test_page_entry_not_empty_addr` | PageEntry with non-zero addr is not empty | PASS | PASS |
| 18 | `test_quota_subtract_basic` | 100 - 2 = 98 with matching fields | PASS | PASS |
| 19 | `test_quota_subtract_zero` | Subtracting 0 preserves all fields | PASS | PASS |
| 20 | `test_quota_subtract_all` | Subtracting entire mem_4k yields 0 | PASS | PASS |
| 21 | `test_page_ptr_2m_valid` | 0x200000 (2MB) is valid 2m page ptr | PASS | PASS |
| 22 | `test_page_index_2m_valid` | Indices 0 and 512 are 2m-valid | PASS | PASS |
| 23 | `test_rf_counter_full_semantics` | usize::MAX equals usize::MAX (full counter) | PASS | PASS |
| 24 | `test_page_index_truncate_2m` | Truncation to 512-boundary works correctly | PASS | PASS |
| 25 | `test_page_index_merge_2m_valid` | Merge validity: i < j < i + 0x200 | PASS | PASS |

---

## Completeness Results

### Round 1: Precondition Violations

**File**: `completeness_round1.rs`
**Result**: **43 verified, 5 errors** (all 5 tests fail as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | `test_precond_violation_unaligned_page_valid` | Assert page_ptr_valid(0x1001) - unaligned | FAIL | FAIL |
| 2 | `test_precond_violation_page_index_at_boundary` | Assert page_index_valid(NUM_PAGES) - out of range | FAIL | FAIL |
| 3 | `test_precond_violation_merge_2m_j_leq_i` | Assert merge_2m_valid(10, 5) - j <= i violates i < j | FAIL | FAIL |
| 4 | `test_precond_violation_quota_wrong_mem2m` | Quota subtract with mismatched mem_2m fields | FAIL | FAIL |
| 5 | `test_precond_violation_page_index_huge` | Assert page_index_valid(usize::MAX) - way out of range | FAIL | FAIL |

### Round 2: Overly Strong Postconditions

**File**: `completeness_round2.rs`
**Result**: **43 verified, 5 errors** (all 5 tests fail as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | `test_overly_strong_page_ptr_bound` | Assert valid page ptr < 0x100000 - too tight | FAIL | FAIL |
| 2 | `test_overly_strong_index_always_zero` | Assert ptr-to-index always equals 0 - too strong | FAIL | FAIL |
| 3 | `test_overly_strong_2m_index_bound` | Assert 2m-valid index < 512 - too tight | FAIL | FAIL |
| 4 | `test_overly_strong_even_index` | Assert page index always even - not guaranteed | FAIL | FAIL |
| 5 | `test_overly_strong_index_small` | Assert page index < 1024 - too tight | FAIL | FAIL |

### Round 3: Negated/Contradicted Postconditions

**File**: `completeness_round3.rs`
**Result**: **43 verified, 5 errors** (all 5 tests fail as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | `test_negate_seq_push_contains` | Negate: pushed value NOT contained | FAIL | FAIL |
| 2 | `test_negate_page_entry_is_empty` | Negate: all-zero PageEntry is NOT empty | FAIL | FAIL |
| 3 | `test_negate_page_ptr_valid` | Negate: 0x1000 is NOT valid page ptr | FAIL | FAIL |
| 4 | `test_negate_page_index_valid` | Negate: index 100 is NOT valid | FAIL | FAIL |
| 5 | `test_negate_quota_subtract` | Negate: correct subtraction is false | FAIL | FAIL |

### Round 4: Wrong Specific Values

**File**: `completeness_round4.rs`
**Result**: **43 verified, 5 errors** (all 5 tests fail as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | `test_wrong_value_ptr2index` | 0x2000/4096 = 3 (wrong, should be 2) | FAIL | FAIL |
| 2 | `test_wrong_value_index2ptr` | 3*4096 = 0x4000 (wrong, should be 0x3000) | FAIL | FAIL |
| 3 | `test_wrong_value_truncate` | truncate_2m(600) = 1024 (wrong, should be 512) | FAIL | FAIL |
| 4 | `test_wrong_value_merge_boundary` | merge_2m_valid(0, 0x200) - j=i+0x200 is NOT valid | FAIL | FAIL |
| 5 | `test_wrong_value_quota_amount` | 50 - 3 = 45 (wrong, should be 47) | FAIL | FAIL |

### Round 5: Cross-function Misuse and Edge Cases

**File**: `completeness_round5.rs`
**Result**: **43 verified, 5 errors** (all 5 tests fail as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|--------------|----------|--------|
| 1 | `test_cross_fn_4k_implies_2m` | page_ptr_valid does NOT imply page_ptr_2m_valid | FAIL | FAIL |
| 2 | `test_cross_fn_2m_implies_1g` | page_ptr_2m_valid does NOT imply page_ptr_1g_valid | FAIL | FAIL |
| 3 | `test_cross_fn_roundtrip_unaligned` | ptr-idx-ptr roundtrip fails for unaligned ptr | FAIL | FAIL |
| 4 | `test_edge_case_contradiction` | Assert PageEntry is empty AND addr != 0 - contradiction | FAIL | FAIL |
| 5 | `test_cross_fn_unrelated_quotas` | Two unrelated quotas don't satisfy subtract with k=0 | FAIL | FAIL |

---

## Overall Assessment

### Correctness
**All 25 correctness tests pass.** The tested specs are correct:
- `seq_push_lemma` correctly specifies sequence push containment properties
- Page utility specs (`page_ptr_valid`, `page_index_valid`, `spec_page_ptr2page_index`, etc.) correctly define validity and conversion
- `PageEntry::is_empty` correctly identifies empty entries
- `Quota::spec_subtract_mem_4k` correctly models quota subtraction
- `spec_page_index_merge_2m_vaild` and `spec_page_index_truncate_2m` behave as specified

### Completeness
**All 25 completeness tests fail as expected.** The specs reject:
- Invalid inputs (unaligned pointers, out-of-range indices, mismatched quota fields)
- Overly strong claims (tighter bounds than actually hold)
- Negations of true properties
- Wrong concrete values
- Invalid cross-function relationships (4k implies 2m, unaligned roundtrips)

### Spec Gaps Identified
1. **`syscall_new_proc_with_endpoint_iommu` has empty `ensures`**: The main syscall function makes no guarantees about its return value or the state of `self` after execution. This is a significant spec gap - callers cannot reason about what the syscall does.
2. **Many `closed spec fn wf()` predicates**: Key well-formedness specs for `ProcessManager`, `PageTable`, `StaticLinkedList`, etc. are opaque (`closed`), limiting external testability of complex invariants.
3. **`thread_inv` uses `unimplemented!()` without `external_body`**: This is technically unsound (equivalent to `assume(false)`) - the proof body does not actually prove the invariant.
