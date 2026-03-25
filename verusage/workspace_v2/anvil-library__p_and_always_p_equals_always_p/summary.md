# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/p_and_always_p_equals_always_p.rs`
**Date:** 2026-03-24T04:17:00Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Two true positives from the external_body axioms: `always_to_current` bridging the intensional closure gap for always-forall extraction, and `temp_pred_equality` forcing intensional equality from semantic equivalence (with φ3 demonstrating the direct soundness consequence of deriving false from distinct-but-equivalent preds). Two false positives: the and-always idempotence is a correct temporal property, and suffix instantiation adds no new trust surface.

## True Positives (Spec Issues)

### always_to_current_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_to_current` is external_body extracting `p.satisfied_by(ex)` from `always(p).satisfied_by(ex)` — requires resolving that `always`'s `TempPred::new` closure-wrapped forall, instantiated at suffix(0), yields `p.satisfied_by(ex)` through the suffix(0) identity. Unverified trust assumption.

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — a separate unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` forces `p == q` from mutual entailment, so if two TempPreds are intensionally distinct (`p !== q`) but semantically equivalent, this derives `false`. Core soundness consequence of the axiom.

## All Candidates

### φ1: always_to_current_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_to_current is external_body extracting p at the current state from always(p)'s TempPred::new closure-wrapped forall (instantiated at suffix(0)) — intensional closure gap resolved without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_to_current` is external_body extracting `p.satisfied_by(ex)` from `always(p).satisfied_by(ex)` — requires resolving that `always`'s `TempPred::new` closure-wrapped forall, instantiated at suffix(0), yields `p.satisfied_by(ex)` through the suffix(0) identity. Unverified trust assumption.

### φ2: temp_pred_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body forcing intensional equality from mutual entailment — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — a separate unverified trust assumption.

### φ3: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p == q from mutual entailment — if two preds are intensionally distinct but semantically equivalent, this derives false from the contradiction
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` forces `p == q` from mutual entailment, so if two TempPreds are intensionally distinct (`p !== q`) but semantically equivalent, this derives `false`. Core soundness consequence of the axiom.

### φ4: and_always_idempotent_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** p.and(always(p)) and always(p) are structurally distinct TempPred closures forced equal via temp_pred_equality — the intensional equality claim depends on both external_body axioms being sound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `p.and(always(p)) == always(p)` is a semantically correct and desirable temporal logic property — always(p) implies both p and always(p). This is an expected consequence of the two axioms, not a spec gap itself.

### φ5: always_to_current_at_suffix
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** always_to_current applied at an arbitrary suffix extracts p at ex.suffix(i) — if the closure bridge between always's forall-over-suffixes and suffix(0) identity is unsound, this extends the unsoundness to any index
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct instantiation of φ1 with `ex.suffix(i)` substituted for `ex`. No new trust surface — just universal instantiation of the same axiom.

