# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/transform_leads_to_with_until.rs`
**Date:** 2026-03-24T04:37:58Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 4
- False positives: 0

## Summary

Four true positives from independent external_body axioms: `always_unfold` (always-forall extraction), `implies_apply` (modus ponens through closure gap), `eventually_proved_by_witness` (existential introduction through closure gap), and `always_p_or_eventually_q` (temporal induction through multiple nested closure layers). Each constitutes an unverified intensional bridge.

## True Positives (Spec Issues)

### always_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_unfold` is external_body bridging `always(p).satisfied_by(ex)` — a `TempPred::new` closure-wrapped forall — to a bare forall. Unverified trust assumption.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — a `TempPred::new` closure-wrapped implication — to modus ponens. Unverified trust assumption.

### eventually_proved_by_witness_closure_bridge
- **Confidence:** high
- **Reasoning:** `eventually_proved_by_witness` is external_body bridging a concrete witness to `eventually`'s `TempPred::new` closure-wrapped existential. Unverified trust assumption.

### always_p_or_eventually_q_induction_bridge
- **Confidence:** high
- **Reasoning:** `always_p_or_eventually_q` is external_body performing temporal induction — derives `always(p ⇒ □p ∨ ◇q)` from a step condition, bridging multiple closure layers (always/implies/or/later/eventually) without proof. Independent unverified trust assumption beyond the simpler unfold axioms.

## All Candidates

### φ1: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging always's TempPred::new closure-wrapped forall to a bare forall — resolves closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_unfold` is external_body bridging `always(p).satisfied_by(ex)` — a `TempPred::new` closure-wrapped forall — to a bare forall. Unverified trust assumption.

### φ2: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q)'s TempPred::new closure-wrapped implication to modus ponens — resolves closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — a `TempPred::new` closure-wrapped implication — to modus ponens. Unverified trust assumption.

### φ3: eventually_proved_by_witness_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** eventually_proved_by_witness is external_body bridging a concrete witness to eventually's TempPred::new closure-wrapped existential — resolves closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `eventually_proved_by_witness` is external_body bridging a concrete witness to `eventually`'s `TempPred::new` closure-wrapped existential. Unverified trust assumption.

### φ4: always_p_or_eventually_q_induction_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_p_or_eventually_q is external_body performing temporal induction — derives always(p => □p ∨ ◇q) from a step condition, bridging multiple layers of closure nesting (always/implies/or/later/eventually) without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_p_or_eventually_q` is external_body performing temporal induction — derives `always(p ⇒ □p ∨ ◇q)` from a step condition, bridging multiple closure layers (always/implies/or/later/eventually) without proof. Independent unverified trust assumption beyond the simpler unfold axioms.

### φ5: transform_leads_to_with_until_bridge
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** chains all four external_body axioms to derive that p1∧p2 leads to (q1∧p2)∨q2 — if any intensional bridge is unsound, this enables combining leads-to with persistence/until reasoning unsoundly

