# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__marshal_ironsht_specific_v__lemma_is_marshalable_CKeyHashMap/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: nonempty_map_empty_vec → `to_vec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, to_vec could silently discard all entries from a non-empty map

### φ2: vec_len_exceeds_domain → `to_vec`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, to_vec could fabricate extra entries not backed by the underlying map

### φ3: duplicate_keys_in_vec → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, the sorted-keys invariant fails to prevent duplicate keys, breaking map semantics

### φ4: value_mismatch_in_vec → `to_vec`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, to_vec could associate a key with the wrong value, silently corrupting data

### φ5: roundtrip_failure → `to_vec`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, the from_vec/to_vec roundtrip is broken, meaning serialization loses information

