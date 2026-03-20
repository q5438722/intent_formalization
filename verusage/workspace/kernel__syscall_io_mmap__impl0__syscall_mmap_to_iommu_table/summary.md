# Summary: Specification Testing for `kernel__syscall_io_mmap__impl0__syscall_mmap_to_iommu_table.rs`

## File Under Test

Defines the `syscall_io_mmap` kernel function for mapping I/O memory in an IOMMU table, along with supporting types, spec functions, and proof invariants for a verified microkernel. Key specs include:

- **`SyscallReturnStruct::NoSwitchNew`**: Constructs a return struct with specified error code, no pcid/cr3, and NoSwitch decision
- **Page utility specs**: `page_ptr_valid`, `page_index_valid`, `spec_page_ptr2page_index`, `spec_page_index2page_ptr`, truncation, 2m/1g validity
- **Bit-level conversion specs**: `usize2page_entry_perm`, `usize2page_entry`, `usize2pa`
- **PageEntry::is_empty**: Structural predicate checking all fields are zero/false
- **Proof invariants**: `thread_inv`, `process_inv`, `fold_mem_4k_lemma` (all external_body)

---

## Correctness Results

**File**: `correctness_tests.rs` — **69 verified, 0 errors** ✅

| Test Name | Description | Expected | Actual |
|-----------|-------------|----------|--------|
| `test_page_ptr_valid_zero` | page_ptr_valid(0) is true | PASS | ✅ PASS |
| `test_page_ptr_valid_4096` | page_ptr_valid(4096) is true | PASS | ✅ PASS |
| `test_page_ptr_valid_8192` | page_ptr_valid(8192) is true | PASS | ✅ PASS |
| `test_page_ptr_invalid_unaligned` | !page_ptr_valid(4095) | PASS | ✅ PASS |
| `test_page_ptr_invalid_1` | !page_ptr_valid(1) | PASS | ✅ PASS |
| `test_page_index_valid_zero` | page_index_valid(0) | PASS | ✅ PASS |
| `test_page_index_valid_one` | page_index_valid(1) | PASS | ✅ PASS |
| `test_page_index_valid_max_minus_1` | page_index_valid(NUM_PAGES-1) | PASS | ✅ PASS |
| `test_page_index_invalid_num_pages` | !page_index_valid(NUM_PAGES) | PASS | ✅ PASS |
| `test_page_ptr2index_zero` | spec_page_ptr2page_index(0) == 0 | PASS | ✅ PASS |
| `test_page_ptr2index_4096` | spec_page_ptr2page_index(4096) == 1 | PASS | ✅ PASS |
| `test_page_ptr2index_8192` | spec_page_ptr2page_index(8192) == 2 | PASS | ✅ PASS |
| `test_page_index2ptr_zero` | spec_page_index2page_ptr(0) == 0 | PASS | ✅ PASS |
| `test_page_index2ptr_one` | spec_page_index2page_ptr(1) == 4096 | PASS | ✅ PASS |
| `test_page_index2ptr_two` | spec_page_index2page_ptr(2) == 8192 | PASS | ✅ PASS |
| `test_page_index_2m_valid_zero` | page_index_2m_valid(0) | PASS | ✅ PASS |
| `test_page_index_2m_valid_512` | page_index_2m_valid(512) | PASS | ✅ PASS |
| `test_page_index_2m_valid_1024` | page_index_2m_valid(1024) | PASS | ✅ PASS |
| `test_page_index_2m_invalid_1` | !page_index_2m_valid(1) | PASS | ✅ PASS |
| `test_page_index_2m_invalid_511` | !page_index_2m_valid(511) | PASS | ✅ PASS |
| `test_page_index_1g_valid_zero` | page_index_1g_valid(0) | PASS | ✅ PASS |
| `test_page_index_1g_invalid_num_pages` | !page_index_1g_valid(NUM_PAGES) | PASS | ✅ PASS |
| `test_page_ptr_2m_valid_zero` | page_ptr_2m_valid(0) | PASS | ✅ PASS |
| `test_page_ptr_2m_valid_2m` | page_ptr_2m_valid(0x200000) | PASS | ✅ PASS |
| `test_page_ptr_2m_invalid_4096` | !page_ptr_2m_valid(4096) | PASS | ✅ PASS |
| `test_page_ptr_1g_valid_zero` | page_ptr_1g_valid(0) | PASS | ✅ PASS |
| `test_truncate_2m_zero` | truncate_2m(0) == 0 | PASS | ✅ PASS |
| `test_truncate_2m_512` | truncate_2m(512) == 512 | PASS | ✅ PASS |
| `test_truncate_2m_513` | truncate_2m(513) == 512 | PASS | ✅ PASS |
| `test_truncate_2m_1023` | truncate_2m(1023) == 512 | PASS | ✅ PASS |
| `test_truncate_1g_zero` | truncate_1g(0) == 0 | PASS | ✅ PASS |
| `test_page_entry_is_empty` | Zero entry is_empty | PASS | ✅ PASS |
| `test_page_entry_not_empty_present` | Present entry !is_empty | PASS | ✅ PASS |
| `test_page_entry_not_empty_addr` | Nonzero addr !is_empty | PASS | ✅ PASS |
| `test_page_entry_not_empty_write` | Write=true !is_empty | PASS | ✅ PASS |
| `test_param_page_index_valid` | ∀ valid i: page_index_valid(i) | PASS | ✅ PASS |
| `test_param_page_ptr_valid` | ∀ aligned ptr: page_ptr_valid(ptr) | PASS | ✅ PASS |
| `test_param_page_index_2m_implies_valid` | 2m_valid ⇒ valid | PASS | ✅ PASS |
| `test_no_switch_new_error` | NoSwitchNew(Error) ensures | PASS | ✅ PASS |
| `test_no_switch_new_no_quota` | NoSwitchNew(ErrorNoQuota) ensures | PASS | ✅ PASS |
| `test_no_switch_new_va_in_use` | NoSwitchNew(ErrorVaInUse) ensures | PASS | ✅ PASS |
| `test_usize2page_entry_perm_zero` | usize2page_entry_perm(0) all false | PASS | ✅ PASS |
| `test_usize2page_entry_zero` | usize2page_entry(0) all zero/false | PASS | ✅ PASS |
| `test_usize2pa_mem_valid` | usize2pa(0) is MEM_valid | PASS | ✅ PASS |
| `test_usize2pa_nonzero_mem_valid` | usize2pa(0x1000) is MEM_valid | PASS | ✅ PASS |
| `test_usize2pa_large_mem_valid` | usize2pa(0xFFFF) is MEM_valid | PASS | ✅ PASS |
| `test_exec_page_ptr2index_zero` | page_ptr2page_index(0) matches spec | PASS | ✅ PASS |
| `test_exec_page_ptr2index_4096` | page_ptr2page_index(4096) matches spec | PASS | ✅ PASS |
| `test_exec_page_index2ptr_zero` | page_index2page_ptr(0) matches spec | PASS | ✅ PASS |
| `test_exec_page_index2ptr_one` | page_index2page_ptr(1) matches spec | PASS | ✅ PASS |
| `test_exec_page_index2ptr_100` | page_index2page_ptr(100) matches spec | PASS | ✅ PASS |

