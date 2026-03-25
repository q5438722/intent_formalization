# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/temp_pred_equality.rs`
**Date:** 2026-03-24T04:24:45Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Two true positives from independent external_body axioms: `implies_apply` (closure modus ponens) and `implies_contraposition_apply` (closure contraposition). A third true positive demonstrates the soundness consequence of these axioms deriving false from distinct-but-equivalent preds. One false positive: double negation elimination is a correct logical property.

## True Positives (Spec Issues)

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a `TempPred::new` closure — to modus ponens. Unverified trust assumption resolving the intensional closure gap.

### implies_contraposition_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_contraposition_apply` is external_body bridging `p.implies(q)` and `not(q)` — each wrapped in distinct `TempPred::new` closures — to `not(p)`. Independent unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` (verified via φ1+φ2) forces `p == q` from mutual entailment. If two preds are intensionally distinct but semantically equivalent, this derives `false`. Core soundness consequence of the two closure bridge axioms.

## All Candidates

### φ1: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping to modus ponens — resolves closure identity mismatch without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a `TempPred::new` closure — to modus ponens. Unverified trust assumption resolving the intensional closure gap.

### φ2: implies_contraposition_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_contraposition_apply is external_body bridging p.implies(q) and not(q) — each wrapped in distinct TempPred::new closures — to not(p) without verifying closure identity
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_contraposition_apply` is external_body bridging `p.implies(q)` and `not(q)` — each wrapped in distinct `TempPred::new` closures — to `not(p)`. Independent unverified trust assumption.

### φ3: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p == q from mutual entailment — if two preds are intensionally distinct but semantically equivalent this derives false
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` (verified via φ1+φ2) forces `p == q` from mutual entailment. If two preds are intensionally distinct but semantically equivalent, this derives `false`. Core soundness consequence of the two closure bridge axioms.

### φ4: extensional_equality_of_spec_fn
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** the two external_body axioms together allow deriving intensional equality of arbitrary spec_fn(Execution) -> bool from pointwise equivalence — collapsing all extensionally equal functions to intensionally equal

### φ5: not_not_equals_self
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** not(not(p)) wraps p in two layers of TempPred::new closures (!(!x)) — forced equal to p via the closure bridge axioms despite structural distinctness
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `not(not(p)) == p` is a semantically correct double negation elimination. This is an expected and desirable property, not a spec gap.

