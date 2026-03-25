# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_v/marshal_v__impl4__lemma_serialize_injective.rs`
**Date:** 2026-03-24T09:39:53Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 5
- False positives: 0

## Summary

All five candidates are true positives identifying the same systemic pattern: multiple `external_body` lemmas across u64, usize, Vec<u8>, and Vec<T> implementations trust serialization injectivity and prefix-freedom axioms that are provable from the open spec definitions. The most impactful is φ4, where Vec<T>'s "verified" injectivity proof chains through Vec<T>'s unverified prefix-freedom external_body, meaning the entire generic Vec serialization injectivity guarantee is only as sound as that trusted axiom. The others (φ1–3, φ5) are individually provable properties left unnecessarily as external_body.

## True Positives (Spec Issues)

### u64_prefix_freedom_external_body
- **Confidence:** medium
- **Reasoning:** `u64::lemma_serialization_is_not_a_prefix_of` is `external_body` when all u64 serializations are fixed 8 bytes, making prefix-freedom equivalent to injectivity — provable from `lemma_auto_spec_u64_to_from_le_bytes` but left unverified.

### usize_serialize_injective_external_body
- **Confidence:** medium
- **Reasoning:** `usize::lemma_serialize_injective` is `external_body` despite usize serializing via cast to u64 — injectivity follows directly from u64 injectivity plus the cast being injective for values ≤ u64::MAX. Unnecessarily trusted.

### vec_u8_prefix_freedom_external_body
- **Confidence:** medium
- **Reasoning:** `Vec<u8>::lemma_serialization_is_not_a_prefix_of` is `external_body` despite being provable from the length-prefix structure — if two Vec<u8> have different views, their length prefixes or data differ, which is derivable from usize serialization properties.

### vec_t_injectivity_depends_on_prefix_freedom
- **Confidence:** medium
- **Reasoning:** `Vec<T>::lemma_serialize_injective` is verified but calls `self.lemma_serialization_is_not_a_prefix_of(other)` which is `external_body` for Vec<T>. The verified proof's soundness depends entirely on this unverified prefix-freedom axiom.

### vec_u8_serialize_injective_external_body
- **Confidence:** medium
- **Reasoning:** `Vec<u8>::lemma_serialize_injective` is `external_body` despite being provable — equal serializations means equal length prefixes (hence equal lengths) and equal data subranges. This is a straightforward consequence of the length-prefix format.

## All Candidates

### φ1: u64_prefix_freedom_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `u64::lemma_serialization_is_not_a_prefix_of` is `external_body` despite being provable — all u64 serializations are 8 bytes so prefix-freedom reduces to injectivity of `spec_u64_to_le_bytes`
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `u64::lemma_serialization_is_not_a_prefix_of` is `external_body` when all u64 serializations are fixed 8 bytes, making prefix-freedom equivalent to injectivity — provable from `lemma_auto_spec_u64_to_from_le_bytes` but left unverified.

### φ2: usize_serialize_injective_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `usize::lemma_serialize_injective` is `external_body` despite being provable from u64 injectivity — full equality of usize values rests on an unverified axiom
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `usize::lemma_serialize_injective` is `external_body` despite usize serializing via cast to u64 — injectivity follows directly from u64 injectivity plus the cast being injective for values ≤ u64::MAX. Unnecessarily trusted.

### φ3: vec_u8_prefix_freedom_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `Vec<u8>::lemma_serialization_is_not_a_prefix_of` is `external_body` — prefix-freedom for length-prefixed byte vectors is provable from usize serialization properties but left entirely trusted
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `Vec<u8>::lemma_serialization_is_not_a_prefix_of` is `external_body` despite being provable from the length-prefix structure — if two Vec<u8> have different views, their length prefixes or data differ, which is derivable from usize serialization properties.

### φ4: vec_t_injectivity_depends_on_prefix_freedom
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `Vec<T>::lemma_serialize_injective` is verified but its proof calls `lemma_serialization_is_not_a_prefix_of` which is `external_body` for `Vec<T>` — injectivity of generic Vec serialization chains through an unverified prefix-freedom axiom
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `Vec<T>::lemma_serialize_injective` is verified but calls `self.lemma_serialization_is_not_a_prefix_of(other)` which is `external_body` for Vec<T>. The verified proof's soundness depends entirely on this unverified prefix-freedom axiom.

### φ5: vec_u8_serialize_injective_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `Vec<u8>::lemma_serialize_injective` is `external_body` — injectivity of length-prefixed byte vector serialization is entirely trusted despite being provable from the length-prefix structure
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `Vec<u8>::lemma_serialize_injective` is `external_body` despite being provable — equal serializations means equal length prefixes (hence equal lengths) and equal data subranges. This is a straightforward consequence of the length-prefix format.

