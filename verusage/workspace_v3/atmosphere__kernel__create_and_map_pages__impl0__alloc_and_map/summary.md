# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__create_and_map_pages__impl0__alloc_and_map/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: exec_disable_on_new_mapping → `alloc_and_map`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Newly allocated page should not have execution disabled; spec never explicitly constrains ret.execute_disable so this tests for accidental over-constraint

### φ2: write_perm_missing_on_new_mapping → `alloc_and_map`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Newly allocated page should be writable; spec never explicitly constrains ret.write so this checks whether the spec accidentally forces read-only pages

### φ3: old_mapping_destroyed → `alloc_and_map`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Existing address space mappings for other VAs must be preserved; if provable, the insert-based ensures loses pre-existing entries

### φ4: page_shared_across_procs → `alloc_and_map`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A freshly allocated page must belong to exactly one process; if provable, the page_mapping singleton set leaks the page to another process

### φ5: free_pages_not_decreased → `alloc_and_map`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Allocation must consume a free page; if provable, the free page count is non-decreasing which means pages are created from nothing

