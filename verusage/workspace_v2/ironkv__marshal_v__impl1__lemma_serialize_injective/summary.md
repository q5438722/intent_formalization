# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_v/marshal_v__impl1__lemma_serialize_injective.rs`
**Date:** 2026-03-24T09:31:17Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives identify the same root issue: `u64::lemma_serialize_injective` is unnecessarily `external_body` when it could be proved from existing vstd lemmas, and usize's injectivity proof inherits this unverified trust. The remaining three are false positives — contrapositive distinctness, fixed-length encoding, and view-equal collapsing to equality for primitives are all correct, expected properties.

## True Positives (Spec Issues)

### u64_serialize_injective_external_body
- **Confidence:** medium
- **Reasoning:** `u64::lemma_serialize_injective` is `external_body` despite being provable from `lemma_auto_spec_u64_to_from_le_bytes`. This is an unnecessary trusted assumption that could be machine-checked.

### usize_injectivity_chains_to_u64_external
- **Confidence:** medium
- **Reasoning:** The usize injectivity proof chains through u64's unverified `external_body` lemma. The entire guarantee rests on a trusted axiom that could instead be proved.

## All Candidates

### φ1: u64_serialize_injective_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `u64::lemma_serialize_injective` is `external_body` — injectivity of u64 LE serialization is trusted without proof despite being provable from `lemma_auto_spec_u64_to_from_le_bytes`
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `u64::lemma_serialize_injective` is `external_body` despite being provable from `lemma_auto_spec_u64_to_from_le_bytes`. This is an unnecessary trusted assumption that could be machine-checked.

### φ2: usize_injectivity_chains_to_u64_external
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** usize injectivity derives `a == b` (not just `view_equal`) — but this chains through `u64`'s unverified `external_body` lemma, so the full equality guarantee for usize rests on an unproved axiom
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The usize injectivity proof chains through u64's unverified `external_body` lemma. The entire guarantee rests on a trusted axiom that could instead be proved.

### φ3: serialize_no_marshalable_guard_trait
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Contrapositive of injectivity gives distinctness of serializations — but relies entirely on the unverified `external_body` u64 injectivity axiom, which if wrong would make distinct u64 values produce identical byte sequences
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the standard contrapositive of serialization injectivity — distinct values having distinct serializations is a correct and desirable property. The fact that it relies on the u64 external_body is the same issue as φ1, not a separate concern.

### φ4: ghost_serialize_fixed_length_u64
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** All u64 serializations have identical length (8 bytes) — this is implicit and never stated as a spec guarantee, so any consumer assuming variable-length encoding would silently break
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Fixed 8-byte LE encoding for u64 is the correct, expected behavior of `spec_u64_to_le_bytes`. This is a desirable property of the serialization format.

### φ5: view_equal_implies_equal_usize
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** For usize, `view_equal` (defined as `self@ === other@`) implies full equality — the `view_equal` abstraction barrier provides no additional flexibility over `==` for primitive types, so trait consumers expecting view-equality to be weaker than equality get no separation
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For primitive types like usize, `view_equal` coinciding with equality is expected — `self@

