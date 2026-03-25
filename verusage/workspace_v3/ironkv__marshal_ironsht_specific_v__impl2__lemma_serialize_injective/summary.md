# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__marshal_ironsht_specific_v__impl2__lemma_serialize_injective/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: to_vec_always_empty → `to_vec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If marshalability forces to_vec to be empty, the hashmap can never faithfully represent any key-value data

### φ2: to_vec_at_most_one_entry → `to_vec`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the size bound or sorted-keys constraint accidentally limits to_vec to at most one element, the map is trivially degenerate

### φ3: to_vec_serialize_collision → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If distinct maps serialize identically via to_vec, deserialization cannot recover the original map, breaking injectivity

### φ4: to_vec_values_always_empty → `to_vec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the spec forces all values to be empty byte vectors, the hashmap cannot store any meaningful payload data

### φ5: sorted_keys_forces_all_equal → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If spec_sorted_keys combined with marshalability collapses all keys to the same value, the map can hold at most one logical entry

