# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__alloc_and_map_4k/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: ret_in_old_free_1g → `alloc_and_map_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If ret is in the 1G free list, the allocator hands out a huge page as a 4K page, causing cross-size memory corruption

### φ2: free_count_unchanged → `alloc_and_map_4k`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the free page count doesn't decrease, the allocator silently fails to consume a page, enabling infinite allocations from a finite pool

### φ3: ret_in_new_allocated → `alloc_and_map_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If ret appears in allocated_pages_4k while also being mapped, the page occupies two states simultaneously, violating allocator state machine invariants

### φ4: other_page_mapping_lost → `alloc_and_map_4k`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If an existing mapped page silently loses a mapping entry during a different page's allocation, it corrupts another address space's page table

### φ5: ret_not_owned_by_container → `alloc_and_map_4k`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the newly allocated page is not owned by the requesting container, it becomes an orphaned resource with no owner, causing a memory leak

