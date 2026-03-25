# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/simplify_predicate.rs`
**Date:** 2026-03-24T04:21:17Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Three true positives from independent external_body axioms: `entails_apply` (closure modus ponens), `temp_pred_equality` (intensional equality from semantic equivalence), and `entails_and_temp` (conjunction introduction through closure gap). A fourth true positive demonstrates the soundness consequence of `temp_pred_equality` deriving false from distinct-but-equivalent preds. One false positive: the absorption equality `p == p.and(q)` is a correct logical property with no new trust surface.

## True Positives (Spec Issues)

### entails_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `entails_apply` is external_body bridging `valid(p.implies(q))` — a forall over `TempPred::new` closure evaluations — to modus ponens at a specific execution. Unverified trust assumption resolving the intensional closure gap.

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — separate unverified trust assumption.

### entails_and_temp_closure_bridge
- **Confidence:** high
- **Reasoning:** `entails_and_temp` is external_body combining two entailments into `spec.entails(p.and(q))`, requiring that `p.and(q)`'s `TempPred::new` conjunction closure evaluates consistently with the individual closure results. Independent unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` forces `p == q` from mutual entailment, so if two TempPreds are intensionally distinct (`p !== q`) but semantically equivalent, this derives `false`. Core soundness consequence of the φ2 axiom.

## All Candidates

### φ1: entails_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** entails_apply is external_body bridging valid(p.implies(q)) — a forall over TempPred::new closure evaluations — to modus ponens at a specific execution without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `entails_apply` is external_body bridging `valid(p.implies(q))` — a forall over `TempPred::new` closure evaluations — to modus ponens at a specific execution. Unverified trust assumption resolving the intensional closure gap.

### φ2: temp_pred_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body forcing intensional equality from mutual entailment — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — separate unverified trust assumption.

### φ3: entails_and_temp_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** entails_and_temp is external_body combining two entailments into an entailment of p.and(q) — requires resolving that p.and(q)'s TempPred::new closure evaluates to the conjunction of the individual closure results
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `entails_and_temp` is external_body combining two entailments into `spec.entails(p.and(q))`, requiring that `p.and(q)`'s `TempPred::new` conjunction closure evaluates consistently with the individual closure results. Independent unverified trust assumption.

### φ4: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p == q from mutual entailment — if two preds are intensionally distinct but semantically equivalent this derives false
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` forces `p == q` from mutual entailment, so if two TempPreds are intensionally distinct (`p !== q`) but semantically equivalent, this derives `false`. Core soundness consequence of the φ2 axiom.

### φ5: simplify_predicate_equality
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** simplify_predicate forces intensional equality between p and p.and(q) — structurally distinct closures (p.pred vs conjunction closure) collapsed to equal via the chain of all three external_body axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `p == p.and(q)` given `p.entails(q)` is a semantically correct and desirable absorption property. This is an expected consequence of the three axioms, not a new spec gap.