---

## Completeness Results

### Round 1: Precondition Violations — **5 errors** ✅

| Test Name | What It Tests | Expected | Actual |
|-----------|---------------|----------|--------|
| `test_precondition_violation_unaligned_ptr` | page_ptr2page_index(4095): ptr not aligned | FAIL | ✅ FAIL |
| `test_precondition_violation_odd_ptr` | page_ptr2page_index(1): ptr not aligned | FAIL | ✅ FAIL |
| `test_precondition_violation_index_too_large` | page_index2page_ptr(NUM_PAGES): index out of range | FAIL | ✅ FAIL |
| `test_precondition_violation_index_way_too_large` | page_index2page_ptr(NUM_PAGES+1000): index out of range | FAIL | ✅ FAIL |
| `test_precondition_violation_partial_align` | page_ptr2page_index(2048): ptr not page-aligned | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions — **5 errors** ✅

| Test Name | What It Tests | Expected | Actual |
|-----------|---------------|----------|--------|
| `test_overly_strong_always_zero` | Assert page_ptr2page_index(4096) == 0 | FAIL | ✅ FAIL |
| `test_overly_strong_ptr_always_zero` | Assert page_index2page_ptr(1) == 0 | FAIL | ✅ FAIL |
| `test_overly_strong_tight_bound` | Assert ptr2index < 10 for any valid ptr | FAIL | ✅ FAIL |
| `test_overly_strong_perm_always_not_present` | Assert perm.present == false for any v | FAIL | ✅ FAIL |
| `test_overly_strong_valid_implies_small` | Assert page_ptr_valid ⇒ ptr < 4096 | FAIL | ✅ FAIL |

