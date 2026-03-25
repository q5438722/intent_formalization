# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_shortcut_temp.rs`
**Date:** 2026-03-24T04:12:28Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Four true positives: `temp_pred_equality` forces intensional equality from semantic equivalence (with φ5 demonstrating the direct soundness consequence of deriving false from distinct-but-equivalent preds), `leads_to_trans` and `leads_to_framed_by_or` are independent external_body axioms bridging intensional closure gaps in transitive composition and disjunction framing respectively. One false positive: or-idempotence is a correct semantic property.

## True Positives (Spec Issues)

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. This collapses structurally distinct closures to equal without proof — an unverified trust assumption.

### leads_to_trans_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_trans` is external_body composing two leads_to properties through nested always/implies/eventually closures. This is a separate unverified trust assumption bridging intensional closure gaps.

### leads_to_framed_by_or_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_framed_by_or` is external_body lifting leads_to through disjunction framing, constructing fresh `or` closures inside the leads_to structure. This is a separate unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` forces `p == q` from mutual entailment, so if two TempPreds are intensionally distinct (`p !== q`) but semantically equivalent, this derives `false`. This is the core soundness concern — structurally different but logically equivalent closures yield contradiction.

## All Candidates

### φ1: temp_pred_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body forcing intensional equality of two TempPreds from mutual entailment — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. This collapses structurally distinct closures to equal without proof — an unverified trust assumption.

### φ2: leads_to_trans_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_trans is external_body composing two leads_to through nested always/implies/eventually closures — bridges intensional gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_trans` is external_body composing two leads_to properties through nested always/implies/eventually closures. This is a separate unverified trust assumption bridging intensional closure gaps.

### φ3: leads_to_framed_by_or_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_framed_by_or is external_body framing leads_to with disjunction — the or-lifted leads_to involves fresh closure constructions creating intensional gaps resolved without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_framed_by_or` is external_body lifting leads_to through disjunction framing, constructing fresh `or` closures inside the leads_to structure. This is a separate unverified trust assumption.

### φ4: or_idempotent_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p.or(q).or(q) == p.or(q) as intensional equality — two structurally distinct nested or-closures collapsed to equal via mutual entailment without verifying closure identity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `p.or(q).or(q) == p.or(q)` is a semantically correct and desirable property — or-idempotence is standard propositional logic. This is an expected consequence of `temp_pred_equality`, not a spec gap itself.

### φ5: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces intensionally distinct but mutually entailing TempPreds to be equal — if two preds are structurally different but semantically equivalent, this derives false from the contradiction
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` forces `p == q` from mutual entailment, so if two TempPreds are intensionally distinct (`p !== q`) but semantically equivalent, this derives `false`. This is the core soundness concern — structurally different but logically equivalent closures yield contradiction.

