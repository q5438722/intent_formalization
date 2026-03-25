# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__util__page_ptr_util_u__page_index2page_ptr/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: nonzero_index_zero_result → `page_index2page_ptr`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A nonzero page index mapping to pointer 0 would indicate arithmetic overflow or a collapsed mapping

### φ2: max_index_wraps_below_input → `page_index2page_ptr`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** At the maximum valid index the result wrapping below the index itself would indicate unsigned overflow in the multiplication

### φ3: non_injective_mapping → `page_index2page_ptr`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Two distinct page indices producing the same pointer would make the mapping ambiguous and corrupt page identity

### φ4: result_exceeds_max_usize → `page_index2page_ptr`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A page pointer exceeding the declared MAX_USIZE bound would violate the system's memory model invariant

### φ5: not_page_aligned → `page_index2page_ptr`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A page pointer that is not 4096-aligned would violate the fundamental page-alignment invariant required by the memory subsystem

