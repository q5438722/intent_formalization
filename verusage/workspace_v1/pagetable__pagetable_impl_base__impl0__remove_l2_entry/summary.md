# Adversarial Proof Test Summary

**Target**: `remove_l2_entry` in `pagetable__pagetable_impl_base__impl0__remove_l2_entry.rs`

## Results: All 15 tests FAIL verification as expected ✅

The specification correctly rejects all adversarial queries — no spec weaknesses detected.

---

### Boundary Tests (5/5 failed ✅)

| # | Test | Property Probed | Result |
|---|------|----------------|--------|
| 1 | `test_boundary_kernel_l4_end_must_be_positive` | wf() does NOT require kernel_l4_end > 0 | FAILED ✅ |
| 2 | `test_boundary_resolve_l4_at_512` | spec_resolve_mapping_l4 at out-of-bounds index 512 is underspecified | FAILED ✅ |
| 3 | `test_boundary_pcid_always_some` | wf() allows pcid to be None (when ioid is Some) | FAILED ✅ |
| 4 | `test_boundary_l3_tables_nonempty` | wf() allows empty l3_tables (no present user-range L4 entries) | FAILED ✅ |
| 5 | `test_boundary_kernel_l4_entry_present` | wf() does not constrain kernel L4 entries to be present | FAILED ✅ |

### Behavioral Mutation Tests (5/5 failed ✅)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_page_closure_unchanged` | Assert page_closure unchanged (spec says it removes target_l1_p) | FAILED ✅ |
| 2 | `test_mutation_mapping_2m_loses_entry` | Assert 2M mapping lost (spec says mapping_2m preserved) | FAILED ✅ |
| 3 | `test_mutation_mapping_1g_gains_entry` | Assert 1G mapping gained (spec says mapping_1g preserved) | FAILED ✅ |
| 4 | `test_mutation_return_wrong_page` | Assert ret == target_l2_p (spec says ret == target_l1_p) | FAILED ✅ |
| 5 | `test_mutation_page_closure_lost_more` | Assert extra page removed (spec removes exactly target_l1_p) | FAILED ✅ |

### Logical Tests (5/5 failed ✅)

| # | Test | Unguaranteed Property | Result |
|---|------|----------------------|--------|
| 1 | `test_logical_all_l4_entries_present` | wf() does NOT imply all user-range L4 entries are present | FAILED ✅ |
| 2 | `test_logical_4k_implies_2m` | A 4K mapping does NOT imply a 2M mapping at the same VA | FAILED ✅ |
| 3 | `test_logical_mapping_4k_nonempty` | wf() does NOT guarantee mapping_4k is non-empty | FAILED ✅ |
| 4 | `test_logical_l2_entries_share_addr` | Two different L2 entries do NOT share the same L1 address (disjointness holds) | FAILED ✅ |
| 5 | `test_logical_remove_implies_l3_removed` | Removing an L2 entry does NOT remove the parent L3 entry | FAILED ✅ |

## Conclusion

The `remove_l2_entry` specification is **consistent** across all three query categories:
- **Boundary**: Invalid/edge-case inputs are correctly unresolvable
- **Behavioral**: Mutated postconditions are correctly rejected
- **Logical**: Unintended stronger properties are correctly not entailed
