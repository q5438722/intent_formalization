# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_forall_implies_equality2.rs`
**Date:** 2026-03-24T04:34:49Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Three true positives from independent external_body axioms (`temp_pred_equality`, `a_to_temp_pred_equality`, `tla_forall_or_equality`), each constituting an unverified intensional bridge, plus a fourth demonstrating the soundness consequence of deriving false from distinct-but-equivalent values. One false positive: the forall-implies distribution is a correct logical equivalence with no independent trust surface.

## True Positives (Spec Issues)

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Unverified trust assumption.

### a_to_temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `a_to_temp_pred_equality` is external_body forcing intensional equality of `spec_fn(A) -> TempPred<T>` from pointwise mutual entailment. Independent unverified trust assumption.

### tla_forall_or_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `tla_forall_or_equality` is external_body forcing unconditional intensional equality between `tla_forall(|a| p(a).or(q))` and `tla_forall(p).or(q)`. Independent unverified trust assumption.

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
- **Why flagged:** a_to_temp_pred_equality is external_body forcing intensional equality of spec_fn(A)->TempPred from pointwise mutual entailment — collapses structurally distinct function families without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `a_to_temp_pred_equality` is external_body forcing intensional equality of `spec_fn(A) -> TempPred<T>` from pointwise mutual entailment. Independent unverified trust assumption.

### φ3: tla_forall_or_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_or_equality is external_body forcing unconditional intensional equality between tla_forall(|a| p(a).or(q)) and tla_forall(p).or(q) — structurally distinct nested closures collapsed without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_forall_or_equality` is external_body forcing unconditional intensional equality between `tla_forall(|a| p(a).or(q))` and `tla_forall(p).or(q)`. Independent unverified trust assumption.

### φ4: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p == q from mutual entailment — if two preds are intensionally distinct but semantically equivalent this derives false
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — derives `false` from intensionally distinct but semantically equivalent preds.

### φ5: tla_forall_implies_equality2_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chains all three external_body axioms to force intensional equality between tla_forall(|a| p=>q(a)) and p=>tla_forall(q) — propagates intensional collapse through implies/or/not/forall nesting
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `∀a.(p⇒q(a)) ≡ p⇒∀a.q(a)` is a standard logical equivalence. This is a verified lemma with no new trust surface beyond φ1–φ3.

