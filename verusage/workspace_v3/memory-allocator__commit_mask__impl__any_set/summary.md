# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/memory-allocator__commit_mask__impl__any_set/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_disjoint → `any_set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean any_set always returns false, making the overlap check useless

### φ2: zero_mask_nonempty_view → `any_set`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A mask with all zero words should have an empty view set; a non-empty view would corrupt disjointness checks

### φ3: never_disjoint → `any_set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean any_set always returns true, making it impossible to detect non-overlapping masks

### φ4: view_always_empty → `any_set`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would collapse every CommitMask view to the empty set, making any_set trivially return false regardless of actual bits

### φ5: full_mask_self_disjoint → `any_set`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A mask with all bits set must overlap with itself; self-disjointness here would indicate the view spec is broken

