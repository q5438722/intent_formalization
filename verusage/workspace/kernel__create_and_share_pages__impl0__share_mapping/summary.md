# Verus Specification Test Summary

## File Under Test
`kernel__create_and_share_pages__impl0__share_mapping.rs` — Defines a verified microkernel's page sharing mechanism. Key proof functions:

- **`va_lemma()`**: Proves properties about virtual address decomposition/composition (index ranges, injectivity, round-trip).
- **`insert_page_mapping_t()`**: Returns a new page mapping with a new (proc, va) entry inserted at a target page.
- **`mapped_page_are_not_allocated()`**: Proves that mapped pages are disjoint from all allocated page sets.
- **`pcid_unique()`**: Proves PCID uniqueness across all processes.
- **`share_mapping()`**: Main exec function that shares a page mapping from source to target process (tested indirectly via its constituent proof functions).

---

## Correctness Results (should all PASS)

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_va_lemma_4k_valid_indices` | 4k-valid VA => all indices in [0,512) | PASS | PASS |
| 2 | `test_va_lemma_2m_valid_indices` | 2m-valid VA => indices in range, l1=0 | PASS | PASS |
| 3 | `test_va_lemma_1g_valid_indices` | 1g-valid VA => indices in range, l1=l2=0 | PASS | PASS |
| 4 | `test_va_lemma_index2va_injective_equal` | Equal index tuples => equal VAs | PASS | PASS |
| 5 | `test_va_lemma_index2va_injective_unequal` | Unequal index tuples => unequal VAs | PASS | PASS |
| 6 | `test_va_lemma_valid_indices_produce_4k_valid_va` | Valid indices => 4k-valid VA | PASS | PASS |
| 7 | `test_va_lemma_valid_indices_produce_2m_valid_va` | Valid indices with l1=0 => 2m-valid VA | PASS | PASS |
| 8 | `test_va_lemma_roundtrip` | Decompose then recompose => identity | PASS | PASS |
| 9 | `test_va_lemma_kernel_end` | 4k-valid VA => l4 >= KERNEL_MEM_END_L4INDEX | PASS | PASS |
| 10 | `test_insert_page_mapping_preserves_domain` | Domain preserved after insert | PASS | PASS |
| 11 | `test_insert_page_mapping_other_entries_unchanged` | Non-target entries unchanged | PASS | PASS |
| 12 | `test_insert_page_mapping_target_updated` | Target entry = old.insert(new_mapping) | PASS | PASS |
| 13 | `test_insert_page_mapping_contains_new` | Result contains new mapping at target | PASS | PASS |
| 14 | `test_mapped_not_allocated_4k` | Mapped page not in allocated_4k | PASS | PASS |
| 15 | `test_mapped_not_allocated_2m` | Mapped page not in allocated_2m | PASS | PASS |
| 16 | `test_mapped_not_allocated_1g` | Mapped page not in allocated_1g | PASS | PASS |
| 17 | `test_mapped_not_allocated_all` | Mapped page not in any allocated set | PASS | PASS |
| 18 | `test_pcid_unique` | Different procs have different PCIDs | PASS | PASS |
| 19 | `test_pcid_unique_triple` | Three distinct procs all have distinct PCIDs | PASS | PASS |
| 20 | `test_insert_page_mapping_spec_satisfied` | insert_page_mapping predicate holds for result | PASS | PASS |

**Verification result: 63 verified, 0 errors**

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_mapped_no_wf` | Call mapped_page_are_not_allocated without wf() | FAIL | FAIL |
| 2 | `test_fail_mapped_invalid_page_ptr` | Call with invalid page pointer | FAIL | FAIL |
| 3 | `test_fail_mapped_not_mapped` | Call without page being mapped | FAIL | FAIL |
| 4 | `test_fail_pcid_unique_no_wf` | Call pcid_unique without wf() | FAIL | FAIL |
| 5 | `test_fail_pcid_unique_not_in_dom` | Call pcid_unique with proc not in domain | FAIL | FAIL |

