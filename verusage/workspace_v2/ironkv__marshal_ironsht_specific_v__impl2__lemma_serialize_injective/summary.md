# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_ironsht_specific_v/marshal_ironsht_specific_v__impl2__lemma_serialize_injective.rs`
**Date:** 2026-03-24T09:15:41Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One of two candidates is a true positive. The empty-vector serialization injectivity test is a false positive — it's a correct and expected consequence of the spec. The missing `is_marshalable` guard on `lemma_serialization_is_not_a_prefix_of` is a true positive: the external_body axiom grants prefix-freedom for all values regardless of marshalability, but `ghost_serialize` is only meaningfully specified for marshalable values, creating a potential soundness gap where the axiom could be applied to values with uninterpreted serializations.

## True Positives (Spec Issues)

### prefix_freedom_no_marshalable_guard
- **Confidence:** medium
- **Reasoning:** `lemma_serialization_is_not_a_prefix_of` is `external_body` with no `is_marshalable` precondition, yet `ghost_serialize` only has a `recommends` (not `requires`) for `is_marshalable`. For non-marshalable values, `ghost_serialize` at the trait level is an uninterpreted `external_body` spec — the prefix-freedom guarantee is asserted without any basis for values outside the marshalable domain, potentially allowing unsound reasoning about arbitrary byte sequences.

## All Candidates

### φ1: serialize_injective_derives_view_equal_from_empty
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Two empty `Vec<u8>` values have identical serializations (same length prefix + empty content), so `lemma_serialize_injective` derives `view_equal` — but this relies on the external_body axiom being correct without machine-checked proof
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Two empty `Vec<u8>` values are view-equal by definition (`self@

### φ2: ghost_serialize_external_body_unconstrained
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `lemma_serialize_injective` is external_body and claims `ghost_serialize` equal implies `view_equal` — by contrapositive this derives serialization distinctness for distinct u64 values, but the trait-level `ghost_serialize` is also external_body with no axioms connecting the trait-level spec to the impl-level open spec

### φ3: prefix_freedom_no_marshalable_guard
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_serialization_is_not_a_prefix_of` has no `is_marshalable` precondition — it grants prefix-freedom for ANY values including non-marshalable ones, where `ghost_serialize` is only guarded by a `recommends` and could be arbitrary
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_serialization_is_not_a_prefix_of` is `external_body` with no `is_marshalable` precondition, yet `ghost_serialize` only has a `recommends` (not `requires`) for `is_marshalable`. For non-marshalable values, `ghost_serialize` at the trait level is an uninterpreted `external_body` spec — the prefix-freedom guarantee is asserted without any basis for values outside the marshalable domain, potentially allowing unsound reasoning about arbitrary byte sequences.

### φ4: vec_t_fold_serialize_collision
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `Vec<Vec<u8>>` serialization concatenates inner element serializations without per-element delimiters — the external_body `lemma_serialize_injective` claims injectivity but this is never machine-verified; if the fold-based concatenation has collisions, the axiom is unsound

### φ5: sorted_keys_singleton_trivially_sorted
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A single-element vector is vacuously sorted since the forall over adjacent pairs has no witnesses — combined with `CKeyHashMap`'s marshalability requiring `spec_sorted_keys`, any single-entry hashmap is trivially marshalable regardless of key validity

