# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_exists_implies_equality1.rs`
**Date:** 2026-03-24T04:26:35Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Three true positives from independent external_body axioms: `temp_pred_equality` (intensional equality from semantic equivalence), `a_to_temp_pred_equality` (function-level intensional equality from pointwise equivalence), and `tla_exists_or_equality` (exists-or distribution as intensional equality). A fourth demonstrates the soundness consequence of deriving false from distinct-but-equivalent preds. One false positive: the exists-implies commutation is a correct logical equivalence with no independent trust surface.

## True Positives (Spec Issues)

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Unverified trust assumption.

### a_to_temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `a_to_temp_pred_equality` is external_body forcing intensional equality of `spec_fn(A) -> TempPred<T>` from pointwise mutual entailment. Independent unverified trust assumption lifting the closure collapse to function-level equality.

### tla_exists_or_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `tla_exists_or_equality` is external_body forcing unconditional intensional equality between `tla_exists(|a| p(a).or(q))` and `tla_exists(p).or(q)`. Independent unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — derives `false` from intensionally distinct but semantically equivalent preds.

## All Candidates

### φ1: temp_pred_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body forcing intensional equality from mutual entailment — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Unverified trust assumption.

### φ2: a_to_temp_pred_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** a_to_temp_pred_equality is external_body forcing intensional equality of spec_fn(A)->TempPred from pointwise mutual entailment — collapses structurally distinct function-returning-closure families without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `a_to_temp_pred_equality` is external_body forcing intensional equality of `spec_fn(A) -> TempPred<T>` from pointwise mutual entailment. Independent unverified trust assumption lifting the closure collapse to function-level equality.

### φ3: tla_exists_or_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_exists_or_equality is external_body forcing unconditional intensional equality between tla_exists(|a| p(a).or(q)) and tla_exists(p).or(q) — structurally distinct nested closures collapsed without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_exists_or_equality` is external_body forcing unconditional intensional equality between `tla_exists(|a| p(a).or(q))` and `tla_exists(p).or(q)`. Independent unverified trust assumption.

### φ4: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p == q from mutual entailment — if two preds are intensionally distinct but semantically equivalent this derives false
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — derives `false` from intensionally distinct but semantically equivalent preds.

### φ5: tla_exists_implies_equality_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_exists_implies_equality1 chains all three external_body axioms to force intensional equality between tla_exists(|a| p=>q(a)) and p=>tla_exists(q) — propagates intensional collapse through implies/or/not/exists closure nesting
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `∃a. (p ⟹ q(a)) ≡ p ⟹ ∃a. q(a)` is a standard logical equivalence. This is a verified lemma with no new trust surface beyond φ1–φ3.

