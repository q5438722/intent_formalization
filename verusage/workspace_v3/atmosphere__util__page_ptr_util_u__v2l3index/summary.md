# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__util__page_ptr_util_u__v2l3index/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_zero_index → `v2l3index`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean the L3 index space collapses to a single entry, making page table walking trivially broken

### φ2: kernel_l4_bound_leaks_to_l3 → `v2l3index`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Would mean the L4 kernel-memory lower bound incorrectly constrains L3 indices, making index 0 unreachable

### φ3: distinct_va_always_collide → `v2l3index`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean all distinct valid addresses share the same L3 index, destroying any address-space partitioning at level 3

### φ4: 1g_valid_forces_zero_index → `v2l3index`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Would mean 1GB-aligned pages can only occupy L3 entry 0, preventing multiple 1GB mappings in the same L3 table

### φ5: upper_half_index_unreachable → `v2l3index`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean indices 256–511 are unreachable, silently halving the usable L3 page-table entry space

