# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__alloc_page_4k/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: alloc_ret_still_free → `alloc_page_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Allocated page remaining in the free set would allow double-allocation

### φ2: alloc_free_len_unchanged → `alloc_page_4k`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Free list length not decreasing means allocation silently fails to remove a page

### φ3: alloc_4k_pollutes_2m → `alloc_page_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A 4k allocation leaking into the 2m allocated set breaks size-class isolation

### φ4: alloc_ret_is_mapped → `alloc_page_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A freshly allocated page appearing as mapped would bypass the map lifecycle

### φ5: alloc_free_allocated_overlap → `alloc_page_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Free and allocated sets overlapping means a page can be used twice simultaneously

