# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__memory_manager__spec_impl__impl0__create_iommu_table_l4_entry/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: cross_ioid_mapping_leak → `create_iommu_table_l4_entry`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, a mapping can appear in another IOID's address space after an unrelated L4 entry creation, violating IOMMU isolation

### φ2: null_page_map_ptr_accepted → `create_iommu_table_l4_entry`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, a null pointer (0) can be installed as a page table page, which would corrupt the IOMMU page-walk structure

### φ3: pcid_page_table_mapping_corrupted → `create_iommu_table_l4_entry`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, an IOMMU table operation can silently corrupt a process's page table address mappings, breaking memory isolation between IOMMU and process page tables

### φ4: target_ioid_deactivated → `create_iommu_table_l4_entry`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, creating an L4 entry deactivates the target IOID, making the new entry unreachable and leaking the donated page

### φ5: other_l4_entry_destroyed → `create_iommu_table_l4_entry`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, installing one L4 entry silently destroys another existing L4 entry, disconnecting an entire L3 subtree and leaking its pages while preserving mappings only vacuously

