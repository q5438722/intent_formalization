# Specification Testing Summary

## File Under Test

**`kernel__syscall_new_thread__impl0__syscall_new_thread.rs`** — An OS kernel syscall implementation for creating new threads. The key function `syscall_new_thread` takes a `Kernel` and `ThreadPtr`, checks resource availability (thread list capacity, memory quota, scheduler capacity, free pages), and either creates a new thread or returns an error. The spec guarantees:
- Well-formedness (`wf()`) is preserved
- The return value is an error **if and only if** `syscall_new_thread_requirement` is false

Key spec functions tested: `syscall_new_thread_requirement`, `spec_is_error`, `Quota::spec_subtract_mem_4k`, `PageEntry::is_empty`, `spec_page_ptr2page_index`, `spec_page_index2page_ptr`, `page_ptr_valid`, `page_index_valid`.

---

## Correctness Results (should all PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_is_error_with_error` | SyscallReturnStruct with Error is_error | PASS | ✅ PASS |
| 2 | `test_is_error_with_success_usize` | SuccessUsize is NOT is_error | PASS | ✅ PASS |
| 3 | `test_is_error_with_else` | Else is NOT is_error | PASS | ✅ PASS |
| 4 | `test_is_error_with_error_no_quota` | ErrorNoQuota is NOT is_error (spec only matches Error) | PASS | ✅ PASS |
| 5 | `test_is_error_with_cpu_idle` | CpuIdle is NOT is_error | PASS | ✅ PASS |
| 6 | `test_quota_subtract_mem_4k_basic` | Basic quota subtraction (10-1=9) | PASS | ✅ PASS |
| 7 | `test_quota_subtract_mem_4k_zero` | Subtract 0 from quota | PASS | ✅ PASS |
| 8 | `test_quota_subtract_mem_4k_large` | Subtract 50 from 100 | PASS | ✅ PASS |
| 9 | `test_page_entry_is_empty` | All-zero PageEntry is empty | PASS | ✅ PASS |
| 10 | `test_page_entry_not_empty_present` | PageEntry with present=true NOT empty | PASS | ✅ PASS |
| 11 | `test_page_entry_not_empty_addr` | PageEntry with nonzero addr NOT empty | PASS | ✅ PASS |
| 12 | `test_page_ptr2index_4096` | ptr2index(4096) == 1 | PASS | ✅ PASS |
| 13 | `test_page_ptr2index_zero` | ptr2index(0) == 0 | PASS | ✅ PASS |
| 14 | `test_page_ptr2index_8192` | ptr2index(8192) == 2 | PASS | ✅ PASS |
| 15 | `test_page_index2ptr_1` | index2ptr(1) == 4096 | PASS | ✅ PASS |
| 16 | `test_page_index2ptr_zero` | index2ptr(0) == 0 | PASS | ✅ PASS |
| 17 | `test_page_ptr_valid_aligned` | page_ptr_valid(4096) | PASS | ✅ PASS |
| 18 | `test_page_ptr_valid_zero` | page_ptr_valid(0) | PASS | ✅ PASS |
| 19 | `test_page_ptr_invalid_unaligned` | !page_ptr_valid(4097) | PASS | ✅ PASS |
| 20 | `test_page_index_valid_1` | page_index_valid(1) | PASS | ✅ PASS |
| 21 | `test_page_index_valid_zero` | page_index_valid(0) | PASS | ✅ PASS |
| 22 | `test_page_index_valid_max_minus_1` | page_index_valid(NUM_PAGES-1) | PASS | ✅ PASS |
| 23 | `test_page_index_invalid_max` | !page_index_valid(NUM_PAGES) | PASS | ✅ PASS |
| 24 | `test_roundtrip_ptr_to_index_to_ptr` | index2ptr(ptr2index(4096)) == 4096 | PASS | ✅ PASS |
| 25 | `test_roundtrip_index_to_ptr_to_index` | ptr2index(index2ptr(1)) == 1 | PASS | ✅ PASS |
| 26 | `test_page_index_2m_valid` | 2m-valid(512) | PASS | ✅ PASS |
| 27 | `test_page_index_2m_invalid` | !2m-valid(1) | PASS | ✅ PASS |
| 28 | `test_biconditional_req_false_implies_error` | req==false ⟹ is_error | PASS | ✅ PASS |
| 29 | `test_biconditional_req_true_implies_no_error` | req==true ⟹ !is_error | PASS | ✅ PASS |
| 30 | `test_biconditional_error_implies_req_false` | is_error ⟹ req==false | PASS | ✅ PASS |
| 31 | `test_requirement_false_thread_list_full` | thread_list_full ⟹ req==false | PASS | ✅ PASS |
| 32 | `test_requirement_false_no_quota` | no_quota ⟹ req==false | PASS | ✅ PASS |
| 33 | `test_requirement_false_scheduler_full` | scheduler_full ⟹ req==false | PASS | ✅ PASS |
| 34 | `test_requirement_false_no_free_pages` | no_free_pages ⟹ req==false | PASS | ✅ PASS |
| 35 | `test_requirement_true_all_met` | all conditions met ⟹ req==true | PASS | ✅ PASS |
| 36 | `test_wf_preserved` | wf() is preserved (postcondition) | PASS | ✅ PASS |

