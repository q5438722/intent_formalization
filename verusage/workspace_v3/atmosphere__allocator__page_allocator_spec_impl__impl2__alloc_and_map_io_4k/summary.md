# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__alloc_and_map_io_4k/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: ret_still_in_free_set → `alloc_and_map_io_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** After allocating ret from the free list, it must not still be present in the free set

### φ2: regular_mappings_nonempty_after_io_alloc → `alloc_and_map_io_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A freshly IO-allocated page should have no regular (Pcid) mappings — non-empty would indicate stale state leaking

### φ3: other_container_pages_modified → `alloc_and_map_io_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Allocating a page to one container must never modify the owned-page set of a different container

### φ4: ret_was_already_mapped → `alloc_and_map_io_4k`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The returned page must not have already been in the mapped set — that would imply double-mapping

### φ5: allocated_set_grows → `alloc_and_map_io_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The page transitions free→mapped, so it must not appear in the allocated set — presence there would mean a page is simultaneously allocated and mapped

