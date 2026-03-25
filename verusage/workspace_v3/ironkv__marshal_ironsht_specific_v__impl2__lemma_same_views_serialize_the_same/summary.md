# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__marshal_ironsht_specific_v__impl2__lemma_same_views_serialize_the_same/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: empty_map_nonempty_vec → `to_vec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** An empty map should never produce a non-empty vector of key-value pairs

### φ2: key_not_in_domain → `to_vec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Every key in the to_vec result should be present in the map's domain

### φ3: value_mismatch → `to_vec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The value for a key in to_vec should match the corresponding value in the map

### φ4: duplicate_keys → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The to_vec result should not contain duplicate keys since it represents a map

### φ5: nonempty_map_empty_vec → `to_vec`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A map with at least one key should never produce an empty vector representation

