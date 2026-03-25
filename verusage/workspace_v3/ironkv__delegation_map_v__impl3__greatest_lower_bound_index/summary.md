# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl3__greatest_lower_bound_index/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: equal_is_lt → `is_lt`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If Equal is classified as Less, is_lt would return true on Equal inputs, corrupting comparison logic

### φ2: greater_is_lt → `is_lt`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If Greater is classified as Less, is_lt conflates opposite orderings, breaking all ordering-dependent code

### φ3: glb_ignores_later_keys → `greatest_lower_bound_index`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** When iter equals the last key, the GLB should be that key itself, not the minimum key — this would mean the search never advances past index 0

### φ4: glb_never_matches_iter → `greatest_lower_bound_index`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the GLB is always strictly less than iter and never equal, exact key lookups would return the wrong lower bound, missing the matching key

### φ5: end_iter_glb_is_min → `greatest_lower_bound_index`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For an end iterator the GLB should be the greatest key, not the least — returning index 0 would discard all keys above the minimum

