# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__memory_manager__spec_impl__impl0__alloc_iommu_table/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: free_ioids_not_consumed → `alloc_iommu_table`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the free ioid list never shrinks, the allocator can hand out ioids without consuming resources, allowing unbounded allocation

### φ2: alloc_activates_extra_pcid → `alloc_iommu_table`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If allocating an ioid causes a previously-inactive pcid to become active, ioid and pcid allocation domains are not properly isolated

### φ3: other_ioid_mapping_corrupted → `alloc_iommu_table`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If allocating a new ioid silently corrupts another active ioid's IOMMU page-table mapping, memory isolation between IO devices is broken

### φ4: alloc_wrong_proc_ptr → `alloc_iommu_table`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the newly allocated ioid is bound to a zero/null proc pointer instead of the requested one, device-to-process ownership is lost

### φ5: ret_mapping_nonempty → `alloc_iommu_table`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the freshly allocated IOMMU table already contains mappings, the new IO device could access stale physical memory from a prior owner, breaking DMA isolation

