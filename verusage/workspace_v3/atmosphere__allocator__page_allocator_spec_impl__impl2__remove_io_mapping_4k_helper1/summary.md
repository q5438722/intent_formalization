# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__remove_io_mapping_4k_helper1/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: target_still_mapped → `remove_io_mapping_4k_helper1`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** After removing the sole IO mapping (ref_count==1), the page should no longer be mapped; being still mapped would mean the unmap transition failed.

### φ2: target_becomes_allocated → `remove_io_mapping_4k_helper1`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The target was mapped (not allocated) and allocated_pages_4k is unchanged; if it now appears allocated the spec conflates mapped and allocated states.

### φ3: container_map_4k_unchanged → `remove_io_mapping_4k_helper1`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The spec explicitly updates container_map_4k by removing target_ptr from its container's set; if this equals the old map, the removal is vacuous and container tracking is broken.

### φ4: extra_io_mappings_remain → `remove_io_mapping_4k_helper1`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** With ref_count==1 the only mapping is (ioid,va); any remaining io_mappings after removal would mean the spec under-constrains the ref_count invariant.

### φ5: regular_mappings_nonempty → `remove_io_mapping_4k_helper1`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Since ref_count==1 and io_mappings is non-empty, regular mappings must be empty (ref_count = mappings.len + io_mappings.len); a non-empty regular mapping set would violate the ref_count invariant.

