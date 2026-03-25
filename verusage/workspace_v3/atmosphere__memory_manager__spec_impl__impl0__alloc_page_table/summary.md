# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__memory_manager__spec_impl__impl0__alloc_page_table/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: new_pcid_2m_mapping_nonempty → `alloc_page_table`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Spec only constrains 4k mapping domain to be empty; a non-empty 2m mapping on a fresh page table would violate isolation

### φ2: ret_exceeds_pcid_max → `alloc_page_table`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the returned pcid is out of bounds, all subsequent array accesses indexed by it would be unsound

### φ3: spec_postconditions_inconsistent → `alloc_page_table`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, the combined pre/postconditions are unsatisfiable, meaning the function can never be correctly called

### φ4: ioid_mapping_changes → `alloc_page_table`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A page table allocation should never modify IOMMU mappings; if provable the frame condition or iommu invariant is broken

### φ5: new_pcid_page_closure_nonempty → `alloc_page_table`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the page_closure is non-empty yet page_table_pages is unchanged from when pcid was inactive, page tracking is inconsistent with no_memory_leak

