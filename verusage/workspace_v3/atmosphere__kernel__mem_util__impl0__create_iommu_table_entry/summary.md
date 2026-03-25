# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__mem_util__impl0__create_iommu_table_entry/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: alloc_always_max → `create_iommu_table_entry`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, the function always allocates 3 pages and never reuses existing intermediate page table entries

### φ2: alloc_always_zero → `create_iommu_table_entry`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, the function never allocates any pages making it a no-op that cannot create new table structure

### φ3: no_partial_alloc → `create_iommu_table_entry`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, partial reuse of existing L4/L3/L2 tables is impossible — the function either does nothing or allocates all three levels

### φ4: va_indices_all_zero → `create_iommu_table_entry`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, va_4k_valid is so restrictive that all valid VAs decompose to index (0,0,0,_), collapsing the entire IOMMU address space to a single L2 entry

### φ5: mem_valid_only_zero → `create_iommu_table_entry`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, MEM_valid is vacuously satisfied only by zero, making every page entry address (including the returned L2 ptr ret.1) null

