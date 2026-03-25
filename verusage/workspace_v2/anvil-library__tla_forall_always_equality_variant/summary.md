# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_forall_always_equality_variant.rs`
**Date:** 2026-03-24T04:29:29Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 5
- False positives: 0

## Summary

Five true positives: three from independent external_body axioms (`temp_pred_equality`, `a_to_temp_pred_equality`, `tla_forall_always_equality`), each constituting an unverified intensional bridge, plus two demonstrating their soundness consequences by deriving false from distinct-but-equivalent values.

## True Positives (Spec Issues)

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Unverified trust assumption.

### a_to_temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `a_to_temp_pred_equality` is external_body forcing intensional equality of `spec_fn(A) -> TempPred<T>` from pointwise mutual entailment. Independent unverified trust assumption.

### tla_forall_always_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `tla_forall_always_equality` is external_body forcing unconditional intensional equality between `tla_forall(|a| always(p(a)))` and `always(tla_forall(p))`. Independent unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — derives `false` from intensionally distinct but semantically equivalent preds.

### function_equality_from_pointwise_entailment
- **Confidence:** high
- **Reasoning:** Direct soundness consequence of `a_to_temp_pred_equality` — derives `false` from intensionally distinct but pointwise-equivalent functions.

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
- **Reasoning:** `a_to_temp_pred_equality` is external_body forcing intensional equality of `spec_fn(A) -> TempPred<T>` from pointwise mutual entailment. Independent unverified trust assumption.

### φ3: tla_forall_always_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_always_equality is external_body forcing unconditional intensional equality between tla_forall(|a| always(p(a))) and always(tla_forall(p)) — structurally distinct nested closures collapsed without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_forall_always_equality` is external_body forcing unconditional intensional equality between `tla_forall(|a| always(p(a)))` and `always(tla_forall(p))`. Independent unverified trust assumption.

### φ4: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p == q from mutual entailment — if two preds are intensionally distinct but semantically equivalent this derives false
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — derives `false` from intensionally distinct but semantically equivalent preds.

### φ5: function_equality_from_pointwise_entailment
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** a_to_temp_pred_equality forces f == g from pointwise mutual entailment — derives false if two functions produce semantically equivalent but intensionally distinct TempPreds
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct soundness consequence of `a_to_temp_pred_equality` — derives `false` from intensionally distinct but pointwise-equivalent functions.

