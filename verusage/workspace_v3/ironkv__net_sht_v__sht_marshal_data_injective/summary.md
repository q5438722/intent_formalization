# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__net_sht_v__sht_marshal_data_injective/original.rs`
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
- **Why flagged:** to_vec should not always return an empty vector for every marshalable hash map

### φ2: to_vec_at_most_one → `to_vec`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Hash maps can contain many entries so to_vec should be able to return more than one element

### φ3: to_vec_constant_function → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Different hash maps should generally produce different to_vec results so a constant function implies total information loss

### φ4: to_vec_all_elements_identical → `to_vec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A hash map with multiple distinct entries should produce distinct key-value pairs in to_vec not identical copies

### φ5: to_vec_ignores_content → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Maps with different serialized content represent different data so to_vec must reflect that difference in its output

