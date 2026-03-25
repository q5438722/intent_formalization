# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_apply.rs`
**Date:** 2026-03-24T04:04:08Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives: `leads_to_unfold`, `implies_apply`, and `execution_equality` are independent external_body axioms each bridging distinct intensional closure gaps — always-forall extraction, TempPred modus ponens, and spec_fn extensional-to-intensional equality, respectively. The remaining two are false positives: suffix(0) identity is a correct semantic property, and leads_to_apply is a standard temporal logic rule.

## True Positives (Spec Issues)

### leads_to_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_unfold` is external_body bridging the forall embedded inside `always`'s `TempPred::new` closure (via `leads_to` = `always(p.implies(eventually(q)))`) to a bare forall in the ensures. This is an unverified trust assumption resolving the intensional closure gap.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is a separate unverified trust assumption resolving the intensional closure gap.

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

## All Candidates

### φ1: leads_to_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_unfold is external_body bridging the forall inside always's TempPred::new closure (in leads_to's definition) to a bare forall in the ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_unfold` is external_body bridging the forall embedded inside `always`'s `TempPred::new` closure (via `leads_to` = `always(p.implies(eventually(q)))`) to a bare forall in the ensures. This is an unverified trust assumption resolving the intensional closure gap.

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

### φ4: suffix_zero_equals_self
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** suffix(0) creates a fresh closure |i| f(i+0) intensionally distinct from f — execution_equality silently bridges this; leads_to_apply critically depends on this identity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ex.suffix(0) == ex` is semantically correct since `f(i+0) == f(i)` for all `i`. This is a desirable property and the intended use case of `execution_equality`.

### φ5: leads_to_apply_any_spec
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** leads_to_apply combines all three external_body axioms to derive eventually(q) from spec — the full trust chain (implies_apply + leads_to_unfold + execution_equality) is exercised without any verified link
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct restatement of the verified lemma `leads_to_apply` — if spec entails both p and p~>q, then spec entails eventually(q). This is a standard and expected temporal logic property.

