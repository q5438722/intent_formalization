# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__net_sht_v__sht_demarshall_data_method/original.rs`
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
- **Why flagged:** If entailed, to_vec ignores all hashmap contents and always returns an empty vector

### φ2: to_vec_length_at_most_one → `to_vec`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, to_vec can never faithfully represent a hashmap with more than one entry

### φ3: to_vec_serialize_constant → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, serialization of to_vec is identical for all hashmaps, destroying injectivity

### φ4: to_vec_all_view_equal → `to_vec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, to_vec produces view-equal results for any two hashmaps, collapsing all maps to one value

### φ5: to_vec_serialize_len_too_small → `to_vec`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, serialized to_vec output is at most 8 bytes (a single usize length prefix), meaning element data is never serialized

