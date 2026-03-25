# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_ironsht_specific_v/marshal_ironsht_specific_v__lemma_is_marshalable_CKeyHashMap.rs`
**Date:** 2026-03-24T09:18:09Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

Two of three candidates are false positives: the empty-fold base case is definitionally true and the serialization length bound is a tautological restatement of an `is_marshalable` conjunct. The one true positive is the `external_body` `lemma_seq_fold_left_append_len_int`, which establishes a critical correspondence between byte-level and integer-level fold computations without machine-checked proof — this axiom underpins all serialization length calculations and any error in its statement would propagate silently.

## True Positives (Spec Issues)

### lemma_seq_fold_left_append_len_unchecked
- **Confidence:** medium
- **Reasoning:** `lemma_seq_fold_left_append_len_int` is `external_body` and asserts that the length of a byte-concatenation fold equals the corresponding integer-sum fold. While mathematically correct, this is a fundamental property used throughout all serialization length reasoning and is never machine-checked — any error in the statement (e.g., off-by-one in the inductive step) would silently corrupt all downstream bounds.

## All Candidates

### φ1: lemma_to_vec_external_body_from_vec_roundtrip
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `lemma_to_vec` asserts `from_vec(to_vec(h)) == h` — combined with uninterpreted `spec_from_vec`, if two hashmaps have view-equal to_vec results, the axiom doesn't guarantee the hashmaps themselves are equal since `spec_from_vec` is uninterpreted and `to_vec` view-equality doesn't imply `to_vec` equality

### φ2: fold_left_sum_external_body_negative
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_seq_fold_left_sum_le` is external_body and could allow deriving incorrect bounds — an empty sequence fold should be 0 but the axiom is trusted without machine-checked base case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An empty sequence's `fold_left` with init 0 returning 0 is the definitional base case of `fold_left` — this holds by the semantics of `Seq::fold_left` itself, not by any external_body axiom. The property is trivially correct.

### φ3: lemma_to_vec_dom_len_no_injectivity
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `lemma_to_vec` is external_body and asserts `to_vec().len() == view.dom().len()` — but since `view` is uninterpreted, this axiom constrains the relationship between two uninterpreted functions without any grounding, and a wrong implementation could satisfy the spec vacuously

### φ4: ckeyhashmap_serialize_len_unchecked
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The `is_marshalable` for `CKeyHashMap` directly includes `ghost_serialize().len() <= ckeyhashmap_max_serialized_size()` — but this bound (0x100000 = 1MiB) is arbitrarily chosen, and the external_body `lemma_seq_fold_left_sum_le` used to prove it could be unsound for the specific fold patterns used
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `is_marshalable` spec for `CKeyHashMap` explicitly includes `self.to_vec().ghost_serialize().len() <= ckeyhashmap_max_serialized_size() as int` as a conjunct. The ensures is a direct restatement of a precondition — this is tautological, not a spec gap.

### φ5: lemma_seq_fold_left_append_len_unchecked
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_seq_fold_left_append_len_int` is external_body and equates byte-level fold length with integer-level fold sum — this correspondence is trusted without proof and any discrepancy would silently corrupt all serialization length calculations
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `lemma_seq_fold_left_append_len_int` is `external_body` and asserts that the length of a byte-concatenation fold equals the corresponding integer-sum fold. While mathematically correct, this is a fundamental property used throughout all serialization length reasoning and is never machine-checked — any error in the statement (e.g., off-by-one in the inductive step) would silently corrupt all downstream bounds.