**Verification result: 43 verified, 5 errors**

### Round 2: Overly Strong Postconditions

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_mapped_not_in_mapped` | Assert mapped page not in mapped_pages_4k (too strong) | FAIL | FAIL |
| 2 | `test_fail_pcid_consecutive` | Assert PCIDs are consecutive (too strong) | FAIL | FAIL |
| 3 | `test_fail_insert_no_change` | Assert target unchanged after insert (too strong) | FAIL | FAIL |
| 4 | `test_fail_va_l4_exact` | Assert l4 == KERNEL_MEM_END_L4INDEX exactly (too strong) | FAIL | FAIL |
| 5 | `test_fail_va_4k_l1_zero` | Assert l1 == 0 for 4k VA (only for 2m) | FAIL | FAIL |

**Verification result: 43 verified, 5 errors**

### Round 3: Negated/Contradicted Postconditions

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_mapped_is_allocated_4k` | Assert mapped page IS in allocated_4k | FAIL | FAIL |
| 2 | `test_fail_mapped_is_allocated_2m` | Assert mapped page IS in allocated_2m | FAIL | FAIL |
| 3 | `test_fail_pcids_equal` | Assert different procs have SAME PCID | FAIL | FAIL |
| 4 | `test_fail_insert_domain_changed` | Assert domain changed after insert | FAIL | FAIL |
| 5 | `test_fail_va_l4_out_of_range` | Assert l4 >= 512 for valid VA | FAIL | FAIL |

**Verification result: 43 verified, 5 errors**

### Round 4: Wrong Specific Values

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_insert_removes_mapping` | Assert insert acts as remove | FAIL | FAIL |
| 2 | `test_fail_va_l3_always_zero` | Assert l3 == 0 for all 4k VAs | FAIL | FAIL |
| 3 | `test_fail_va_l2_always_zero` | Assert l2 == 0 for all 4k VAs | FAIL | FAIL |
| 4 | `test_fail_index2va_wrong_equality` | Assert index2va equal for different l1 values | FAIL | FAIL |
| 5 | `test_fail_kernel_end_wrong_value` | Assert l4 == 0 (should be >= 1) | FAIL | FAIL |

**Verification result: 43 verified, 5 errors**

### Round 5: Cross-function Misuse & Edge Cases

| # | Test Name | What it tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_pcid_unique_for_non_member` | Assert PCID unique for proc not in domain | FAIL | FAIL |
| 2 | `test_fail_double_insert_no_first` | Assert first insert's mapping disappears after second | FAIL | FAIL |
| 3 | `test_fail_insert_changes_wrong_entry` | Assert insert modifies a non-target entry | FAIL | FAIL |
| 4 | `test_fail_mapped_implies_free` | Assert mapped implies free (wrong inference) | FAIL | FAIL |
| 5 | `test_fail_2m_implies_not_4k` | Assert 2m-valid implies not 4k-valid (wrong) | FAIL | FAIL |

**Verification result: 43 verified, 5 errors**

---

## Overall Assessment

### Correctness: PASS
All 20 correctness tests verify successfully. The specifications correctly describe the behavior of virtual address decomposition, page mapping insertion, mapped/allocated disjointness, and PCID uniqueness.

### Completeness: PASS
All 25 completeness tests (5 rounds x 5 tests) are correctly rejected by the verifier. The specifications are tight enough to reject precondition violations, overly strong claims, negated postconditions, wrong concrete values, and cross-function misuse.

### Spec Gaps Found: None

### Notes
- `share_mapping()` is an `exec fn` and cannot be called from `proof fn` tests directly. Its correctness is indirectly tested through its constituent proof functions (`va_lemma`, `mapped_page_are_not_allocated`, `pcid_unique`, `insert_page_mapping_t`) which are all called within its body.
- Many spec functions use `closed spec fn` for `wf()` predicates, making it impossible to construct concrete well-formed instances. All tests use parameterized (universally quantified) inputs with `requires` clauses.
