# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_v/marshal_v__impl0__lemma_serialize_injective.rs`
**Date:** 2026-03-24T09:26:10Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `usize::lemma_serialize_injective` is unnecessarily `external_body` when it could be proved from the existing u64 injectivity proof, creating an unneeded trusted assumption. The remaining three candidates are false positives — non-marshalable exclusion, universal u64 marshalability, and fixed 8-byte serialization length are all correct, expected properties of the serialization design.

## True Positives (Spec Issues)

### usize_serialize_injective_external_body
- **Confidence:** medium
- **Reasoning:** The `usize::lemma_serialize_injective` is marked `external_body` despite being straightforwardly provable from the u64 implementation (usize serializes as `(*self as u64).ghost_serialize()` and u64 injectivity is already proved). This is a missed proof opportunity that leaves a trusted assumption where none is needed.

## All Candidates

### φ1: usize_serialize_injective_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `usize::lemma_serialize_injective` is `external_body` — injectivity of usize serialization is trusted without proof, despite being provable from the u64 injectivity since usize serializes via cast to u64
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `usize::lemma_serialize_injective` is marked `external_body` despite being straightforwardly provable from the u64 implementation (usize serializes as `(*self as u64).ghost_serialize()` and u64 injectivity is already proved). This is a missed proof opportunity that leaves a trusted assumption where none is needed.

### φ2: usize_distinct_values_same_serialize
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Non-marshalable usize values (above u64::MAX on hypothetical >64-bit platforms) have `ghost_serialize` governed only by a `recommends` — the trait-level `external_body` ghost_serialize could map them to anything, potentially collapsing distinct values to the same serialization with no way to detect it
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This merely confirms that values outside the marshalable domain are correctly flagged as non-marshalable. The `is_marshalable` guard is the intended mechanism to exclude such values, and the behavior of `ghost_serialize` on non-marshalable inputs is intentionally unspecified (via `recommends`).

### φ3: u64_all_marshalable
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Every u64 is unconditionally marshalable — there is no size bound on the serialization output, so callers cannot bound buffer sizes from `is_marshalable` alone without additional knowledge about serialization length
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Every u64 being marshalable is the correct and intended design — u64 has a fixed-size 8-byte LE encoding that always succeeds. Buffer sizing is a separate concern handled by serialization length, not by `is_marshalable`.

### φ4: ghost_serialize_trait_uninterpreted_for_marshalable
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The injectivity axiom combined with `view_equal` being `self@ === other@` means serialization injectivity implies value equality for u64 — but if the ensures direction were reversed (proving distinct values have distinct serializations), it would rely on contrapositive of an external_body axiom which may not be sound

### φ5: usize_ghost_serialize_len_equals_u64
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Marshalable usize serialization always produces exactly 8 bytes (same as u64) — this fixed length is implicit and never stated as a spec guarantee, so any consumer relying on variable-length encoding or length < 8 would silently break
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct and expected consequence of usize serializing via cast to u64, which always produces 8 bytes of LE encoding. The fixed length is a desirable property of the serialization format, not a spec gap.

