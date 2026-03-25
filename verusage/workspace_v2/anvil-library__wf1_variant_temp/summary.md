# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/wf1_variant_temp.rs`
**Date:** 2026-03-24T04:43:18Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 5
- False positives: 0

## Summary

Five true positives from five independent external_body axioms: `leads_to_unfold` (leads-to elimination through nested closures), `implies_apply` (modus ponens through closure gap), `always_propagate_forwards` (always suffix propagation through closure gap), `eventually_proved_by_witness` (existential introduction through closure gap), and `execution_equality` (pointwise state equality to intensional Execution equality). Each constitutes an unverified intensional bridge. Notably, `always_p_or_eventually_q` — a sixth external_body axiom performing temporal induction — was not submitted as a candidate but carries an independent trust surface.

## True Positives (Spec Issues)

### leads_to_unfold_closure_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s `always(implies(eventually))` nested closure wrapping to a bare forall. Unverified trust assumption.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — a `TempPred::new` closure-wrapped implication — to modus ponens. Unverified trust assumption.

### always_propagate_forwards_closure_bridge
- **Confidence:** high
- **Reasoning:** `always_propagate_forwards` is external_body bridging `always(p).satisfied_by(ex)` to `always(p).satisfied_by(ex.suffix(i))`, resolving the closure identity between `suffix(i).suffix(j)` and `suffix(i+j)` at the `always` level. Unverified trust assumption.

### eventually_proved_by_witness_closure_bridge
- **Confidence:** high
- **Reasoning:** `eventually_proved_by_witness` is external_body bridging a concrete witness to `eventually`'s `TempPred::new` closure-wrapped existential. Unverified trust assumption.

### execution_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `execution_equality` is external_body forcing intensional `==` of `Execution` from pointwise state equality. Unverified trust assumption collapsing `spec_fn(nat)->T` without proof.

## All Candidates

### φ1: leads_to_unfold_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_unfold is external_body bridging leads_to's always(implies(eventually)) nested closure wrapping to a bare forall — resolves multiple layers of closure identity without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s `always(implies(eventually))` nested closure wrapping to a bare forall. Unverified trust assumption.

### φ2: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q)'s TempPred::new closure-wrapped implication to modus ponens — resolves closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — a `TempPred::new` closure-wrapped implication — to modus ponens. Unverified trust assumption.

### φ3: always_propagate_forwards_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_propagate_forwards is external_body bridging always's closure-wrapped forall to suffix propagation — resolves closure identity between always(p).satisfied_by(ex) and always(p).satisfied_by(ex.suffix(i)) without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_propagate_forwards` is external_body bridging `always(p).satisfied_by(ex)` to `always(p).satisfied_by(ex.suffix(i))`, resolving the closure identity between `suffix(i).suffix(j)` and `suffix(i+j)` at the `always` level. Unverified trust assumption.

### φ4: eventually_proved_by_witness_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** eventually_proved_by_witness is external_body bridging a concrete witness to eventually's TempPred::new closure-wrapped existential — resolves closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `eventually_proved_by_witness` is external_body bridging a concrete witness to `eventually`'s `TempPred::new` closure-wrapped existential. Unverified trust assumption.

### φ5: execution_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body forcing intensional equality of Execution from pointwise state equality — collapses structurally distinct spec_fn(nat)->T without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `execution_equality` is external_body forcing intensional `==` of `Execution` from pointwise state equality. Unverified trust assumption collapsing `spec_fn(nat)->T` without proof.

