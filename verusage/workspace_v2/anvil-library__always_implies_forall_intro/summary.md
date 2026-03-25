# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_implies_forall_intro.rs`
**Date:** 2026-03-24T03:40:06Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 3
- False positives: 0

## Summary

All three candidates are true positives. φ5 is the most critical: it demonstrates that `tla_forall_always_implies_equality2` can derive `false` from a satisfiable precondition, since the two sides are built from structurally distinct closures that could be intensionally unequal. φ1 shows the same axiom forces `.pred` field equality between distinct closures. φ2 identifies a lower-severity but real trust gap in `implies_apply` bridging closure-wrapping intensional mismatches.

## True Positives (Spec Issues)

### tla_forall_always_implies_intensional_equality
- **Confidence:** high
- **Reasoning:** The external_body axiom forces `==` (intensional equality) between two TempPred values built from structurally distinct closures. This propagates to `.pred` field equality, conflating extensional equivalence with intensional identity — a real unverified trust assumption.

### implies_apply_closure_bridge
- **Confidence:** medium
- **Reasoning:** The external_body `implies_apply` bridges the intensional gap between `p.implies(q).satisfied_by(ex)` (which wraps the implication in a fresh `TempPred::new` closure) and the direct conclusion `q.satisfied_by(ex)`. While semantically correct, this is an unverified axiom that silently resolves closure-wrapping identity mismatches.

### equality_axiom_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** This is the sharpest finding. If the LHS and RHS of `tla_forall_always_implies_equality2` are ever intensionally distinct (entirely possible given different closure constructions), the axiom forces `==` and the precondition `!=` directly yields `false`. This is a concrete soundness hole inherent in any external_body axiom asserting `==` between structurally distinct spec objects.

## All Candidates

### φ1: tla_forall_always_implies_intensional_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_always_implies_equality2 is external_body forcing intensional equality of structurally distinct TempPred closures — the LHS nests forall inside always inside forall while the RHS nests forall inside implies inside always, and equating their pred fields conflates extensional with intensional identity
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body axiom forces `==` (intensional equality) between two TempPred values built from structurally distinct closures. This propagates to `.pred` field equality, conflating extensional equivalence with intensional identity — a real unverified trust assumption.

### φ2: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging the intensional gap between p.implies(q).satisfied_by(ex) (which goes through TempPred::new creating a fresh closure) and the direct implication — it silently resolves the closure-wrapping mismatch without proof
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The external_body `implies_apply` bridges the intensional gap between `p.implies(q).satisfied_by(ex)` (which wraps the implication in a fresh `TempPred::new` closure) and the direct conclusion `q.satisfied_by(ex)`. While semantically correct, this is an unverified axiom that silently resolves closure-wrapping identity mismatches.

### φ3: spec_entails_tla_forall_vacuous
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** spec_entails_tla_forall allows an unsatisfiable spec (always false) to entail tla_forall of anything vacuously — while logically valid, this means any property can be "proven" under a contradictory spec without warning

### φ4: always_implies_forall_intro_empty_type
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** tla_forall over always(p implies true) is trivially valid for any p — combined with tla_forall_always_implies_equality2, this could equate a trivially-valid predicate with always(p implies tla_forall(const_true)) collapsing meaningful distinctions

### φ5: equality_axiom_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** if any instance exists where the LHS and RHS of tla_forall_always_implies_equality2 are intensionally distinct (which is expected for different closure constructions), the external_body axiom forces equality and derives false — a direct soundness hole
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This is the sharpest finding. If the LHS and RHS of `tla_forall_always_implies_equality2` are ever intensionally distinct (entirely possible given different closure constructions), the axiom forces `==` and the precondition `!=` directly yields `false`. This is a concrete soundness hole inherent in any external_body axiom asserting `==` between structurally distinct spec objects.

