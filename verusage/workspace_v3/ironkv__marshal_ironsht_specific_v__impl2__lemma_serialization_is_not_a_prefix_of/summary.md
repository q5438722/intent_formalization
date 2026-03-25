# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__marshal_ironsht_specific_v__impl2__lemma_serialization_is_not_a_prefix_of/original.rs`
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
- **Why flagged:** An empty map should never produce a non-empty vector representation

### φ2: single_entry_vec_length_exceeds_one → `to_vec`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A map with exactly one entry should not produce a vector with more than one element

### φ3: equal_view_different_vec_len → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Maps with identical abstract views must produce vectors of the same length; differing lengths would break the bijection

### φ4: vec_key_not_in_domain → `to_vec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Every key appearing in the vector representation should belong to the map's domain

### φ5: duplicate_keys_in_vec → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A faithful map-to-vector conversion must not contain duplicate keys since maps have unique keys

