# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__memory_manager__spec_impl__impl0__resolve_iommu_table_mapping/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_returns_none → `resolve_iommu_table_mapping`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, no IOMMU mapping could ever be resolved for any active ioid and valid VA, rendering DMA translation useless.

### φ2: resolved_addr_always_zero → `resolve_iommu_table_mapping`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, every resolved IOMMU mapping would point to physical address zero, meaning all device DMA targets the null page.

### φ3: write_always_enabled → `resolve_iommu_table_mapping`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, all IOMMU-resolved pages are writable, so read-only DMA protections would be impossible.

### φ4: execute_disable_never_set → `resolve_iommu_table_mapping`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, no IOMMU mapping could ever disable execution, breaking NX-bit enforcement for device-accessible memory.

### φ5: different_va_same_physical_addr → `resolve_iommu_table_mapping`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, all distinct mapped VAs for a device alias to the same physical address, collapsing the entire IOMMU address space.

