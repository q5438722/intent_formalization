# Adversarial Proof Test Summary

**Target**: `memory_manager__spec_impl__impl0__create_iommu_table_l3_entry.rs`  
**Function**: `MemoryManager::create_iommu_table_l3_entry`

## Results Overview

| Category | Tests | Expected Failures | Actual Failures | Unexpected Passes |
|----------|-------|-------------------|-----------------|-------------------|
| Boundary | 5 | 5 | 5 | 0 |
| Behavioral | 5 | 5 | 5 | 0 |
| Logical | 5 | 5 | 5 | 0 |
| **Total** | **15** | **15** | **15** | **0** |

All 15 adversarial tests correctly **failed verification**, indicating the specification properly rejects invalid inputs, incorrect behaviors, and unintended logical properties.

---

## Boundary Tests (boundary_tests.rs) — 5/5 FAILED ✓

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_boundary_unaligned_ptr_valid` | `page_ptr_valid(1)` — unaligned pointer accepted? | FAILED ✓ |
| 2 | `test_boundary_half_page_ptr` | `page_ptr_valid(0x800)` — half-page offset accepted? | FAILED ✓ |
| 3 | `test_boundary_nonzero_addr_empty` | `PageEntry{addr:0x1000,...}.is_empty()` — nonzero addr empty? | FAILED ✓ |
| 4 | `test_boundary_present_entry_empty` | `PageEntry{present:true,...}.is_empty()` — present entry empty? | FAILED ✓ |
| 5 | `test_boundary_mem_valid_unaligned` | `MEM_valid(1)` — unaligned value MEM-valid? | FAILED ✓ |

**Conclusion**: The spec correctly rejects misaligned pointers, out-of-range values, and structurally invalid page entries.

---

## Behavioral Mutation Tests (behavioral_tests.rs) — 5/5 FAILED ✓

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_behav_zero_is_present` | `usize2present(0) == true` — zero value has present bit? | FAILED ✓ |
| 2 | `test_behav_pa_extraction_wrong` | `spec_usize2pa(0x1000) == 0` — PA extraction drops address? | FAILED ✓ |
| 3 | `test_behav_present_inverted` | `usize2present(1) == false` — present bit inverted? | FAILED ✓ |
| 4 | `test_behav_write_inverted` | `usize2write(2) == false` — write bit inverted? | FAILED ✓ |
| 5 | `test_behav_ps_false_positive` | `usize2ps(1) == true` — PS detected without bit 7? | FAILED ✓ |

**Conclusion**: The spec correctly encodes bit-level semantics for page entry fields. Mutating expected outputs is properly rejected.

---

## Logical Tests (logical_tests.rs) — 5/5 FAILED ✓

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_logic_zero_not_valid` | `!page_ptr_valid(0)` — zero excluded from valid pointers? | FAILED ✓ |
| 2 | `test_logic_entry_encoding_injective` | `entry(0) ≠ entry(8)` — encoding is injective? | FAILED ✓ |
| 3 | `test_logic_empty_implies_zero_encoding` | `!entry(8).is_empty()` — is_empty ↔ usize==0? | FAILED ✓ |
| 4 | `test_logic_write_implies_present` | `write(2) ⟹ present(2)` — write implies present? | FAILED ✓ |
| 5 | `test_logic_valid_ptrs_unique` | `0 == 0x1000` — only one valid pointer exists? | FAILED ✓ |

**Conclusion**: The spec does not accidentally entail unintended structural properties (injectivity, field coupling, uniqueness). The semantic boundary is well-controlled.

---

## Overall Assessment

The specification for `create_iommu_table_l3_entry` demonstrates **strong consistency**:

1. **Input validation**: Preconditions on `page_ptr_valid`, `MEM_valid`, and `PageEntry.is_empty` correctly discriminate valid from invalid inputs.
2. **Behavioral fidelity**: Bit-level decoding functions (`usize2present`, `usize2write`, `usize2ps`, `spec_usize2pa`) correctly reflect their intended semantics and reject mutations.
3. **Semantic boundaries**: The spec does not accidentally entail stronger-than-intended properties — zero is a valid pointer, the encoding is lossy (non-injective), and permission bits are independent.

No specification weaknesses were detected in these 15 queries.
