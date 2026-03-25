# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__remove_mapping_4k_helper1/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: target_remains_mapped → `remove_mapping_4k_helper1`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** After removing the only mapping (ref_count==1), the page should no longer be considered mapped; if provable, the spec fails to enforce unmapping.

### φ2: target_becomes_free → `remove_mapping_4k_helper1`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** An IO page whose last mapping is removed should become unavailable, not free; if provable, the spec allows leaking IO pages back to the free pool.

### φ3: other_mapped_page_dropped → `remove_mapping_4k_helper1`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Removing one page's mapping must not cause a different mapped page to lose its mapped status; if provable, the spec permits collateral damage to unrelated pages.

### φ4: allocated_gains_target → `remove_mapping_4k_helper1`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A page whose mapping is removed should not silently transition to allocated; if provable, the spec confuses unmapping with deallocation, enabling use-after-unmap.

### φ5: container_map_key_erased → `remove_mapping_4k_helper1`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The owning container's key should persist in the container map (with the target removed from its page set); if provable, the spec erroneously deletes the entire container entry, orphaning its other pages.

