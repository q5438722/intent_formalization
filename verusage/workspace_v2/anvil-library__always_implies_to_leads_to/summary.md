# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_implies_to_leads_to.rs`
**Date:** 2026-03-24T03:41:40Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives identified. φ1 is the most critical: `execution_equality` forces intensional spec_fn equality from extensional equivalence, an unverified external_body assumption. φ3 and φ4 identify two additional independent external_body axioms (`always_unfold` and `implies_apply`) that each bridge their own intensional closure gaps. φ2 and φ5 are false positives — p leads_to p is a standard temporal logic theorem, and false-spec vacuous entailment is basic logic.

## True Positives (Spec Issues)

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** The external_body axiom forces intensional `spec_fn` equality from pointwise agreement. This is the core soundness assumption — unverified, and it collapses the extensional/intensional distinction that Verus normally maintains for closures.

### always_unfold_intensional_bridge
- **Confidence:** medium
- **Reasoning:** The external_body `always_unfold` bridges the intensional gap between the `forall` quantifier embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures. This is an unverified trust assumption, though semantically correct.

### implies_apply_closure_bridge
- **Confidence:** medium
- **Reasoning:** The external_body `implies_apply` bridges the gap between `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — and the direct modus ponens conclusion. This is a separate unverified intensional bridge axiom.

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body forcing intensional equality of spec_fn from pointwise extensional equivalence — two structurally distinct closures are collapsed to equal
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body axiom forces intensional `spec_fn` equality from pointwise agreement. This is the core soundness assumption — unverified, and it collapses the extensional/intensional distinction that Verus normally maintains for closures.

### φ2: leads_to_from_trivial_implies
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** any p trivially leads_to itself via always(p→p) which is always true — leads_to(p,p) is vacuously derivable for every p, potentially masking real liveness obligations
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** p leads_to p is a standard theorem in TLA-style temporal logic — every state property trivially leads to itself (it already holds, so "eventually" is satisfied at offset 0). This is expected and desirable, not a spec gap.

### φ3: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging the forall embedded inside always's TempPred::new closure and a bare forall in the ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The external_body `always_unfold` bridges the intensional gap between the `forall` quantifier embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures. This is an unverified trust assumption, though semantically correct.

### φ4: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping and direct implication — silently resolves closure identity mismatch
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The external_body `implies_apply` bridges the gap between `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — and the direct modus ponens conclusion. This is a separate unverified intensional bridge axiom.

### φ5: leads_to_vacuous_under_false_spec
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously entails every leads_to property — any liveness claim under a contradictory spec is trivially true, which could hide specification errors
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A false antecedent entailing anything is standard logic (ex falso quodlibet). This is not a spec gap — it's a fundamental property of implication that applies to every entailment-based framework.

