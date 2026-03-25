# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_ironsht_specific_v/marshal_ironsht_specific_v__impl2__lemma_serialization_is_not_a_prefix_of.rs`
**Date:** 2026-03-24T09:14:26Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

Two of three candidates are false positives: the reflexivity test is trivially true and u64's `view_equal` symmetry is a direct consequence of its definition. The true positive is `to_vec_view_biconditional_unsound`, which exploits the external_body `lemma_to_vec_view` to derive that the uninterpreted `view` is fully determined by `spec_to_vec` — a strong trusted assumption that could be unsound if the HashMap view carries information not captured by the sorted key-value vector representation.

## True Positives (Spec Issues)

### to_vec_view_biconditional_unsound
- **Confidence:** medium
- **Reasoning:** The `external_body` `lemma_to_vec_view` asserts a biconditional that allows deriving `h1@ == h2@` from `h1.spec_to_vec()@ == h2.spec_to_vec()@`. Since both `view` and `spec_to_vec` are uninterpreted, this trusted axiom forces them to carry identical information — if the real HashMap `view` distinguishes states that produce the same sorted key-value vector (e.g., different internal orderings or metadata), the axiom is unsound. This direction of the biconditional is a strong unverified assumption.

## All Candidates

### φ1: lemma_to_vec_view_reflexive_trivial
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Trivially true — but the real concern is that `lemma_to_vec_view` is external_body and its biconditional could be used to derive non-trivial facts about the uninterpreted `spec_to_vec` from equality of uninterpreted `view`, without any machine-checked justification
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Both `h@ == h@` and `h.spec_to_vec()@ == h.spec_to_vec()@` are trivially true by reflexivity of equality, independent of the lemma. The lemma call is unnecessary and the property reveals nothing about the external_body axiom's soundness.

### φ2: view_equal_not_symmetric_for_u64
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `view_equal` is used in `lemma_serialization_is_not_a_prefix_of` as the negation trigger — if `view_equal` is not actually an equivalence relation for all Marshalable impls, the prefix-freedom axiom could have gaps; this tests symmetry for the base case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** For `u64`, `view_equal` is defined as `self@

### φ3: prefix_freedom_derives_inequality
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `lemma_serialization_is_not_a_prefix_of` is external_body and grants prefix-freedom — combined with same-length cases this derives full serialization injectivity, but since `ghost_serialize` for `Vec<u8>` is open (length prefix + content), this should hold; if it doesn't, the axiom is too weak

### φ4: tuple_serialize_no_delimiter
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Tuple serialization is simple concatenation with no length delimiter between fields — the external_body prefix-freedom axiom claims this is fine, but if the first field's serialization length varies, parsing could misalign without a delimiter, making the axiom unsound

### φ5: to_vec_view_biconditional_unsound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `lemma_to_vec_view` is external_body and asserts a biconditional between `view` equality and `to_vec` view-equality — this direction (to_vec equal implies view equal) means the uninterpreted `view` is fully determined by `spec_to_vec`, which may over-constrain the model if the real HashMap view includes information not captured by the sorted key-value vector
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `external_body` `lemma_to_vec_view` asserts a biconditional that allows deriving `h1@ == h2@` from `h1.spec_to_vec()@ == h2.spec_to_vec()@`. Since both `view` and `spec_to_vec` are uninterpreted, this trusted axiom forces them to carry identical information — if the real HashMap `view` distinguishes states that produce the same sorted key-value vector (e.g., different internal orderings or metadata), the axiom is unsound. This direction of the biconditional is a strong unverified assumption.

