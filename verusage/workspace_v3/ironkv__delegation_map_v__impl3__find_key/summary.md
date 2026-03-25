# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl3__find_key/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_returns_none → `find_key`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If verified, find_key can never return Some, meaning no key can ever be found in a valid map.

### φ2: valid_implies_empty_domain → `find_key`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If verified, valid() forces an empty domain, making find_key's precondition trivially guarantee None for every key.

### φ3: found_key_not_in_domain → `find_key`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If verified, a key present in the sorted key vector is absent from the map domain, making find_key's Some and None postconditions mutually inconsistent.

### φ4: index_always_zero → `find_key`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If verified, a key can only reside at index 0, meaning valid maps degenerate to at most one entry.

### φ5: no_two_distinct_keys → `find_key`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If verified, distinct indices cannot hold distinct keys, violating the no-duplicates invariant and collapsing the map to a single-key structure.

