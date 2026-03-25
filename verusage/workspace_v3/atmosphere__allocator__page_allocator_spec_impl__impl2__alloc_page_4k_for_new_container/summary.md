# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__alloc_page_4k_for_new_container/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: returned_page_still_free → `alloc_page_4k_for_new_container`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If verified, an allocated page remains in the free set, enabling double-allocation and memory corruption.

### φ2: returned_page_was_mapped → `alloc_page_4k_for_new_container`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If verified, a page simultaneously in free_4k and a mapped set can be allocated, corrupting an active mapping.

### φ3: alloc_makes_page_mapped → `alloc_page_4k_for_new_container`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If verified, a freshly allocated page appears as mapped, violating the alloc→map state machine and enabling premature page-table use.

### φ4: extra_free_page_lost → `alloc_page_4k_for_new_container`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If verified, allocating one 4k page silently removes an additional page from the free list, causing a memory leak.

### φ5: returned_page_in_allocated_2m → `alloc_page_4k_for_new_container`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If verified, a page can be in both free_4k and allocated_2m, meaning wf() fails to enforce cross-size disjointness and a 2m allocation could alias a 4k allocation.

