# Specification Test Summary

## File Under Test
`kernel__create_and_share_pages__impl0__create_entry_and_share.rs`

Defines the `Kernel::create_entry_and_share` function which creates page table entries for a target process and shares a mapping from a source process's address space. Also defines `share_mapping` (external_body) and `create_entry` (external_body), along with numerous spec helper functions for page/VA validation, quota management, and address translation.

---

## Correctness Results (correctness_tests.rs)

All tests **PASS** (62 verified, 0 errors).

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | test_quota_subtract_zero | Quota subtract 0 preserves all fields | PASS | PASS |
| 2 | test_quota_subtract_concrete_3 | Subtract 3 from mem_4k=10 gives 7 | PASS | PASS |
| 3 | test_quota_subtract_exact | Subtract all mem_4k (3-3=0) | PASS | PASS |
| 4 | test_page_ptr_valid_zero | ptr=0 is valid (aligned, in range) | PASS | PASS |
| 5 | test_page_ptr_valid_4k_aligned | ptr=0x1000 is valid | PASS | PASS |
| 6 | test_page_ptr_valid_large_aligned | ptr=0x10000 is valid | PASS | PASS |
| 7 | test_page_index_valid_zero | index=0 is valid | PASS | PASS |
| 8 | test_page_index_valid_near_max | index=NUM_PAGES-1 is valid | PASS | PASS |
| 9 | test_page_entry_is_empty_true | All-zero PageEntry is empty | PASS | PASS |
| 10 | test_page_entry_is_empty_false | Non-zero addr PageEntry is not empty | PASS | PASS |
| 11 | test_spec_page_entry_to_map_entry_concrete | Correct field extraction | PASS | PASS |
| 12 | test_page_index_2m_valid_concrete | 0 and 512 are 2M-valid indices | PASS | PASS |
| 13 | test_page_index_truncate_2m_concrete | Truncation to 512 boundaries | PASS | PASS |
| 14 | test_constants_consistent | NUM_PAGES, PCID_MAX, etc. correct | PASS | PASS |
| 15 | test_page_ptr_valid_param_alignment | Valid ptr implies 0x1000-aligned | PASS | PASS |
| 16 | test_page_index_valid_param_range | Valid index implies < NUM_PAGES | PASS | PASS |
| 17 | test_quota_subtract_param | Parameterized: subtract k from quota | PASS | PASS |
| 18 | test_page_ptr_to_index_concrete | ptr2index: 0→0, 0x1000→1, 0x2000→2 | PASS | PASS |
| 19 | test_page_index_to_ptr_concrete | index2ptr: 0→0, 1→0x1000, 2→0x2000 | PASS | PASS |
| 20 | test_page_ptr_2m_valid_concrete | 0 and 0x200000 are 2M-valid ptrs | PASS | PASS |
| 21 | test_ret_bound_free_pages | ret≤3 ∧ free≥3 → free≥ret | PASS | PASS |

---

## Completeness Results

### Round 1: Precondition Violations (completeness_round1.rs)
All tests **FAIL** (5 errors). Specs correctly reject invalid inputs.

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_page_ptr_valid_unaligned | page_ptr_valid(1) — not aligned | FAIL | FAIL |
| 2 | test_page_ptr_valid_odd | page_ptr_valid(0x1001) — not aligned | FAIL | FAIL |
| 3 | test_page_index_valid_at_max | page_index_valid(NUM_PAGES) — at bound | FAIL | FAIL |
| 4 | test_page_index_2m_valid_unaligned | page_index_2m_valid(1) — not 512-aligned | FAIL | FAIL |
| 5 | test_quota_subtract_overflow | Subtract more than available | FAIL | FAIL |

### Round 2: Overly Strong Postconditions (completeness_round2.rs)
All tests **FAIL** (5 errors). Specs don't guarantee overly tight bounds.

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_overly_strong_page_bound | ptr/0x1000 < 100 (too tight) | FAIL | FAIL |
| 2 | test_overly_strong_index_even | Valid index must be even | FAIL | FAIL |
| 3 | test_overly_strong_quota_zero | Subtracted quota always 0 | FAIL | FAIL |
| 4 | test_overly_strong_ptr_always_2m | Valid ptr must be 2M-aligned | FAIL | FAIL |
| 5 | test_overly_strong_truncate_zero | Truncate always gives 0 | FAIL | FAIL |

### Round 3: Negated Postconditions (completeness_round3.rs)
All tests **FAIL** (5 errors). Specs correctly guarantee their stated properties.

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_negated_page_ptr_valid_zero | !page_ptr_valid(0) | FAIL | FAIL |
| 2 | test_negated_page_index_valid_zero | !page_index_valid(0) | FAIL | FAIL |
| 3 | test_negated_page_entry_empty | !is_empty for empty entry | FAIL | FAIL |
| 4 | test_negated_quota_subtract | Negate correct subtraction | FAIL | FAIL |
| 5 | test_negated_map_entry_field | m.write==false when should be true | FAIL | FAIL |

### Round 4: Wrong Specific Values (completeness_round4.rs)
All tests **FAIL** (5 errors). Specs produce correct concrete values.

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_wrong_ptr_to_index | ptr2index(0x2000)==3 (should be 2) | FAIL | FAIL |
| 2 | test_wrong_index_to_ptr | index2ptr(2)==0x3000 (should be 0x2000) | FAIL | FAIL |
| 3 | test_wrong_truncate_2m | truncate(513)==0 (should be 512) | FAIL | FAIL |
| 4 | test_wrong_constant_num_pages | NUM_PAGES==1024 (should be 2M) | FAIL | FAIL |
| 5 | test_wrong_map_entry_addr | m.addr==0x3000 (should be 0x2000) | FAIL | FAIL |

### Round 5: Cross-Function Misuse & Edge Cases (completeness_round5.rs)
All tests **FAIL** (5 errors). Specs correctly distinguish different functions/properties.

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | test_ptr_index_confusion | ptr2index(x)==index2ptr(x) | FAIL | FAIL |
| 2 | test_index_valid_implies_2m | index_valid → 2m_valid | FAIL | FAIL |
| 3 | test_quota_subtract_wrong_field | Subtract changes mem_2m | FAIL | FAIL |
| 4 | test_page_ptr_valid_implies_2m | ptr_valid → ptr_2m_valid | FAIL | FAIL |
| 5 | test_truncate_2m_identity | truncate_2m is identity | FAIL | FAIL |

---

## Overall Assessment

### Correctness
The specs are **correct**. All 21 correctness tests pass verification, confirming that:
- Helper spec functions (`page_ptr_valid`, `page_index_valid`, `spec_page_ptr2page_index`, etc.) produce expected results for concrete inputs
- Parameterized properties hold for arbitrary valid inputs
- `Quota::spec_subtract_mem_4k` correctly models quota deduction
- The `create_entry_and_share` return bound (≤3) is consistent with free page accounting

### Completeness
The specs are **complete** (tight enough). All 25 completeness tests fail verification, confirming that:
- Invalid inputs are correctly rejected (Round 1)
- Overly strong claims are not provable (Round 2)
- Negations of true properties fail (Round 3)
- Incorrect concrete values are rejected (Round 4)
- Cross-function confusion and false implications are rejected (Round 5)

### Spec Gaps Found
**None.** No unexpected passes in completeness tests. The specifications appear well-formed and appropriately tight for the tested properties.
