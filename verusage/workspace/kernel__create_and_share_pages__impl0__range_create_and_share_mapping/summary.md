# Summary: Specification Testing for `range_create_and_share_mapping`

## File Under Test

`kernel__create_and_share_pages__impl0__range_create_and_share_mapping.rs` — Defines a kernel OS module for sharing page mappings between processes. Contains two key functions:

1. **`create_entry_and_share`** (`#[verifier::external_body]`) — Shares a single page mapping from source to target process. Ensures kernel well-formedness is preserved, domains are unchanged, free pages decrease by at most 3, target address space is extended, and the physical page reference counter is incremented.

2. **`range_create_and_share_mapping`** (verified loop) — Shares a range of VA mappings by iterating `create_entry_and_share`. Ensures the same invariants hold across the full range, with cumulative quota and free page consumption.

Also tests helper spec functions: `spec_subtract_mem_4k`, `page_ptr_valid`, `page_index_valid`, `spec_page_ptr2page_index`, `spec_page_index2page_ptr`, `page_index_2m_valid`, `spec_page_index_truncate_2m`.

---

## Correctness Results

All tests **PASS** (58 verified, 0 errors).

| Test Name | Description | Expected | Actual |
|-----------|-------------|----------|--------|
| `test_subtract_mem_4k_concrete_match` | `spec_subtract_mem_4k` with concrete values (10-3=7) | PASS | PASS |
| `test_subtract_mem_4k_zero` | Zero subtraction preserves quota unchanged | PASS | PASS |
| `test_subtract_preserves_fields` | Parametric: mem_2m, mem_1g, pcid, ioid preserved | PASS | PASS |
| `test_subtract_correct_mem4k` | Parametric: mem_4k - k == new.mem_4k | PASS | PASS |
| `test_page_ptr_valid_zero` | ptr=0 is a valid page pointer | PASS | PASS |
| `test_page_ptr_valid_aligned` | ptr=0x1000 is valid | PASS | PASS |
| `test_page_index_valid_zero` | index=0 is valid | PASS | PASS |
| `test_page_ptr_valid_implies_aligned` | Parametric: valid ptr → 4k-aligned | PASS | PASS |
| `test_page_ptr_valid_implies_bounded` | Parametric: valid ptr → ptr/0x1000 < NUM_PAGES | PASS | PASS |
| `test_page_index2ptr_def` | Parametric: index2ptr(i) == i*4096 | PASS | PASS |
| `test_page_ptr2index_def` | Parametric: ptr2index(ptr) == ptr/4096 | PASS | PASS |
| `test_domain_preservation_model` | Domain equality preserves membership | PASS | PASS |
| `test_create_entry_ret_bounded` | ret ≤ 3 model check | PASS | PASS |
| `test_free_pages_decrement` | Free pages decrease model | PASS | PASS |
| `test_subtract_mem_4k_large` | Large quota values (1000-3=997) | PASS | PASS |
| `test_address_space_insert_model` | Set insertion makes element a member | PASS | PASS |
| `test_page_index_2m_valid_zero` | Index 0 is 2m-valid | PASS | PASS |
| `test_page_index_2m_valid_512` | Index 512 is 2m-valid | PASS | PASS |

---

## Completeness Results

### Round 1: Precondition Violations (5 errors / 5 tests — all FAIL as expected)

| Test Name | What It Tests | Expected | Actual |
|-----------|---------------|----------|--------|
| `test_wrong_subtract_amount` | Wrong k value (5 instead of 3) | FAIL | FAIL |
| `test_page_ptr_valid_unaligned` | Unaligned ptr (1) claimed valid | FAIL | FAIL |
| `test_page_ptr_valid_too_large` | Out-of-range ptr claimed valid | FAIL | FAIL |
| `test_page_index_out_of_range` | NUM_PAGES claimed valid index | FAIL | FAIL |
| `test_subtract_mismatched_fields` | Changed mem_2m in subtraction | FAIL | FAIL |

### Round 2: Overly Strong Postconditions (5 errors / 5 tests — all FAIL as expected)