**Result: 36/36 passed** — `verification results:: 78 verified, 0 errors`

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_req_true_despite_thread_list_full` | Asserts req==true when thread list is full | FAIL | ✅ FAIL |
| 2 | `test_wrong_req_true_despite_no_quota` | Asserts req==true when no mem_4k quota | FAIL | ✅ FAIL |
| 3 | `test_wrong_req_true_despite_scheduler_full` | Asserts req==true when scheduler is full | FAIL | ✅ FAIL |
| 4 | `test_wrong_req_true_despite_no_free_pages` | Asserts req==true when no free pages | FAIL | ✅ FAIL |
| 5 | `test_wrong_no_error_despite_req_false` | Asserts !is_error when req is false | FAIL | ✅ FAIL |

**Result: 5/5 failed** — `verification results:: 42 verified, 5 errors`

### Round 2: Overly Strong Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_too_strong_req_only_free_pages` | Asserts req true with only free_pages > 0 | FAIL | ✅ FAIL |
| 2 | `test_too_strong_quota_subtract_ignore_mem_2m` | Asserts subtract works even when mem_2m differs | FAIL | ✅ FAIL |
| 3 | `test_too_strong_quota_subtract_ignore_pcid` | Asserts subtract works even when pcid differs | FAIL | ✅ FAIL |
| 4 | `test_too_strong_page_ptr_bound` | Asserts page_ptr_valid ⟹ ptr < 0x10000 | FAIL | ✅ FAIL |
| 5 | `test_too_strong_page_index_bound` | Asserts page_index_valid ⟹ idx < 100 | FAIL | ✅ FAIL |

**Result: 5/5 failed** — `verification results:: 42 verified, 5 errors`

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_negate_is_error` | Asserts Error is NOT is_error | FAIL | ✅ FAIL |
| 2 | `test_negate_is_not_error` | Asserts SuccessUsize IS is_error | FAIL | ✅ FAIL |
| 3 | `test_negate_quota_subtract` | Asserts valid subtract does NOT hold | FAIL | ✅ FAIL |
| 4 | `test_negate_page_entry_empty` | Asserts present=true entry IS empty | FAIL | ✅ FAIL |
| 5 | `test_negate_requirement_all_met` | Asserts req==false when all conditions met | FAIL | ✅ FAIL |

**Result: 5/5 failed** — `verification results:: 42 verified, 5 errors`

### Round 4: Wrong Specific Values

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_ptr2index_8192` | Asserts ptr2index(8192)==1 (should be 2) | FAIL | ✅ FAIL |
| 2 | `test_wrong_index2ptr_2` | Asserts index2ptr(2)==4096 (should be 8192) | FAIL | ✅ FAIL |
| 3 | `test_wrong_ptr2index_4096` | Asserts ptr2index(4096)==0 (should be 1) | FAIL | ✅ FAIL |
| 4 | `test_wrong_index2ptr_0` | Asserts index2ptr(0)==4096 (should be 0) | FAIL | ✅ FAIL |
| 5 | `test_wrong_quota_subtract_value` | Asserts subtract(10,1)==8 (should be 9) | FAIL | ✅ FAIL |

**Result: 5/5 failed** — `verification results:: 42 verified, 5 errors`

### Round 5: Cross-function Misuse & Edge Cases

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_wrong_roundtrip` | Asserts ptr2index(index2ptr(1))==2 (should be 1) | FAIL | ✅ FAIL |
| 2 | `test_wrong_requirement_same_for_all_threads` | Asserts req(t1) ⟹ req(t2) for arbitrary threads | FAIL | ✅ FAIL |
| 3 | `test_wrong_error_means_not_wf` | Asserts error ⟹ !wf (wf is always preserved) | FAIL | ✅ FAIL |
| 4 | `test_wrong_zero_not_valid` | Asserts !page_ptr_valid(0) (0 IS valid) | FAIL | ✅ FAIL |
| 5 | `test_wrong_index_ptr_equivalence` | Asserts page_index_valid ⟹ page_ptr_valid | FAIL | ✅ FAIL |

**Result: 5/5 failed** — `verification results:: 42 verified, 5 errors`

---

## Overall Assessment

### Correctness: ✅ PASS
All 36 correctness tests verified successfully. The specs correctly describe:
- The `syscall_new_thread_requirement` function accurately captures the four conditions for thread creation
- The biconditional `req==false <==> is_error()` correctly links the requirement to the return value
- `spec_is_error` correctly identifies only the `Error` variant (not `ErrorNoQuota`, `CpuIdle`, etc.)
- `Quota::spec_subtract_mem_4k` correctly validates all quota fields
- Page pointer/index conversion functions are correct and invertible

### Completeness: ✅ PASS
All 25 completeness tests failed as expected. The specs are tight enough to reject:
- Assertions that ignore individual conditions of `syscall_new_thread_requirement`
- Overly strong bounds on page pointer/index values
- Claims that quota subtraction ignores non-mem_4k fields
- Negated postconditions
- Wrong specific arithmetic values
- Cross-function misuse (e.g., different threads having same requirement, index/ptr equivalence)

### Spec Gaps Found: None
No unexpected passes in completeness tests. The specifications appear both correct and complete for the properties tested.
