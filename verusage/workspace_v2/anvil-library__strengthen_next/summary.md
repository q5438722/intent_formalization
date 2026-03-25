# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/strengthen_next.rs`
**Date:** 2026-03-24T04:23:50Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Three true positives from independent external_body axioms: `temp_pred_equality` (intensional equality from semantic equivalence), `always_and_equality` (always-conjunction distribution), and `entails_and_temp` (conjunction introduction through closure gap). A fourth true positive demonstrates the soundness consequence of `temp_pred_equality` deriving false from distinct-but-equivalent preds. One false positive: `strengthen_next` is a correct temporal logic pattern with no independent trust surface.

## True Positives (Spec Issues)

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — unverified trust assumption.

### always_and_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_and_equality` is external_body forcing `always(p.and(q)) == always(p).and(always(q))` unconditionally. These are structurally distinct nested `TempPred::new` closures — independent unverified trust assumption.

### entails_and_temp_closure_bridge
- **Confidence:** high
- **Reasoning:** `entails_and_temp` is external_body combining pointwise entailments into conjunction entailment, requiring that `p.and(q)`'s `TempPred::new` closure evaluates consistently with individual results. Independent unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — if two TempPreds are intensionally distinct but semantically equivalent, this derives `false` from the contradiction with forced equality.

## All Candidates

### φ1: temp_pred_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body forcing intensional equality from mutual entailment — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — unverified trust assumption.

### φ2: always_and_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_and_equality is external_body forcing intensional equality between always(p.and(q)) and always(p).and(always(q)) — structurally distinct nested TempPred::new closures collapsed without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_and_equality` is external_body forcing `always(p.and(q)) == always(p).and(always(q))` unconditionally. These are structurally distinct nested `TempPred::new` closures — independent unverified trust assumption.

### φ3: entails_and_temp_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** entails_and_temp is external_body combining two entailments into conjunction entailment — requires resolving that p.and(q)'s TempPred::new closure evaluates to the conjunction of individual closure results
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `entails_and_temp` is external_body combining pointwise entailments into conjunction entailment, requiring that `p.and(q)`'s `TempPred::new` closure evaluates consistently with individual results. Independent unverified trust assumption.

### φ4: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p == q from mutual entailment — if two preds are intensionally distinct but semantically equivalent this derives false
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — if two TempPreds are intensionally distinct but semantically equivalent, this derives `false` from the contradiction with forced equality.

### φ5: strengthen_next_equality_chain
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** strengthen_next chains all three external_body axioms to substitute always(lift_action(next_and_inv)) for always(lift_action(next).and(lift_state(inv))) — the intensional equality forced by temp_pred_equality propagates through always_and_equality's closure gap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Strengthening next with an invariant is a correct and standard TLA reasoning pattern. This is a verified lemma whose trust dependencies are φ1–φ3 — no new trust surface.

