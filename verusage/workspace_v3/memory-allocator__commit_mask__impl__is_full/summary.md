# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/memory-allocator__commit_mask__impl__is_full/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: full_missing_zero → `is_full`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If is_full returns true yet index 0 is absent, the ensures clause does not correctly characterize the full set

### φ2: always_full → `is_full`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If is_full always returns true regardless of mask content, the spec is vacuously weak

### φ3: false_implies_empty → `is_full`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If not-full implies empty, the spec conflates partial occupancy with emptiness

### φ4: full_is_half_range → `is_full`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the full set equals [0,256) instead of [0,512), COMMIT_MASK_BITS is incorrectly halved

### φ5: full_contains_out_of_range → `is_full`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the full set contains index 512 the upper bound is off-by-one and the mask covers more bits than intended

