# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_forall_not_equality.rs`
**Date:** 2026-03-24T04:35:24Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Two true positives from independent external_body axioms: `tla_forall_unfold` (forall extraction through closure gap) and `temp_pred_equality` (intensional equality from semantic equivalence), plus a third demonstrating the soundness consequence of deriving false from distinct-but-equivalent preds. Two false positives: the De Morgan quantifier equivalence and its directional application are correct logical consequences with no independent trust surface.

## True Positives (Spec Issues)

### tla_forall_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `tla_forall_unfold` is external_body bridging `tla_forall(a_to_p).satisfied_by(ex)` — a `TempPred::new` closure-wrapped forall — to a bare forall. Unverified trust assumption.

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Independent unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — derives `false` from intensionally distinct but semantically equivalent preds.

## All Candidates

### φ1: tla_forall_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_unfold is external_body bridging tla_forall's TempPred::new closure-wrapped forall to a bare forall — resolves closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_forall_unfold` is external_body bridging `tla_forall(a_to_p).satisfied_by(ex)` — a `TempPred::new` closure-wrapped forall — to a bare forall. Unverified trust assumption.

### φ2: temp_pred_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body forcing intensional equality from mutual entailment — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Independent unverified trust assumption.

### φ3: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p == q from mutual entailment — if two preds are intensionally distinct but semantically equivalent this derives false
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — derives `false` from intensionally distinct but semantically equivalent preds.

### φ4: tla_forall_not_equality_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chains both external_body axioms to force intensional equality between tla_forall(|a| not(p(a))) and not(tla_exists(p)) — structurally distinct nested closures collapsed
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `∀a.¬p(a) ≡ ¬∃a.p(a)` is a standard logical equivalence (De Morgan for quantifiers). This is a verified lemma with no new trust surface beyond φ1 and φ2.

### φ5: tla_exists_negation_from_forall_not
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** uses the forced intensional equality to derive semantic consequence — if the closure bridge is unsound, ∀a.¬p(a) might not correctly negate ∃a.p(a) at the TempPred level
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct semantic consequence of φ4's equality — substituting one side for the other. Semantically correct with no new trust surface.

