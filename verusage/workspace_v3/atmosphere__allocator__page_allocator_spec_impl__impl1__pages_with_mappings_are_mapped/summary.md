# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl1__pages_with_mappings_are_mapped/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: roundtrip_index_ptr_index_lossy → `page_index2page_ptr`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If index→ptr→index roundtrip is lossy, page tracking is fundamentally broken

### φ2: roundtrip_ptr_index_ptr_lossy → `page_ptr2page_index`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If ptr→index→ptr roundtrip is lossy, allocated page addresses cannot be recovered from indices

### φ3: page_index2page_ptr_not_aligned → `page_index2page_ptr`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If page_index2page_ptr returns non-4k-aligned addresses, the result violates page_ptr2page_index's precondition

### φ4: page_ptr2page_index_non_injective → `page_ptr2page_index`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If distinct aligned pointers map to the same index, the allocator cannot distinguish different pages

### φ5: page_index2page_ptr_zero_nonzero → `page_index2page_ptr`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If index 0 does not map to address 0, the base-address assumption used throughout page_array_wf is violated

