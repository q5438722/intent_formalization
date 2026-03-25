# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_forall_always_equality.rs`
**Date:** 2026-03-24T04:28:51Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Three true positives from independent external_body axioms: `always_unfold` (always-forall extraction), `tla_forall_unfold` (tla_forall-forall extraction), and `temp_pred_equality` (intensional equality from semantic equivalence), plus a fourth demonstrating the soundness consequence of deriving false from distinct-but-equivalent preds. One false positive: the forall-always commutation is a correct temporal logic equivalence with no independent trust surface.

## True Positives (Spec Issues)

### always_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_unfold` is external_body bridging `always(p).satisfied_by(ex)` — a `TempPred::new` closure-wrapped forall — to a bare forall. Unverified trust assumption.

### tla_forall_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `tla_forall_unfold` is external_body bridging `tla_forall(a_to_p).satisfied_by(ex)` — a `TempPred::new` closure-wrapped forall — to a bare forall. Independent unverified trust assumption.

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — separate unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — derives `false` from intensionally distinct but semantically equivalent preds.

## All Candidates

### φ1: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging always's TempPred::new closure-wrapped forall to a bare forall in ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_unfold` is external_body bridging `always(p).satisfied_by(ex)` — a `TempPred::new` closure-wrapped forall — to a bare forall. Unverified trust assumption.

### φ2: tla_forall_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_unfold is external_body bridging tla_forall's TempPred::new closure-wrapped forall to a bare forall — resolves closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_forall_unfold` is external_body bridging `tla_forall(a_to_p).satisfied_by(ex)` — a `TempPred::new` closure-wrapped forall — to a bare forall. Independent unverified trust assumption.

### φ3: temp_pred_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body forcing intensional equality from mutual entailment — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — separate unverified trust assumption.

### φ4: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p == q from mutual entailment — if two preds are intensionally distinct but semantically equivalent this derives false
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — derives `false` from intensionally distinct but semantically equivalent preds.

### φ5: tla_forall_always_equality_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_always_equality chains all three external_body axioms to force intensional equality between tla_forall(|a| always(p(a))) and always(tla_forall(p)) — structurally distinct nested closures collapsed
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `∀a.□p(a) ⟺ □∀a.p(a)` is a standard temporal logic equivalence. This is a verified lemma with no new trust surface beyond φ1–φ3.

