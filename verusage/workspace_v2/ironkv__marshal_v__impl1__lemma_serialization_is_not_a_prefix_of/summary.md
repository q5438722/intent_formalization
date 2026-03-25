# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_v/marshal_v__impl1__lemma_serialization_is_not_a_prefix_of.rs`
**Date:** 2026-03-24T09:28:45Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `u64::lemma_serialization_is_not_a_prefix_of` is unnecessarily `external_body` when prefix-freedom for fixed-length 8-byte serializations is provable from injectivity of LE encoding. The other two candidates are false positives — u64's `view_equal` symmetry follows directly from `===` semantics, and fixed-length serialization is the intended design.

## True Positives (Spec Issues)

### u64_prefix_freedom_external_body
- **Confidence:** medium
- **Reasoning:** `u64::lemma_serialization_is_not_a_prefix_of` is `external_body` despite being provable — since all u64 serializations are 8 bytes, prefix-freedom reduces to injectivity of `spec_u64_to_le_bytes`, which is a provable fact. This is an unnecessary trusted assumption.

## All Candidates

### φ1: u64_prefix_freedom_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `u64::lemma_serialization_is_not_a_prefix_of` is `external_body` — prefix-freedom for u64 is entirely trusted without proof, despite being provable from `spec_u64_to_le_bytes` fixed-length property
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `u64::lemma_serialization_is_not_a_prefix_of` is `external_body` despite being provable — since all u64 serializations are 8 bytes, prefix-freedom reduces to injectivity of `spec_u64_to_le_bytes`, which is a provable fact. This is an unnecessary trusted assumption.

### φ2: usize_prefix_delegates_to_u64_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `usize` prefix-freedom proof delegates to `u64`'s `external_body` lemma — the entire chain of prefix-freedom reasoning for usize rests on an unverified u64 axiom

### φ3: prefix_no_marshalable_guard
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `lemma_serialization_is_not_a_prefix_of` has no `is_marshalable` precondition — prefix-freedom is asserted even for values where `ghost_serialize` is only governed by `recommends`, not `requires`

### φ4: view_equal_no_symmetry_enforced
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The trait never requires `view_equal` to be symmetric — an asymmetric implementation could allow `a.lemma_serialization_is_not_a_prefix_of(&b)` to fire in one direction but not the other, creating inconsistent prefix-freedom guarantees
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For u64, `view_equal` is defined as `self@

### φ5: u64_fixed_length_trivializes_prefix_freedom
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** All u64 serializations have the same length (8 bytes), making the prefix-freedom lemma's `len() <= len()` precondition always true — the "not a prefix" guarantee reduces to simple injectivity, yet this is hidden behind an `external_body` that claims a stronger property than needed
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** All u64 serializations being 8 bytes is a correct and expected consequence of `spec_u64_to_le_bytes` producing fixed-length little-endian encodings. This is a desirable property of the serialization format, not a spec gap.

