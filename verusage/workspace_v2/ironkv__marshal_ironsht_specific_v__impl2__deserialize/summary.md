# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_ironsht_specific_v/marshal_ironsht_specific_v__impl2__deserialize.rs`
**Date:** 2026-03-24T09:07:59Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 0

## All Candidates

### φ1: from_vec_to_vec_not_roundtrip
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_from_vec` only guarantees `from_vec(v).to_vec() == v` for sorted inputs but never states injectivity — two distinct sorted vectors could map to the same CKeyHashMap, or equivalently the roundtrip axiom could be vacuously true if `spec_from_vec` is unconstrained

### φ2: unsorted_from_vec_unconstrained
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `lemma_from_vec` only constrains `from_vec(v).to_vec()` when `spec_sorted_keys(v)` holds — for unsorted input the result is completely uninterpreted, so the SMT solver could derive any property about unsorted from_vec results

### φ3: ckeyhashmap_is_marshalable_but_to_vec_empty
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** An empty `to_vec()` is trivially sorted, trivially marshalable, and fits within the size limit — so any CKeyHashMap with an empty key-vector is marshalable regardless of what logical state the hashmap actually holds, since `spec_to_vec` is uninterpreted

### φ4: sorted_keys_empty_trivially_true
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `spec_sorted_keys` is vacuously true for empty and single-element vectors — the `forall` quantifier over adjacent pairs has no witnesses, so empty vectors pass the sorted check without ever validating any ordering property

### φ5: deserialize_external_body_no_injectivity
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `deserialize` is `external_body` with ensures that only relate the output to the input subrange — but there is no axiom that `ghost_serialize` is injective, so two different byte sequences could deserialize to the same value, or two identical values could have different serializations

