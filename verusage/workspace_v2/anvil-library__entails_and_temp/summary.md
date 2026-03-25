# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/entails_and_temp.rs`
**Date:** 2026-03-24T03:54:30Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `implies_apply` is an external_body axiom bridging the intensional closure gap in TempPred modus ponens. The remaining four are false positives — conjunction chaining, idempotence, valid-implies-entailed, and ex falso are all standard and expected logical properties.

## True Positives (Spec Issues)

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is an unverified trust assumption resolving the intensional closure gap.

## All Candidates

### φ1: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping and direct implication — silently resolves closure identity mismatch
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is an unverified trust assumption resolving the intensional closure gap.

### φ2: entails_and_infinite_chain
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** entails_and_temp composes unboundedly — chaining it builds arbitrarily deep nested and-closures, each requiring implies_apply to resolve a fresh intensional closure gap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Composing `entails_and_temp` to build `spec.entails(p.and(q).and(r))` from individual entailments is standard conjunction introduction — a correct and expected property. No new trust surface beyond what φ1 already identifies.

### φ3: entails_and_self_idempotent
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** p.and(p) creates a new closure intensionally distinct from p — if the spec entails p.and(p), the framework silently equates redundant conjunctions through the external_body axiom without proving idempotence
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `spec.entails(p)` implying `spec.entails(p.and(p))` is just conjunction idempotence, a standard and desirable logical property. It's a direct instantiation of `entails_and_temp` with no new trust surface.

### φ4: valid_implies_entails_anything
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** if valid(p) holds then any spec vacuously entails p — implies_apply bridges the closure gap so valid properties are entailed by everything, collapsing the distinction between validity and entailment
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** If `p` holds on all executions (`valid(p)`), then any spec trivially entails `p` — this is the definition of validity. The "collapse" is correct: valid properties are entailed by everything.

### φ5: entails_false_spec_vacuous
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously entails any conjunction — entails_and_temp propagates this without checking satisfiability, potentially hiding specification errors
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An unsatisfiable spec vacuously entailing anything is standard ex falso quodlibet — a fundamental property of implication, not a spec gap.

