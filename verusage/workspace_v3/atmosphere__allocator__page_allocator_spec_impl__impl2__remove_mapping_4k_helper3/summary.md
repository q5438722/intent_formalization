# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__remove_mapping_4k_helper3/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: target_leaves_mapped_4k → `remove_mapping_4k_helper3`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Target page should remain in mapped_pages_4k since ref_count != 1 implies other mappings still exist after removing one

### φ2: target_enters_free_list → `remove_mapping_4k_helper3`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A mapped page with remaining references should never be moved to the free list after removing a single mapping

### φ3: mapped_2m_page_lost → `remove_mapping_4k_helper3`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Removing a 4k mapping should never cause a 2m mapped page to leave the mapped_pages_2m set

### φ4: mapped_1g_page_lost → `remove_mapping_4k_helper3`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Removing a 4k mapping should never cause a 1g mapped page to leave the mapped_pages_1g set

### φ5: other_4k_page_unmapped → `remove_mapping_4k_helper3`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The spec preserves mappings only for pages still mapped in post-state; a different 4k page could silently lose its mapped status since mapped_pages_4k preservation is not explicitly ensured

