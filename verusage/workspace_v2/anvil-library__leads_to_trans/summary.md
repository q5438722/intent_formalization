# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_trans.rs`
**Date:** 2026-03-24T04:13:49Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 5
- False positives: 0

## Summary

Five true positives: `always_unfold`, `eventually_unfold`, `implies_apply`, `execution_equality`, and `eventually_propagate_backwards` are independent external_body axioms each bridging distinct intensional closure gaps — always-forall extraction, eventually-exists extraction, TempPred modus ponens, spec_fn extensional-to-intensional equality, and eventually backward suffix propagation, respectively. The verified `leads_to_trans` lemma depends on all six of them (including `entails_apply` not flagged here).

## True Positives (Spec Issues)

### always_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_unfold` is external_body bridging `always(p).satisfied_by(ex)` — which evaluates a `TempPred::new` closure containing a forall — to a bare forall over `p.satisfied_by(ex.suffix(i))`. Unverified trust assumption resolving the intensional closure gap.

### eventually_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `eventually_unfold` is external_body bridging `eventually(p).satisfied_by(ex)` — which evaluates a `TempPred::new` closure containing an exists — to a bare exists over `p.satisfied_by(ex.suffix(i))`. Separate unverified trust assumption.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh `TempPred::new` closure — to modus ponens. Separate unverified trust assumption resolving the intensional closure gap.

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. Structurally distinct closures collapsed to equal without proof — the root unverified trust assumption.

### eventually_propagate_backwards_intensional_bridge
- **Confidence:** high
- **Reasoning:** `eventually_propagate_backwards` is external_body shifting `eventually(p)` from `ex.suffix(i)` to `ex`, requiring that a witness `j` at `ex.suffix(i).suffix(j)` maps to `ex.suffix(i+j)` — bridges the suffix composition intensional gap without proof.

## All Candidates

### φ1: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging always's TempPred::new closure-wrapped forall to a bare forall in ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_unfold` is external_body bridging `always(p).satisfied_by(ex)` — which evaluates a `TempPred::new` closure containing a forall — to a bare forall over `p.satisfied_by(ex.suffix(i))`. Unverified trust assumption resolving the intensional closure gap.

### φ2: eventually_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** eventually_unfold is external_body bridging eventually's TempPred::new closure-wrapped exists to a bare exists in ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `eventually_unfold` is external_body bridging `eventually(p).satisfied_by(ex)` — which evaluates a `TempPred::new` closure containing an exists — to a bare exists over `p.satisfied_by(ex.suffix(i))`. Separate unverified trust assumption.

### φ3: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping — resolves closure identity mismatch without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh `TempPred::new` closure — to modus ponens. Separate unverified trust assumption resolving the intensional closure gap.

### φ4: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality forces intensional equality of spec_fn from pointwise extensional equivalence — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. Structurally distinct closures collapsed to equal without proof — the root unverified trust assumption.

### φ5: eventually_propagate_backwards_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** eventually_propagate_backwards is external_body shifting eventually(p) from a suffix back to the original execution — requires resolving that ex.suffix(i).suffix(j) witnesses ex.suffix(i+j), bridging intensional closure gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `eventually_propagate_backwards` is external_body shifting `eventually(p)` from `ex.suffix(i)` to `ex`, requiring that a witness `j` at `ex.suffix(i).suffix(j)` maps to `ex.suffix(i+j)` — bridges the suffix composition intensional gap without proof.

