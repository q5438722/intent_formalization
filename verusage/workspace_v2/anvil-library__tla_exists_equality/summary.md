# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_exists_equality.rs`
**Date:** 2026-03-24T04:26:03Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 2
- False positives: 1

## Summary

One true positive from the external_body axiom `temp_pred_equality` forcing intensional equality from semantic equivalence, plus a second demonstrating its soundness consequence of deriving false from distinct-but-equivalent preds. One false positive: the exists-lift commutation is a correct logical equivalence with no independent trust surface.

## True Positives (Spec Issues)

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — if two TempPreds are intensionally distinct but semantically equivalent, this derives `false`.

## All Candidates

### φ1: temp_pred_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body forcing intensional equality from mutual entailment — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — unverified trust assumption.

### φ2: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p == q from mutual entailment — if two preds are intensionally distinct but semantically equivalent this derives false
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — if two TempPreds are intensionally distinct but semantically equivalent, this derives `false`.

### φ3: tla_exists_equality_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_exists_equality forces intensional equality between lift_state(∃a.f(a,t)) and tla_exists(a => lift_state(f(a,·))) — structurally distinct nested closures collapsed via temp_pred_equality
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `lift_state(∃a.f(a,t)) == tla_exists(a => lift_state(f(a,·)))` is a semantically correct commutation of existential quantification with state lifting. This is a verified lemma with no new trust surface beyond φ1.

### φ4: lift_state_closure_identity
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** temp_pred_equality collapses lift_state(p) == lift_state(q) from pointwise equivalence — forces intensional equality of spec_fn(T)->bool through the Execution wrapper without proof

### φ5: tla_exists_vacuous
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** even when f is universally false, tla_exists_equality forces structural equality between two TempPreds built from distinct empty-existential closures — the equality holds vacuously but the intensional collapse is still unverified

