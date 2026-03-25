# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__single_delivery_model_v__same_view_same_marshalable/original.rs`
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
- **Why flagged:** If entailed, to_vec would always return an empty vector regardless of map contents, losing all entries.

### φ2: to_vec_len_always_one → `to_vec`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, to_vec collapses any multi-entry map to at most one element, silently discarding data.

### φ3: to_vec_ignores_map_contents → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, to_vec produces identical output for any two maps, meaning the result is independent of actual contents.

### φ4: to_vec_not_marshalable → `to_vec`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, the result of to_vec on a marshalable map is itself not marshalable, breaking serialization round-trips.

### φ5: to_vec_view_equal_symmetric_fail → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, view-equal maps produce different to_vec results, violating the extensional equality contract.

