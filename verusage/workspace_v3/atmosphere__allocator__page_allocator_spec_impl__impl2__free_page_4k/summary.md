# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__free_page_4k/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: free_page_stays_allocated → `free_page_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** After freeing a page, it should no longer be in the allocated set; if it is, deallocation is a no-op and the page leaks.

### φ2: free_page_not_in_free_set → `free_page_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** After freeing, the page must appear in the free set; if it doesn't, the page is permanently lost and can never be reused.

### φ3: free_page_corrupts_2m_free → `free_page_4k`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Freeing a 4k page must not disturb 2m/1g free lists; if it does, a single free corrupts unrelated page pools.

### φ4: free_page_becomes_mapped → `free_page_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A freshly freed page must never appear as mapped; if it does, user-space could access a page in the free pool, breaking memory safety.

### φ5: free_page_changes_container_ownership → `free_page_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Freeing an unowned page must not alter any container's owned-page set; if it does, containers silently gain or lose pages.

