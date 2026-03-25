# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/stable_and_temp.rs`
**Date:** 2026-03-24T04:23:15Z
**Status:** COMPLETE

## Stats

- Candidates generated: 2
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive: `stable_unfold` is the sole external_body axiom bridging stable's nested closure evaluation to a bare implication-with-forall. The conjunction stability lemma is a verified consequence with no independent trust surface.

## True Positives (Spec Issues)

### stable_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `stable_unfold` is external_body bridging `stable(p).satisfied_by(ex)` — which evaluates `p.implies(always(p))` through nested `TempPred::new` closures — to a bare `p.satisfied_by(ex) ==> forall |i| p.satisfied_by(ex.suffix(i))`. Unverified trust assumption resolving the intensional closure gap.

## All Candidates

### φ1: stable_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** stable_unfold is external_body bridging stable(p)'s nested closure (p.implies(always(p)) wrapped in TempPred::new) to a bare implication with forall — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `stable_unfold` is external_body bridging `stable(p).satisfied_by(ex)` — which evaluates `p.implies(always(p))` through nested `TempPred::new` closures — to a bare `p.satisfied_by(ex) ==> forall |i| p.satisfied_by(ex.suffix(i))`. Unverified trust assumption resolving the intensional closure gap.

### φ2: stable_and_valid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** stable_and_temp derives stability of conjunction from component stability — chains stable_unfold's closure bridge twice plus resolves p.and(q)'s TempPred::new conjunction closure against always's forall closure
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Stability of conjunction given component stability is a correct temporal logic property. This is a verified lemma whose only trust dependency is `stable_unfold` (φ1) — no new trust surface.

