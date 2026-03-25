# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/entails_preserved_by_always.rs`
**Date:** 2026-03-24T03:56:21Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `always_unfold` and `implies_apply` are independent external_body axioms each bridging distinct intensional closure gaps — extracting a forall from always's closure wrapper, and modus ponens through TempPred::new wrapping, respectively. The remaining three are false positives: monotone lifting through specs, transitive chaining through always, and valid-implies-entailed are all standard logical properties that compose the identified axioms with no new trust surface.

## True Positives (Spec Issues)

### always_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_unfold` is external_body bridging the `forall` embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures. This is an unverified trust assumption.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is a separate unverified trust assumption resolving the intensional closure gap.

## All Candidates

### φ1: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging the forall inside always's TempPred::new closure and a bare forall in the ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_unfold` is external_body bridging the `forall` embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures. This is an unverified trust assumption.

### φ2: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping and direct implication — silently resolves closure identity mismatch
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is a separate unverified trust assumption resolving the intensional closure gap.

### φ3: entails_preserved_always_any_spec
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** composing entails_preserved_by_always with implies_apply allows lifting any pointwise entailment through always for arbitrary specs — amplifies the unverified trust surface across both external_body axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This composes `entails_preserved_by_always` (a verified lemma) with `implies_apply` to weaken through an arbitrary spec. The result is semantically correct monotonicity — no new trust surface beyond φ1 and φ2.

### φ4: always_entails_monotone_chain
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** chaining implies_apply on suffix executions composes transitivity through always — each application trusts implies_apply to resolve a fresh closure gap, and unbounded chaining amplifies the trust surface
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Transitivity of entailment lifted through always is a standard and expected property. It composes the already-identified axioms with no new trust surface.

### φ5: valid_implies_entails_anything
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** if valid(p) holds then any spec vacuously entails p — implies_apply bridges the closure gap so valid properties are entailed by everything, collapsing the distinction between validity and entailment
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** If `p` holds on all executions (`valid(p)`), then any spec trivially entails `p` — this is the definition of validity. The "collapse" is correct and expected.

