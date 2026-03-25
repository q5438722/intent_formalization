# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__allocator__page_allocator_spec_impl__impl2__alloc_and_map_2m/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: ret_already_mapped → `alloc_and_map_2m`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, alloc_and_map_2m could return a page that was already mapped, so mapped_pages_2m never actually grows

### φ2: ret_not_from_free_list → `alloc_and_map_2m`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, the spec allows returning a page that was never in the free list, meaning the free set is unchanged but a phantom page gets mapped

### φ3: ret_in_free_pages_1g → `alloc_and_map_2m`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, a 2m allocation returns a page that is simultaneously tracked as a free 1g page, indicating cross-size-class contamination

### φ4: ret_in_mapped_4k_after → `alloc_and_map_2m`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, the newly mapped 2m page also appears in the 4k mapped set, violating page-size exclusivity

### φ5: ret_was_in_allocated_2m → `alloc_and_map_2m`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, the returned page was already in the allocated set while also being free, violating state mutual exclusion enforced by wf()

