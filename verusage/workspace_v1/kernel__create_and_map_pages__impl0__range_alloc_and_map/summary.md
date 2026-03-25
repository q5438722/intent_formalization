# Adversarial Proof Test Summary

**Target**: `kernel__create_and_map_pages__impl0__range_alloc_and_map.rs`
**Function**: `Kernel::range_alloc_and_map`

## Results Overview

| Test File | Tests | Passed (verified) | Failed (rejected) | Status |
|---|---|---|---|---|
| `boundary_tests.rs` | 5 | 0 | 5 | ✅ All rejected |
| `behavioral_mutation_tests.rs` | 5 | 0 | 5 | ✅ All rejected |
| `logical_tests.rs` | 5 | 0 | 5 | ✅ All rejected |
| **Total** | **15** | **0** | **15** | **✅ Spec is consistent** |

All 15 adversarial tests were correctly **rejected** by Verus verification (42 verified items + 5 errors per file), meaning no invalid inputs, incorrect behaviors, or unintended reasoning passed through the specification.

---

## Boundary Tests (5/5 rejected)

| # | Test | Target Spec | Failure Mode |
|---|---|---|---|
| 1 | `test_boundary_va_zero_invalid` | `spec_va_4k_valid(0)` | VA=0 fails L4 index check (0 < KERNEL_MEM_END_L4INDEX) |
| 2 | `test_boundary_unaligned_page_ptr` | `page_ptr_valid(1)` | ptr=1 fails alignment check (1 % 0x1000 ≠ 0) |
| 3 | `test_boundary_page_index_at_limit` | `page_index_valid(NUM_PAGES)` | Off-by-one: index must be < NUM_PAGES |
| 4 | `test_boundary_va_max_invalid` | `spec_va_4k_valid(usize::MAX)` | MAX fails alignment and range checks |
| 5 | `test_boundary_present_entry_not_empty` | `PageEntry.is_empty()` | present=true violates is_empty |

**Conclusion**: Boundary predicates correctly reject edge-case invalid inputs.

## Behavioral Mutation Tests (5/5 rejected)

| # | Test | Mutation | Why Rejected |
|---|---|---|---|
| 1 | `test_mutation_quota_mem2m_wrong` | Changed mem_2m (50→40) | spec_subtract_mem_4k requires mem_2m preserved |
| 2 | `test_mutation_quota_wrong_amount` | Wrong result (100-10≠80) | spec requires exact arithmetic: mem_4k - k == new.mem_4k |
| 3 | `test_mutation_quota_pcid_wrong` | Changed pcid (10→20) | spec requires pcid preserved |
| 4 | `test_mutation_page_entry_nonzero_addr` | addr=4096 claimed empty | is_empty requires addr==0 |
| 5 | `test_mutation_quota_ioid_wrong` | Changed ioid (5→15) | spec requires ioid preserved |

**Conclusion**: Specification correctly rejects all mutated outputs. The `spec_subtract_mem_4k` relation precisely constrains all quota fields, and `PageEntry.is_empty()` checks every field.

## Logical Tests (5/5 rejected)

| # | Test | Unintended Property Tested | Why Rejected |
|---|---|---|---|
| 1 | `test_logical_subtract_not_deterministic` | Non-determinism of quota subtraction | Spec is deterministic: same inputs → same outputs |
| 2 | `test_logical_valid_ptr_not_unique` | Valid ptr implies ptr==0 | Many valid ptrs exist; spec doesn't over-constrain |
| 3 | `test_logical_roundtrip_breaks` | index↔ptr roundtrip breaks | Conversion functions are proper inverses |
| 4 | `test_logical_zero_subtract_changes_value` | k=0 subtraction changes mem_4k | Identity operation correctly preserved |
| 5 | `test_logical_subtract_changes_mem1g` | 4k subtraction changes mem_1g | Cross-field independence correctly maintained |

**Conclusion**: The specification does not allow unintended logical inferences. Determinism holds, conversions are consistent, and cross-field independence is maintained.

---

## Overall Assessment

The `range_alloc_and_map` specification and its supporting spec functions (`spec_va_4k_valid`, `page_ptr_valid`, `page_index_valid`, `PageEntry.is_empty`, `Quota.spec_subtract_mem_4k`, `spec_page_ptr2page_index`, `spec_page_index2page_ptr`) demonstrate **consistency** across all three test categories. The specification:

1. **Properly guards boundaries** — edge cases (zero, max, misaligned, off-by-one) are all rejected
2. **Rejects incorrect behaviors** — mutated outputs are caught by precise field-level constraints
3. **Prevents unintended reasoning** — no spurious logical consequences are derivable
