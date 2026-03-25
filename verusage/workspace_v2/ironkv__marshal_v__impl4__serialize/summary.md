# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_v/marshal_v__impl4__serialize.rs`
**Date:** 2026-03-24T09:42:33Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 0

## All Candidates

### φ1: u64_serialize_external_body_unimplemented
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `u64::serialize` is `external_body` with `unimplemented!()` — the exec body panics at runtime while the spec ensures (prefix preservation + appended serialization) are trusted without verification

### φ2: usize_serialize_external_body_unimplemented
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `usize::serialize` is `external_body` with `unimplemented!()` — the exec body panics at runtime while the trait-level ensures clauses are trusted without any implementation

### φ3: vec_u8_serialize_external_body_unimplemented
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `Vec<u8>::serialize` is `external_body` with `unimplemented!()` — the spec-exec correspondence for length-prefixed byte vector serialization is entirely trusted, and the exec body panics at runtime

### φ4: fold_left_append_right_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_seq_fold_left_append_right` is `external_body` — the fold decomposition (prefix fold + last element = full fold) is entirely trusted, and if incorrect would silently corrupt all Vec<T>::serialize proofs

### φ5: trait_serialize_external_body_default
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The trait-level `Marshalable::serialize` is `external_body` — the default impl is `unimplemented!()` and all ensures clauses (prefix preservation, appended serialization) are globally trusted for any type that doesn't override with a verified implementation

