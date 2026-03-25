# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__remove_mapping_4k_helper2/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: target_still_mapped_after_removal → `remove_mapping_4k_helper2`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** After removing the sole mapping (ref_count==1, non-io) the page should transition to Free4k and no longer appear in mapped_pages_4k

### φ2: target_added_to_allocated → `remove_mapping_4k_helper2`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The page should be freed (moved to free list), not placed into the allocated set; allocated_pages_4k is preserved unchanged and should not contain the target

### φ3: free_pages_4k_unchanged → `remove_mapping_4k_helper2`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The freed page must be pushed onto free_pages_4k; if the free set were unchanged, the page would be lost and leak memory

### φ4: mappings_nonempty_after_sole_removal → `remove_mapping_4k_helper2`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** With ref_count==1 and is_io_page==false the single mapping is the only one; after removal mappings must be empty, so a non-empty result signals an inconsistent spec

### φ5: container_map_drops_other_pages → `remove_mapping_4k_helper2`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Only target_ptr should be removed from the container's page set; if other pages belonging to the same container also disappear, the container tracking is corrupted