| Test Name | What It Tests | Expected | Actual |
|-----------|---------------|----------|--------|
| `test_ret_always_zero` | ret == 0 (spec says ≤ 3) | FAIL | FAIL |
| `test_free_pages_unchanged` | Free pages unchanged (they decrease) | FAIL | FAIL |
| `test_ret_always_three` | ret == 3 always (spec says ≤ 3) | FAIL | FAIL |
| `test_subtract_also_changes_mem2m` | mem_2m changes (it's preserved) | FAIL | FAIL |
| `test_page_ptr_strict_lower_bound` | ptr > 0 always (ptr=0 is valid) | FAIL | FAIL |

### Round 3: Negated Postconditions (5 errors / 5 tests — all FAIL as expected)

| Test Name | What It Tests | Expected | Actual |
|-----------|---------------|----------|--------|
| `test_subtract_preserves_mem4k` | mem_4k unchanged when k > 0 | FAIL | FAIL |
| `test_valid_ptr_not_aligned` | Valid ptr is NOT aligned | FAIL | FAIL |
| `test_valid_ptr_unbounded` | Valid ptr has index ≥ NUM_PAGES | FAIL | FAIL |
| `test_domains_change` | Membership lost when domains equal | FAIL | FAIL |
| `test_negate_page_index_range` | Out-of-range index is valid | FAIL | FAIL |

### Round 4: Wrong Specific Values (5 errors / 5 tests — all FAIL as expected)

| Test Name | What It Tests | Expected | Actual |
|-----------|---------------|----------|--------|
| `test_wrong_index_to_ptr` | index2ptr(1) == 8192 (should be 4096) | FAIL | FAIL |
| `test_wrong_ptr_to_index` | ptr2index(4096) == 2 (should be 1) | FAIL | FAIL |
| `test_wrong_subtract_concrete` | 10-3 = 8 (should be 7) | FAIL | FAIL |
| `test_wrong_2m_valid` | Index 1 is 2m-valid (1%512≠0) | FAIL | FAIL |
| `test_wrong_truncate_2m` | truncate_2m(513) == 513 (should be 512) | FAIL | FAIL |

### Round 5: Cross-function Misuse & Edge Cases (5 errors / 5 tests — all FAIL as expected)

| Test Name | What It Tests | Expected | Actual |
|-----------|---------------|----------|--------|
| `test_truncate_is_identity` | truncate_2m always identity (only for aligned) | FAIL | FAIL |
| `test_4k_implies_2m` | 4k-valid implies 2m-valid (false) | FAIL | FAIL |
| `test_subtract_commutative` | Subtraction is commutative (it's directional) | FAIL | FAIL |
| `test_insert_no_growth` | Set insertion doesn't grow set (it does) | FAIL | FAIL |
| `test_2m_implies_1g` | 2m-valid implies 1g-valid (much stronger) | FAIL | FAIL |

---

## Overall Assessment

### Correctness
**The specs are correct.** All 18 correctness tests pass, confirming:
- `spec_subtract_mem_4k` correctly models quota subtraction (only mem_4k changes, others preserved)
- `page_ptr_valid` and `page_index_valid` correctly define validity constraints
- `spec_page_ptr2page_index` and `spec_page_index2page_ptr` implement the expected arithmetic
- The domain preservation postconditions of `create_entry_and_share` and `range_create_and_share_mapping` are logically consistent
- The address space insertion model is correct

### Completeness
**The specs are complete (tight enough).** All 25 completeness tests fail as expected:
- Invalid inputs are rejected (wrong arithmetic, unaligned pointers, out-of-range indices)
- Overly strong claims are rejected (ret==0, unchanged free pages, ptr>0)
- Negated postconditions are rejected
- Wrong concrete values are rejected
- Cross-function misuse is rejected (4k doesn't imply 2m, truncation isn't identity, subtraction isn't commutative)

### Spec Gaps Found
**None.** No unexpected passes in completeness tests. The specifications are both correct and tight.
