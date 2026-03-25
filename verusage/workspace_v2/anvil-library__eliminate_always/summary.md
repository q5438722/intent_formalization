# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/eliminate_always.rs`
**Date:** 2026-03-24T03:53:06Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `execution_equality` forces intensional spec_fn equality from extensional equivalence (root axiom), and `implies_apply` bridges a separate intensional closure gap for TempPred modus ponens. The remaining three are false positives — suffix(0) identity, always-elimination as the T axiom, and suffix composition are all semantically correct consequences that merely instantiate the identified axioms on desirable properties.

## True Positives (Spec Issues)

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is an independent external_body axiom bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is a separate unverified trust assumption resolving the intensional closure gap for TempPred implication.

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body forcing intensional equality of spec_fn from pointwise extensional equivalence — two structurally distinct closures are collapsed to equal
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

### φ2: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping and direct implication — silently resolves closure identity mismatch
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is an independent external_body axiom bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is a separate unverified trust assumption resolving the intensional closure gap for TempPred implication.

### φ3: suffix_zero_equals_self
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** ex.suffix(0) creates closure |i| f(i+0) intensionally distinct from ex — execution_equality silently equates these without proof
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ex.suffix(0) == ex` is semantically correct and desirable. It's a concrete instantiation of φ1's axiom on a trivially correct property — no new trust surface.

### φ4: eliminate_always_reflexive
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** eliminate_always derives spec.entails(p) from spec.entails(always(p)) for any spec — combined with the trivial always(p).entails(always(p)), it strips temporal modality with only external_body axioms as justification
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `always(p).entails(p)` is the standard T axiom of temporal logic — a fundamental and desirable property. The precondition `always(p).entails(always(p))` is trivially true, making this just a restatement of `eliminate_always` on a concrete input.

### φ5: suffix_composition_equals_addition
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** doubly-nested suffix creates a closure intensionally distinct from single-offset suffix — execution_equality collapses them without proof
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Suffix composition is a semantically correct and desirable property. The intensional concern is entirely subsumed by φ1's identification of `execution_equality` as the root axiom.

