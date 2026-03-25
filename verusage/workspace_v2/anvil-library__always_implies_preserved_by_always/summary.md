# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_implies_preserved_by_always.rs`
**Date:** 2026-03-24T03:40:56Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives identified. φ1 is the most fundamental: `execution_equality` forces intensional spec_fn equality from extensional equivalence, the core unverified trust assumption. φ4 and φ5 identify two additional independent external_body axioms (`implies_apply` and `always_unfold`) that each bridge their own intensional closure gaps. φ2 and φ3 are false positives — semantically correct consequences (suffix(0) identity and suffix composition) that merely instantiate φ1's axiom on desirable properties.

## True Positives (Spec Issues)

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** The external_body axiom `execution_equality` wraps struct equality around pointwise function agreement, and since struct equality implies field equality, this derives `f == g` (intensional spec_fn equality) from mere extensional equivalence. This is the root soundness assumption — unverified and potentially unsound if Verus's SMT encoding ever distinguishes extensionally equal closures.

### implies_apply_closure_bridge
- **Confidence:** medium
- **Reasoning:** The external_body `implies_apply` bridges the gap between `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure created by `TempPred::new` — and the direct modus ponens conclusion. This is a separate unverified trust assumption from execution_equality, operating on TempPred closures rather than Execution closures.

### always_unfold_intensional_bridge
- **Confidence:** medium
- **Reasoning:** The external_body `always_unfold` extracts the `forall` quantifier from inside `always`'s `TempPred::new` closure into a bare `forall` in the ensures. This is a distinct unverified axiom bridging the intensional gap for TempPred evaluation, separate from both execution_equality and implies_apply.

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body and forces intensional equality of spec_fn from pointwise extensional equivalence — two distinct closures agreeing on all inputs are collapsed to structurally equal
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body axiom `execution_equality` wraps struct equality around pointwise function agreement, and since struct equality implies field equality, this derives `f == g` (intensional spec_fn equality) from mere extensional equivalence. This is the root soundness assumption — unverified and potentially unsound if Verus's SMT encoding ever distinguishes extensionally equal closures.

### φ2: suffix_zero_equals_self
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** ex.suffix(0) creates a new closure |i| f(i+0) intensionally distinct from ex — execution_equality silently equates these without proof, papering over the intensional/extensional gap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a semantically correct and desirable property — `ex.suffix(0)` should equal `ex`. It's a direct concrete application of φ1's axiom; it identifies no new trust surface beyond what execution_equality already provides.

### φ3: suffix_composition_equals_addition
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** doubly-nested suffix creates a closure intensionally distinct from single-offset suffix — execution_equality collapses them without proof, and always_implies_preserved_by_always depends on this at the key composition step
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the standard suffix composition law, semantically correct and essential for temporal logic reasoning. It's the specific instance that `always_implies_preserved_by_always` depends on, but it's a desirable property — the concern is entirely subsumed by φ1's identification of the underlying execution_equality axiom.

### φ4: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging the intensional gap between p.implies(q).satisfied_by(ex) (which wraps in TempPred::new) and direct implication — silently resolves closure identity mismatch
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The external_body `implies_apply` bridges the gap between `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure created by `TempPred::new` — and the direct modus ponens conclusion. This is a separate unverified trust assumption from execution_equality, operating on TempPred closures rather than Execution closures.

### φ5: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging the forall embedded in always's TempPred::new closure and a bare forall in the ensures — these are intensionally distinct formulations that the axiom equates without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The external_body `always_unfold` extracts the `forall` quantifier from inside `always`'s `TempPred::new` closure into a bare `forall` in the ensures. This is a distinct unverified axiom bridging the intensional gap for TempPred evaluation, separate from both execution_equality and implies_apply.