### Round 3: Negated Postconditions — **5 errors** ✅

| Test Name | What It Tests | Expected | Actual |
|-----------|---------------|----------|--------|
| `test_negated_no_switch_has_pcid` | Assert NoSwitchNew has Some pcid (contradicts is_None) | FAIL | ✅ FAIL |
| `test_negated_no_switch_is_switch` | Assert switch_decision == Switch (contradicts NoSwitch) | FAIL | ✅ FAIL |
| `test_negated_no_switch_has_cr3` | Assert cr3 is Some (contradicts is_None) | FAIL | ✅ FAIL |
| `test_negated_usize2pa_not_mem_valid` | Assert !MEM_valid(usize2pa(0)) | FAIL | ✅ FAIL |
| `test_negated_page_ptr2index_wrong` | Assert idx != spec result | FAIL | ✅ FAIL |

### Round 4: Wrong Concrete Values — **7 errors** ✅

| Test Name | What It Tests | Expected | Actual |
|-----------|---------------|----------|--------|
| `test_wrong_value_ptr2index` | Assert ptr2index(4096) == 2 (should be 1) | FAIL | ✅ FAIL |
| `test_wrong_value_index2ptr` | Assert index2ptr(1) == 8192 (should be 4096) | FAIL | ✅ FAIL |
| `test_wrong_value_ptr_valid_unaligned` | Assert page_ptr_valid(4095) | FAIL | ✅ FAIL |
| `test_wrong_value_index_valid_max` | Assert page_index_valid(NUM_PAGES) | FAIL | ✅ FAIL |
| `test_wrong_value_zero_invalid` | Assert !page_ptr_valid(0) (0 IS valid) | FAIL | ✅ FAIL |
| `test_wrong_value_truncate_2m` | Assert truncate_2m(513) == 0 (should be 512) | FAIL | ✅ FAIL |
| `test_wrong_value_page_entry_empty` | Assert entry with nonzero addr is empty | FAIL | ✅ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases — **5 errors** ✅

| Test Name | What It Tests | Expected | Actual |
|-----------|---------------|----------|--------|
| `test_cross_function_swap` | Assert ptr2index(i) == index2ptr(i) (they're inverses) | FAIL | ✅ FAIL |
| `test_cross_index_valid_implies_2m` | Assert page_index_valid ⇒ page_index_2m_valid | FAIL | ✅ FAIL |
| `test_cross_ptr_valid_implies_2m` | Assert page_ptr_valid ⇒ page_ptr_2m_valid | FAIL | ✅ FAIL |
| `test_cross_truncate_is_identity` | Assert truncate_2m(i) == i for any valid i | FAIL | ✅ FAIL |
| `test_cross_no_switch_wrong_error_code` | Assert error_code is Error when ErrorNoQuota passed | FAIL | ✅ FAIL |

---

## Overall Assessment

### Correctness: ✅ All 69 tests PASS
The specs correctly describe the intended behavior. All open spec functions compute expected values for concrete inputs, parameterized tests confirm universal properties, and external_body function ensures are consistent.

### Completeness: ✅ All 27 tests FAIL as expected
The specs are tight enough to reject:
- Precondition violations (unaligned pointers, out-of-range indices)
- Overly strong claims (tighter bounds than guaranteed)
- Negated postconditions (opposite of ensures clauses)
- Wrong concrete values (incorrect arithmetic)
- Cross-function confusion (confusing inverse functions, missing alignment requirements)

### Spec Gaps Found: None
No completeness tests unexpectedly passed. The tested specs are both correct and complete for the properties examined.

### Notes
- The `syscall_io_mmap` function has no `ensures` clause, so its postconditions cannot be directly tested. Testing focused on the supporting specs used by this function.
- Proof functions `thread_inv`, `process_inv`, and `fold_mem_4k_lemma` are `external_body` and require constructing valid `ProcessManager`/`Kernel` objects (with many closed `wf()` predicates), making direct testing infeasible.
- The `page_index_1g_valid` negative tests with small concrete values (1, 512) could not be verified due to SMT solver limitations with modular arithmetic on `(512 * 512) as usize`.
