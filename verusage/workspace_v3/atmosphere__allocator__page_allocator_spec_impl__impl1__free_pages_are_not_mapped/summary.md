# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl1__free_pages_are_not_mapped/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: round_trip_ptr_to_index_to_ptr_fails → `page_ptr2page_index`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If ptr→index→ptr round-trip is lossy, address identity is broken and page lookups would silently corrupt

### φ2: round_trip_index_to_ptr_to_index_fails → `page_index2page_ptr`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If index→ptr→index round-trip is lossy, the allocator would map different indices to the same physical page

### φ3: index2ptr_not_aligned → `page_index2page_ptr`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If page_index2page_ptr returns non-4k-aligned addresses, the result violates page_ptr2page_index's precondition and all downstream page-table invariants break

### φ4: distinct_indices_same_ptr → `page_index2page_ptr`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If two distinct indices map to the same pointer, the allocator could hand out the same physical page twice, violating memory safety

### φ5: zero_ptr_nonzero_index → `page_ptr2page_index`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If address 0x0 does not map to index 0, the base-case identity is violated and off-by-one errors propagate through every page array access

