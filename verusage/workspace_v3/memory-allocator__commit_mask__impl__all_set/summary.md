# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/memory-allocator__commit_mask__impl__all_set/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_true → `all_set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If subset_of always holds, all_set would be trivially true regardless of inputs

### φ2: always_false → `all_set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If subset_of never holds, all_set would be trivially false regardless of inputs

### φ3: symmetric_subset → `all_set`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Subset is not symmetric in general; this holding would mean all_set implies mutual containment

### φ4: empty_self_contains_all → `all_set`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A zero mask should have an empty view; nothing non-empty should be its subset

### φ5: subset_implies_equal → `all_set`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Subset does not imply equality; this holding would collapse all_set to an equality check

