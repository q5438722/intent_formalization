# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/entails_trans.rs`
**Date:** 2026-03-24T03:57:04Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `implies_apply` is the sole external_body axiom bridging the intensional closure gap in TempPred modus ponens. The remaining four are false positives — transitive chaining, valid-implies-entailed, ex falso, and reflexivity are all standard and expected logical properties.

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

### φ2: entails_trans_unbounded_chain
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chaining entails_trans composes implies_apply across arbitrarily many intermediaries — each step trusts the external_body axiom to resolve a fresh intensional closure gap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Transitivity of entailment is a standard and desirable logical property. It composes `entails_trans` correctly with no new trust surface beyond φ1.

### φ3: valid_implies_entails_anything
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** if valid(p) holds then any spec vacuously entails p — implies_apply bridges the closure gap so valid properties are entailed by everything, collapsing the distinction between validity and entailment
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** If `p` holds on all executions (`valid(p)`), then any spec trivially entails `p` — this is the definition of validity. The "collapse" is correct and expected.

### φ4: entails_false_spec_vacuous
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously entails any property — entails propagates this without checking satisfiability, potentially hiding specification errors
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An unsatisfiable spec vacuously entailing anything is standard ex falso quodlibet — a fundamental property of implication, not a spec gap.

### φ5: entails_reflexive
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** p.entails(p) requires implies_apply to bridge the intensional gap in p.implies(p).satisfied_by(ex) through TempPred::new — if reflexivity needs an external_body axiom, the closure wrapping is not semantically transparent
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Reflexivity of entailment is a basic and desirable logical property. That it requires `implies_apply` is an implementation detail — the property itself is expected.

