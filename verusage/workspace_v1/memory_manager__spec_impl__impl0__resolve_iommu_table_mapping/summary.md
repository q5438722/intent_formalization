# Adversarial Proof Test Summary

**Target**: `memory_manager__spec_impl__impl0__resolve_iommu_table_mapping.rs`
**Function**: `MemoryManager::resolve_iommu_table_mapping`

## Specification Under Test

`resolve_iommu_table_mapping(ioid, va)` resolves a 4K virtual address in an IOMMU page table.

**Preconditions**: `self.wf()`, `self.ioid_active(ioid)`, `va_4k_valid(va)`

**Postconditions**:
1. `mapping_4k().dom().contains(va) == ret.is_Some()` — returns Some iff VA is mapped
2. If `ret.is_Some()`, the mapping entry equals `page_entry_to_map_entry(&ret.unwrap())`

---

## Results Summary

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary | 5 | 5 | 0 |
| Behavioral Mutation | 5 | 5 | 0 |
| Logical | 5 | 5 | 0 |
| **Total** | **15** | **15** | **0** |

**Verdict: All 15 adversarial tests correctly FAIL verification. The specification rejects all tested undesirable properties.**

---

## Boundary Tests (5/5 FAIL ✓)

| # | Test | Failure Mode |
|---|---|---|
| 1 | `test_boundary_ioid_out_of_range` | Assert `ioid_active(IOID_MAX)` — out-of-range IOID (4096 ≥ 4096) |
| 2 | `test_boundary_ioid_inactive` | Assert active when ioid is in free set |
| 3 | `test_boundary_va_not_4k_valid` | Assert `va_4k_valid(1)` — misaligned VA |
| 4 | `test_boundary_l4_index_512` | Assert L4 resolve succeeds at index 512 (out of bounds) |
| 5 | `test_boundary_va_zero_below_kernel` | Assert `va_4k_valid(0)` — below kernel L4 boundary |

## Behavioral Mutation Tests (5/5 FAIL ✓)

| # | Test | Mutation |
|---|---|---|
| 1 | `test_mutation_resolve_returns_some_on_empty_table` | Claim mapping contains VA when table is empty |
| 2 | `test_mutation_wrong_map_entry_addr` | Claim physical address is offset by 0x1000 |
| 3 | `test_mutation_wrong_write_bit` | Flip the write permission bit |
| 4 | `test_mutation_wrong_execute_disable_bit` | Flip the execute_disable bit |
| 5 | `test_mutation_contains_when_l4_none` | Claim mapping exists when L4 resolves to None |

## Logical Tests (5/5 FAIL ✓)

| # | Test | Unintended Property |
|---|---|---|
| 1 | `test_logical_resolve_injective` | Claim different VAs → different PAs (injectivity) |
| 2 | `test_logical_cross_ioid_mapping_disjoint` | Claim different IOIDs have disjoint VA mappings |
| 3 | `test_logical_active_ioid_has_mappings` | Claim active IOIDs always have mappings |
| 4 | `test_logical_returned_entry_present_is_true` | Claim user permission bit is always true |
| 5 | `test_logical_cross_function_pcid_ioid_confusion` | Claim IOMMU mapping equals page table mapping |

---

## Analysis

The specification is **well-constrained** for the tested properties:

- **Boundary correctness**: Invalid IOIDs, misaligned VAs, and out-of-range indices are all properly rejected.
- **Behavioral precision**: Mutations to address, write, and execute_disable fields are caught. Empty-table and absent-mapping cases are correctly handled.
- **Logical soundness**: The spec does not entail injectivity, cross-IOID disjointness, mandatory mappings, unconstrained permission bits, or cross-subsystem (IOMMU ↔ page table) confusion.

No specification weaknesses were detected in these 15 test categories.
