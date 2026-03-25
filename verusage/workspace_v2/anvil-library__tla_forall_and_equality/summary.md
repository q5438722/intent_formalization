# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_forall_and_equality.rs`
**Date:** 2026-03-24T04:33:30Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Two true positives from independent external_body axioms: `tla_forall_unfold` (forall extraction through closure gap) and `temp_pred_equality` (intensional equality from semantic equivalence), plus a third demonstrating the soundness consequence of deriving false from distinct-but-equivalent preds. Two false positives: the forall-and distribution is a correct logical equivalence, and extracting q from the conjunction is sound since Verus types are inhabited in spec mode.

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

### φ4: tla_forall_and_equality_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chains both external_body axioms to force intensional equality between tla_forall(|a| p(a).and(q)) and tla_forall(p).and(q) — structurally distinct nested closures collapsed
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `(∀a. p(a) ∧ q) ≡ (∀a. p(a)) ∧ q` is a standard logical equivalence (q doesn't depend on a). This is a verified lemma with no new trust surface beyond φ1 and φ2.

### φ5: tla_forall_and_arbitrary_witness
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** the proof of tla_forall_and_equality uses arbitrary() to extract q from the conjunction — if the type A is uninhabited, tla_forall is vacuously true but q might not hold, yet the equality still forces it
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Extracting q from `tla_forall(|a| p(a).and(q))` is semantically correct — the forall includes q in every conjunct. In Verus spec mode every type is inhabited (via `arbitrary()`), so the vacuous-forall concern for uninhabited A doesn't apply.

