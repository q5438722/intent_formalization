# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/spec_entails_tla_forall.rs`
**Date:** 2026-03-24T04:22:46Z
**Status:** COMPLETE

## Stats

- Candidates generated: 2
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive: `implies_apply` is the sole external_body axiom bridging the intensional closure gap for TempPred modus ponens. The tla_forall introduction lemma is a verified consequence with no independent trust surface.

## True Positives (Spec Issues)

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a `TempPred::new` closure — to the direct modus ponens conclusion `q.satisfied_by(ex)`. Unverified trust assumption resolving the intensional closure gap.

## All Candidates

### φ1: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping to modus ponens — resolves closure identity mismatch without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a `TempPred::new` closure — to the direct modus ponens conclusion `q.satisfied_by(ex)`. Unverified trust assumption resolving the intensional closure gap.

### φ2: spec_entails_tla_forall_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** spec_entails_tla_forall combines pointwise entailments into tla_forall entailment — relies on implies_apply's closure bridge to resolve tla_forall's TempPred::new forall-wrapping
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `spec_entails_tla_forall` is a verified lemma whose proof body is checked by Verus. Its only trust dependency is `implies_apply` (φ1) — no new trust surface beyond the root axiom.

