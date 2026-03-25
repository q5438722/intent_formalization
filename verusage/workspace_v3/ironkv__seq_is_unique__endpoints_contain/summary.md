# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__seq_is_unique__endpoints_contain/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: empty_vec_always_contains → `endpoints_contain`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** An empty collection should never contain any element; if provable, the spec is vacuously broken for empty inputs

### φ2: nonempty_always_contains → `endpoints_contain`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A non-empty list should not automatically contain every possible endpoint; if provable, the spec over-approximates membership

### φ3: singleton_match_not_found → `endpoints_contain`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A singleton list whose only element matches the target must report containment; if provable, the spec misses a present element

### φ4: never_present → `endpoints_contain`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Non-empty lists can contain a matching endpoint; if provable, the spec forces the function to always return false

### φ5: distinct_ids_conflated → `endpoints_contain`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Endpoints with different-length IDs must be distinct; if provable, the abstract equality in the spec is unsound

