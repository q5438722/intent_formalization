# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_framed_by_or.rs`
**Date:** 2026-03-24T04:06:55Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Four true positives: `leads_to_unfold`, `implies_apply`, `execution_equality`, and `eventually_proved_by_witness` are independent external_body axioms each bridging distinct intensional closure gaps — always-forall extraction, TempPred modus ponens, spec_fn extensional-to-intensional equality, and eventually existential witness introduction, respectively. The remaining one is a false positive: ex falso from an unsatisfiable spec is standard logic.

## True Positives (Spec Issues)

### leads_to_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s definition — `always(p.implies(eventually(q)))` with its nested `TempPred::new` closures — to a bare forall in the ensures. This is an unverified trust assumption resolving the intensional closure gap.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is a separate unverified trust assumption resolving the intensional closure gap.

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

### eventually_proved_by_witness_intensional_bridge
- **Confidence:** high
- **Reasoning:** `eventually_proved_by_witness` is external_body introducing a concrete witness into `eventually`'s `TempPred::new` existential closure. This bridges the intensional gap between a known `p.satisfied_by(ex.suffix(witness_idx))` and the closure-wrapped exists — a separate unverified trust assumption.

## All Candidates

### φ1: leads_to_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_unfold is external_body bridging the forall inside always's TempPred::new closure to a bare forall in the ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s definition — `always(p.implies(eventually(q)))` with its nested `TempPred::new` closures — to a bare forall in the ensures. This is an unverified trust assumption resolving the intensional closure gap.

### φ2: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping and direct implication — silently resolves closure identity mismatch
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is a separate unverified trust assumption resolving the intensional closure gap.

### φ3: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality forces intensional equality of spec_fn from pointwise extensional equivalence — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

### φ4: eventually_proved_by_witness_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** eventually_proved_by_witness is external_body introducing a witness into eventually's TempPred::new existential closure — bridges the intensional gap between a concrete witness and the closure-wrapped exists
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `eventually_proved_by_witness` is external_body introducing a concrete witness into `eventually`'s `TempPred::new` existential closure. This bridges the intensional gap between a known `p.satisfied_by(ex.suffix(witness_idx))` and the closure-wrapped exists — a separate unverified trust assumption.

### φ5: or_framing_false_spec
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously entails any leads_to including the or-framed version — vacuous truth propagates through the framing lemma without detecting spec inconsistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An unsatisfiable spec vacuously entailing anything is standard ex falso quodlibet. This is not a spec gap.

