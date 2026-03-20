# Summary: Spec Testing for `kernel__create_and_map_pages__impl0__alloc_and_map.rs`

## File Under Test

This file implements the `alloc_and_map` operation for a verified OS kernel. Key components:
- **`PageAllocator::alloc_and_map_4k`** — allocates and maps a 4k page (external_body)
- **`Kernel::alloc_and_map`** — orchestrates page allocation at kernel level (verified body)
- **`va_lemma`** — proves properties about virtual address ↔ index conversions (external_body proof fn)
- **`ProcessManager::pcid_unique`** — proves PCID uniqueness across processes (external_body proof fn)
- **`MemoryManager::pagetable_map_4k_page`** — maps a 4k page in page table (external_body)
- Numerous spec functions: `page_ptr_valid`, `spec_va_4k_valid`, `Quota` operations, `PageEntry::is_empty`, etc.

---

## Correctness Results (should PASS)

**File**: `correctness_tests.rs`
**Result**: `61 verified, 0 errors` (43 original + 18 tests)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_va_lemma_4k_bounds` | 4k valid VA ⇒ all indices in [0, 512) | PASS | ✅ PASS |
| 2 | `test_va_lemma_2m_bounds` | 2m valid VA ⇒ indices in [0, 512), l1 == 0 | PASS | ✅ PASS |
| 3 | `test_va_lemma_1g_bounds` | 1g valid VA ⇒ indices bounded, l1 == l2 == 0 | PASS | ✅ PASS |
| 4 | `test_va_lemma_index_equality` | Equal indices ⇒ equal VAs | PASS | ✅ PASS |
| 5 | `test_va_lemma_index_injectivity` | Different valid indices ⇒ different VAs | PASS | ✅ PASS |
| 6 | `test_va_lemma_valid_indices_4k` | Valid indices ⇒ va_4k_valid(index2va(...)) | PASS | ✅ PASS |
| 7 | `test_va_lemma_valid_indices_2m` | Valid indices with l1=0 ⇒ va_2m_valid | PASS | ✅ PASS |
| 8 | `test_va_lemma_roundtrip` | 4k valid VA decomposition/recomposition roundtrip | PASS | ✅ PASS |
| 9 | `test_pcid_unique_basic` | Different procs have different PCIDs | PASS | ✅ PASS |
| 10 | `test_quota_set_mem_4k` | set_mem_4k preserves other fields | PASS | ✅ PASS |
| 11 | `test_quota_subtract_mem_4k` | subtract_mem_4k checks all fields correctly | PASS | ✅ PASS |
| 12 | `test_page_entry_is_empty` | Zero PageEntry is empty | PASS | ✅ PASS |
| 13 | `test_page_entry_not_empty` | PageEntry with present=true is not empty | PASS | ✅ PASS |
| 14 | `test_page_entry_not_empty_addr` | PageEntry with non-zero addr is not empty | PASS | ✅ PASS |
| 15 | `test_page_ptr_valid_param` | Parameterized page_ptr_valid | PASS | ✅ PASS |
| 16 | `test_page_index_valid_param` | Parameterized page_index_valid | PASS | ✅ PASS |
| 17 | `test_pcid_unique_symmetric` | pcid_unique called on other proc still works | PASS | ✅ PASS |
| 18 | `test_quota_set_mem_4k_zero` | set_mem_4k with zero value | PASS | ✅ PASS |

---

## Completeness Results (should FAIL)

### Round 1: Precondition Violations
**File**: `completeness_round1.rs`
**Result**: `43 verified, 5 errors` (all 5 tests failed as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_pcid_unique_no_wf` | pcid_unique without wf() precondition | FAIL | ✅ FAIL |
| 2 | `test_fail_pcid_unique_no_proc_dom` | pcid_unique without proc in domain | FAIL | ✅ FAIL |
| 3 | `test_fail_pcid_unique_no_preconditions` | pcid_unique with no preconditions at all | FAIL | ✅ FAIL |
| 4 | `test_fail_quota_subtract_wrong_amount` | subtract_mem_4k with incorrect amount (5 vs 3) | FAIL | ✅ FAIL |
| 5 | `test_fail_page_ptr_valid_unaligned` | page_ptr_valid without alignment guarantee | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions
**File**: `completeness_round2.rs`
**Result**: `43 verified, 5 errors` (all 5 tests failed as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_pcid_ordered` | Assert pcid ordering (only distinctness guaranteed) | FAIL | ✅ FAIL |
| 2 | `test_fail_pcid_specific_value` | Assert pcid < 100 (no specific value guaranteed) | FAIL | ✅ FAIL |
| 3 | `test_fail_va_lemma_tighter_l4` | Assert l4 < 256 (spec says < 512) | FAIL | ✅ FAIL |
| 4 | `test_fail_va_lemma_l1_zero_for_4k` | Assert l1 == 0 for 4k VA (only for 2m) | FAIL | ✅ FAIL |
| 5 | `test_fail_va_lemma_l2_zero_for_2m` | Assert l2 == 0 for 2m VA (only for 1g) | FAIL | ✅ FAIL |

### Round 3: Negated/Contradicted Postconditions
**File**: `completeness_round3.rs`
**Result**: `43 verified, 5 errors` (all 5 tests failed as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_pcid_equal` | Assert equal PCIDs (negates uniqueness) | FAIL | ✅ FAIL |
| 2 | `test_fail_va_lemma_negate_l4_bound` | Assert l4 >= 512 (negates bound) | FAIL | ✅ FAIL |
| 3 | `test_fail_va_lemma_negate_injectivity` | Assert equal VAs for different indices | FAIL | ✅ FAIL |
| 4 | `test_fail_negate_is_empty` | Assert !is_empty for zero entry | FAIL | ✅ FAIL |
| 5 | `test_fail_va_lemma_negate_2m_l3` | Assert l3 >= 512 for 2m VA | FAIL | ✅ FAIL |

### Round 4: Wrong Specific Values
**File**: `completeness_round4.rs`
**Result**: `43 verified, 5 errors` (all 5 tests failed as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_page_ptr_valid_unaligned` | page_ptr_valid(0x1001) — unaligned | FAIL | ✅ FAIL |
| 2 | `test_fail_page_index_valid_out_of_range` | page_index_valid(NUM_PAGES) — out of range | FAIL | ✅ FAIL |
| 3 | `test_fail_quota_subtract_wrong` | subtract result 8, expected 7 | FAIL | ✅ FAIL |
| 4 | `test_fail_page_entry_wrong_empty` | is_empty with addr=0x1000 | FAIL | ✅ FAIL |
| 5 | `test_fail_quota_set_wrong_value` | set_mem_4k(7) then assert mem_4k == 8 | FAIL | ✅ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases
**File**: `completeness_round5.rs`
**Result**: `43 verified, 5 errors` (all 5 tests failed as expected)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_va_not_page_ptr` | VA validity ≠ page pointer validity | FAIL | ✅ FAIL |
| 2 | `test_fail_l4_equal_not_va_equal` | Same l4 index ≠ same VA | FAIL | ✅ FAIL |
| 3 | `test_fail_pcid_unique_cardinality` | pcid_unique ≠ specific pcid value | FAIL | ✅ FAIL |
| 4 | `test_fail_quota_subtract_commute` | Wrong subtraction amount | FAIL | ✅ FAIL |
| 5 | `test_fail_index2va_not_page_valid` | index2va result ≠ valid page pointer | FAIL | ✅ FAIL |

---

## Overall Assessment

### Correctness: ✅ All 18 tests PASS
The specifications are **correct** — the `requires`/`ensures` contracts accurately describe the functions' behavior. Key verified properties:
- `va_lemma` correctly constrains index bounds for 4k/2m/1g VAs and provides injectivity of `spec_index2va`
- `pcid_unique` correctly ensures PCID distinctness
- Spec functions (`page_ptr_valid`, `PageEntry::is_empty`, `Quota` operations) behave as defined

### Completeness: ✅ All 25 tests FAIL (as expected)
The specifications are **complete enough** — they reject:
- Missing preconditions (all 3 tested violations detected)
- Overly strong claims (ordering, tighter bounds, wrong tier guarantees)
- Negated postconditions (all contradictions rejected)
- Wrong concrete values (unaligned pointers, wrong arithmetic, bad indices)
- Cross-domain misuse (VA ≠ page ptr, l4 equality ≠ full VA equality)

### Spec Gaps Found: None
No unexpected passes were observed in the completeness tests. The specs appear well-designed and appropriately tight.
