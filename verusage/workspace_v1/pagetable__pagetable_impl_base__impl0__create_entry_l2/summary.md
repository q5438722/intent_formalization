# Adversarial Proof Test Results Summary

**Target**: `pagetable__pagetable_impl_base__impl0__create_entry_l2.rs`
**Function**: `PageTable::create_entry_l2` — creates an L2 page table entry in a 4-level page table hierarchy

---

## Results Overview

| Test File | Tests | Passed (verified) | Failed (rejected) | Expected Failures |
|---|---|---|---|---|
| `boundary_tests.rs` | 5 | 0 | 5 | 5 |
| `behavioral_mutation_tests.rs` | 5 | 0 | 5 | 5 |
| `logical_tests.rs` | 5 | 0 | 5 | 5 |
| **Total** | **15** | **0** | **15** | **15** |

**All 15 adversarial tests were correctly rejected by the specification.**

---

## Boundary Tests (5/5 FAILED as expected)

| # | Test | Property φ | Result |
|---|---|---|---|
| 1 | `test_boundary_l4i_out_of_range` | `spec_resolve_mapping_l4(512).is_Some()` — out-of-bounds L4 index | ✅ REJECTED |
| 2 | `test_boundary_l2_without_l3` | L2 mapping exists when L3 is absent | ✅ REJECTED |
| 3 | `test_boundary_max_page_ptr` | `page_ptr_valid(usize::MAX)` — max value is valid | ✅ REJECTED |
| 4 | `test_boundary_arbitrary_l4_present` | Specific L4 entry (index 256) is present | ✅ REJECTED |
| 5 | `test_boundary_4k_without_l2` | 4K mapping exists when L2 is absent | ✅ REJECTED |

**Conclusion**: The specification correctly enforces index bounds, hierarchical dependency (L3→L2→L1), and page pointer validity constraints.

---

## Behavioral Mutation Tests (5/5 FAILED as expected)

| # | Test | Property φ | Result |
|---|---|---|---|
| 1 | `test_mutation_all_l2_entries_present` | All 512 L2 entries must be present | ✅ REJECTED |
| 2 | `test_mutation_l1_write_forced` | L1 leaf entries must have write=true | ✅ REJECTED |
| 3 | `test_mutation_l3_ps_always_set` | All present L3 entries must be huge pages (ps=true) | ✅ REJECTED |
| 4 | `test_mutation_l1_exec_disabled` | L1 entries must have execute_disable=true | ✅ REJECTED |
| 5 | `test_mutation_2m_implies_4k` | 2M mapping implies 4K mapping at same indices | ✅ REJECTED |

**Conclusion**: The specification correctly distinguishes between upper-level (L4/L3/L2) and leaf-level (L1) permission semantics. `rwx_upper_level_entries` correctly applies only to non-PS upper-level entries. The mutual exclusion between 2M and 4K mappings at the same L2 index is properly enforced.

---

## Logical Tests (5/5 FAILED as expected)

| # | Test | Property φ | Result |
|---|---|---|---|
| 1 | `test_logical_kernel_l4_end_positive` | `kernel_l4_end > 0` from `wf()` | ✅ REJECTED |
| 2 | `test_logical_cr3_nonzero` | `cr3 != 0` from `wf()` | ✅ REJECTED |
| 3 | `test_logical_mapping_4k_injective` | 4K mapping is injective (no VA aliasing) | ✅ REJECTED |
| 4 | `test_logical_pcid_always_some` | `pcid.is_Some()` from `wf()` | ✅ REJECTED |
| 5 | `test_logical_l1_no_self_reference` | L1 entries cannot point to L1 table pages | ✅ REJECTED |

**Conclusion**: The specification does not over-constrain the system:
- `kernel_l4_end = 0` is allowed (potentially intentional for flexibility)
- `cr3 = 0` is allowed by `page_ptr_valid` (null address satisfies alignment/range — potential spec weakness)
- VA aliasing is permitted (multiple VAs can map to same PA)
- Either `pcid` or `ioid` can be `Some` (correctly modeled by `pcid_ioid_wf`)
- L1 entries can point to L1 table page addresses (spec only prevents L1→L2, L1→L3, L1→cr3, but not L1→L1)

---

## Potential Spec Observations

1. **`page_ptr_valid(0)` is satisfiable**: Address 0 passes the validity check (`0 % 0x1000 == 0` and `0 / 0x1000 < NUM_PAGES`). This means `cr3 = 0` is permitted. In real systems, address 0 is typically reserved as a null pointer. This could be a spec weakness if the intent is to disallow null page pointers.

2. **L1→L1 references not prevented**: The `wf_l1` spec prevents L1 entries from pointing to L2 tables, L3 tables, or cr3, but does NOT prevent L1 entries from pointing to addresses that happen to be L1 table pages. This could allow circular references through L1 tables.

3. **No mapping injectivity**: The specification allows multiple virtual addresses to map to the same physical address (aliasing). This is consistent with how page tables work in practice, but could be unintended if the system requires unique VA→PA mappings.

4. **`kernel_l4_end = 0` allowed**: The spec requires `kernel_l4_end < 512` but not `kernel_l4_end > 0`, meaning a page table with zero kernel entries is valid.
