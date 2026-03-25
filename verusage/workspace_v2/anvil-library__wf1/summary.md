# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/wf1.rs`
**Date:** 2026-03-24T04:42:44Z
**Status:** COMPLETE

## Stats

- Candidates generated: 3
- Entailed (verified): 3
- True positives: 3
- False positives: 0

## Summary

Three true positives from independent external_body axioms: `implies_apply` (modus ponens through closure gap), `implies_apply_with_always` (always-distributed modus ponens through nested closure layers), and `execution_equality` (pointwise state equality to intensional Execution equality). Each constitutes an unverified intensional bridge. Note: `weak_fairness_unfold` and `wf1_variant_temp` were not submitted as candidates but are additional external_body axioms in this file with independent trust surfaces.

## True Positives (Spec Issues)

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — a `TempPred::new` closure-wrapped implication — to modus ponens. Unverified trust assumption.

### implies_apply_with_always_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply_with_always` is external_body bridging `always(p.implies(q))` ∧ `always(p)` to `always(q)`, resolving nested always/implies closure layers. Unverified trust assumption.

### execution_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `execution_equality` is external_body forcing intensional `==` of `Execution` from pointwise state equality. Unverified trust assumption collapsing `spec_fn(nat)->T` without proof.

## All Candidates

### φ1: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q)'s TempPred::new closure-wrapped implication to modus ponens — resolves closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — a `TempPred::new` closure-wrapped implication — to modus ponens. Unverified trust assumption.

### φ2: implies_apply_with_always_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply_with_always is external_body bridging always(p=>q) ∧ always(p) to always(q) — resolves nested always/implies closure layers without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply_with_always` is external_body bridging `always(p.implies(q))` ∧ `always(p)` to `always(q)`, resolving nested always/implies closure layers. Unverified trust assumption.

### φ3: execution_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body forcing intensional equality of Execution from pointwise state equality — collapses structurally distinct spec_fn(nat)->T without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `execution_equality` is external_body forcing intensional `==` of `Execution` from pointwise state equality. Unverified trust assumption collapsing `spec_fn(nat)->T` without proof.

