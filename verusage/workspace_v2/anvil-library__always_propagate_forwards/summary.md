# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_propagate_forwards.rs`
**Date:** 2026-03-24T03:49:40Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `execution_equality` forces intensional spec_fn equality from extensional equivalence (the root unverified axiom), and `always_unfold` bridges a separate intensional closure gap. The remaining three are false positives — suffix composition and double nesting are semantically correct consequences, and chaining `always_propagate_forwards` exposes no new trust surface.

## True Positives (Spec Issues)

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

### always_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** The external_body `always_unfold` bridges the intensional gap between the `forall` embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures. This is a separate unverified trust assumption from `execution_equality`.

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body forcing intensional equality of spec_fn from pointwise extensional equivalence — two structurally distinct closures are collapsed to equal
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

### φ2: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging the forall inside always's TempPred::new closure and a bare forall in the ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body `always_unfold` bridges the intensional gap between the `forall` embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures. This is a separate unverified trust assumption from `execution_equality`.

### φ3: always_propagate_chains_arbitrarily
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chaining always_propagate_forwards yields always(p) on doubly-nested suffixes — the two external_body axioms compose to resolve arbitrary suffix nesting without explicit composition lemmas
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a straightforward double application of `always_propagate_forwards`, which is a verified lemma (not external_body). Chaining it is expected and correct — no new trust surface beyond the axioms it already depends on.

### φ4: suffix_composition_equals_addition
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** doubly-nested suffix creates a closure intensionally distinct from single-offset suffix — execution_equality collapses them without proof, and always_propagate_forwards depends on this
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Suffix composition is a semantically correct and desirable property. The intensional concern is entirely subsumed by φ1's identification of `execution_equality` — this is just a concrete instantiation.

### φ5: always_double_nesting
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always(p) implies always(always(p)) — the framework collapses the temporal nesting via always_propagate_forwards without an independent always-idempotence axiom, which could mask issues where inner always should have different semantics than outer
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `always(p) ⟹ always(always(p))` is the standard S4 axiom of temporal logic, a desirable and expected property. It follows directly from `always_propagate_forwards` with no new trust surface.

