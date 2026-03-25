# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_v/marshal_v__impl2__lemma_serialize_injective.rs`
**Date:** 2026-03-24T09:32:28Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 0

## All Candidates

### φ1: vec_u8_injectivity_proved
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Vec<u8> injectivity is verified (not external_body) but chains through u64's external_body injectivity for the length prefix — the proved Vec<u8> injectivity is only as trustworthy as the unverified u64 axiom it depends on

### φ2: vec_t_serialize_injective_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `Vec<T>::lemma_serialize_injective` is `external_body` — injectivity of generic Vec serialization (length prefix + fold of elements) is entirely trusted without proof

### φ3: usize_serialize_injective_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `usize::lemma_serialize_injective` is `external_body` despite serializing via cast to u64 — provable from u64 injectivity but left unverified

### φ4: vec_u8_length_prefix_fixed_8
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The length prefix for Vec<u8> is always exactly 8 bytes (u64 LE encoding of a usize cast) — this fixed overhead is implicit and never stated, so consumers cannot rely on it without knowing the usize→u64 serialization detail

### φ5: empty_vec_serialize_not_empty
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** An empty Vec<u8> serializes to 8 bytes (the length prefix) rather than 0 — the serialization format has non-trivial overhead that makes `ghost_serialize().len() == 0` impossible for any Vec, potentially surprising consumers expecting empty containers to have empty serializations

