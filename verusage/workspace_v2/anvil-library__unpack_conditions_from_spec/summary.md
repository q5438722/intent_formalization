# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/unpack_conditions_from_spec.rs`
**Date:** 2026-03-24T04:37:20Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Four true positives from independent external_body axioms: `execution_equality` (pointwise state equality to intensional Execution equality), `implies_apply` (modus ponens through closure gap), `stable_unfold` (stable predicate elimination through nested closures), and `leads_to_unfold` (leads-to elimination through nested closures). One false positive: `suffix(0) == self` is a correct arithmetic identity with no independent trust surface.

## True Positives (Spec Issues)

### execution_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `execution_equality` is external_body forcing intensional `==` of `Execution` from pointwise state equality. Unverified trust assumption collapsing `spec_fn(nat)->T` without proof.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — a `TempPred::new` closure-wrapped implication — to modus ponens. Unverified trust assumption.

### stable_unfold_closure_bridge
- **Confidence:** high
- **Reasoning:** `stable_unfold` is external_body bridging `stable(p)`'s nested `implies`/`always` closure wrapping to a bare conditional forall. Unverified trust assumption resolving multiple closure layers.

### leads_to_unfold_closure_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s `always(implies(eventually))` nested closure wrapping to a bare forall. Unverified trust assumption.

## All Candidates

### φ1: execution_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body forcing intensional equality of Execution from pointwise state equality — collapses structurally distinct spec_fn(nat)->T without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `execution_equality` is external_body forcing intensional `==` of `Execution` from pointwise state equality. Unverified trust assumption collapsing `spec_fn(nat)->T` without proof.

### φ2: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q)'s TempPred::new closure-wrapped implication to modus ponens — resolves closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — a `TempPred::new` closure-wrapped implication — to modus ponens. Unverified trust assumption.

### φ3: stable_unfold_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** stable_unfold is external_body bridging stable(p)'s nested implies/always closure wrapping to a bare forall — resolves multiple layers of closure identity without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `stable_unfold` is external_body bridging `stable(p)`'s nested `implies`/`always` closure wrapping to a bare conditional forall. Unverified trust assumption resolving multiple closure layers.

### φ4: leads_to_unfold_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_unfold is external_body bridging leads_to's always(implies(eventually)) nested closure wrapping to a bare forall — resolves multiple layers of closure identity without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s `always(implies(eventually))` nested closure wrapping to a bare forall. Unverified trust assumption.

### φ5: suffix_zero_equals_self
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** suffix(0) creates a new Execution with closure |i| nat_to_state(i+0) which is intensionally distinct from nat_to_state — execution_equality forces them equal, but this relies on the unverified axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ex.suffix(0)` is pointwise equal to `ex` since `i + 0 == i` for all `nat`. This is a semantically correct identity with no new trust surface beyond `execution_equality` (φ1). The property itself is desirable.

