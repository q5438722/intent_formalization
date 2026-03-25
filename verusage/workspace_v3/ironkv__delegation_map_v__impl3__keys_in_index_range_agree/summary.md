# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl3__keys_in_index_range_agree/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: is_lt_greater_returns_true → `is_lt`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If Greater is considered less-than, all ordering comparisons would be corrupted

### φ2: is_lt_equal_returns_true → `is_lt`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If Equal is considered less-than, sorted invariants and deduplication logic would silently break

### φ3: agree_always_true → `keys_in_index_range_agree`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If agreement holds for any v, the map is trivial and the function never returns false in ret.0

### φ4: almost_never_holds → `keys_in_index_range_agree`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the almost-agree case is unreachable, ret.1 is meaningless and callers relying on it would have dead logic

### φ5: singleton_range_always_agrees → `keys_in_index_range_agree`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If a single-element range always agrees with any value, the map collapses to a constant and ret.0 is trivially true for lo==hi

