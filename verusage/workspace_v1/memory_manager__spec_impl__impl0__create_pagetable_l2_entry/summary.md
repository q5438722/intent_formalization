# Adversarial Proof Test Summary

**Target**: `memory_manager__spec_impl__impl0__create_pagetable_l2_entry`

## Results Overview

| Test File | Tests | All Failed (as expected) |
|-----------|-------|------------------------|
| `boundary_tests.rs` | 7 | ✅ Yes (7/7 errors) |
| `behavioral_mutation_tests.rs` | 6 | ✅ Yes (6/6 errors) |
| `logical_tests.rs` | 6 | ✅ Yes (6/6 errors) |

**Total: 19 tests, all correctly rejected by Verus.**

---

## Boundary Tests (7 tests)

All tests violate preconditions or use edge-case values that the spec should reject.

| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_boundary_unaligned_page_ptr` | `page_ptr_valid(0x1001)` — not 4K-aligned | ✅ FAIL |
| `test_boundary_ptr_below_page` | `page_ptr_valid(0xFFF)` — below alignment | ✅ FAIL |
| `test_boundary_l4i_in_kernel_range` | `KERNEL_MEM_END_L4INDEX <= 0` — kernel index | ✅ FAIL |
| `test_boundary_l4i_out_of_range` | `512 < 512` — index out of bounds | ✅ FAIL |
| `test_boundary_present_not_empty` | entry with `present=true` is not empty | ✅ FAIL |
| `test_boundary_mem_valid_low_bits` | `MEM_valid(1)` — low bits set | ✅ FAIL |
| `test_boundary_ptr_beyond_range` | `page_ptr_valid(0x1003)` — unaligned | ✅ FAIL |

**Conclusion**: The spec correctly rejects all invalid inputs at boundaries.

---

## Behavioral Mutation Tests (6 tests)

Each test assumes postconditions and asserts a mutated (wrong) output relation.

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_new_4k_mapping_appears` | mapping_4k preserved ⇒ assert new VA appears | ✅ FAIL |
| `test_mutation_closure_unchanged` | page_closure grows ⇒ assert unchanged | ✅ FAIL |
| `test_mutation_l2_addr_wrong` | L2 addr == page_map_ptr ⇒ assert different addr | ✅ FAIL |
| `test_mutation_2m_becomes_some` | 2M resolve is None ⇒ assert is Some | ✅ FAIL |
| `test_mutation_ioid_active_flips` | ioid_active preserved ⇒ assert flips | ✅ FAIL |
| `test_mutation_iommu_tables_change` | iommu_tables preserved ⇒ assert differs | ✅ FAIL |

**Conclusion**: The spec correctly rejects all mutated behavioral claims.

---

## Logical Tests (6 tests)

Each test probes properties not explicitly guaranteed by the specification.

| Test | Property Queried | Result | Spec Insight |
|------|-----------------|--------|--------------|
| `test_logical_null_ptr_invalid` | `!page_ptr_valid(0)` | ✅ FAIL | **Weakness found**: null pointer (0) passes `page_ptr_valid` since `0 % 0x1000 == 0` and `0 / 0x1000 < NUM_PAGES`. The spec does not exclude the null pointer. |
| `test_logical_present_implies_write` | `present ⇒ write` | ✅ FAIL | Correctly unconstrained — write is independent of present. |
| `test_logical_different_l2_resolves` | one L2 resolves ⇒ another resolves | ✅ FAIL | Correctly uncoupled — L2 indices are independent. |
| `test_logical_ptpages_value_unconstrained` | domain growth ⇒ value at new key is target_pcid | ✅ FAIL | **Weakness found**: The postcondition of `create_pagetable_l2_entry` only constrains `page_table_pages@.dom()` (domain grows by `page_map_ptr`) but does NOT specify what value is stored at `page_table_pages@[page_map_ptr]`. The implementation sets it to `target_pcid`, but this is not in the ensures clause. |
| `test_logical_l2_user_unconstrained` | `present ⇒ user` | ✅ FAIL | Correctly unconstrained — user permission is independent. |
| `test_logical_cross_pcid_mapping_coupling` | one pcid's mapping preserved ⇒ another's matches | ✅ FAIL | Correctly uncoupled — per-pcid mappings are independent. |

**Conclusion**: The spec correctly rejects all unintended logical inferences.

---

## Identified Spec Weaknesses

1. **Null pointer accepted as valid**: `page_ptr_valid(0)` evaluates to `true`. In a real memory manager, the null page pointer should likely be excluded.

2. **page_table_pages value unconstrained**: The postcondition specifies `self.page_table_pages@.dom() =~= old(self).page_table_pages@.dom().insert(page_map_ptr)` (domain grows) but does not specify `self.page_table_pages@[page_map_ptr] == target_pcid` (the value stored). The implementation does assign `target_pcid`, but this is not formally guaranteed by the ensures clause. This could allow a verifier to accept implementations that store the wrong pcid for the new page.
