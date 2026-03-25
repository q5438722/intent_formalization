# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/entails_and_different_temp.rs`
**Date:** 2026-03-24T03:53:45Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `implies_apply` is an external_body axiom bridging the intensional closure gap in TempPred modus ponens. The remaining four are false positives — monotonicity of conjunction, ex falso, self-conjunction redundancy, and valid-implies-entailed are all standard and expected logical properties.

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

### φ2: entails_and_weakens_to_single
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** and-ing an arbitrary spec2 still entails p from spec1 alone — the second conjunct is completely unconstrained, meaning spec1.and(anything) entails p, weakening the combined specification's discriminating power
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Conjunction strengthens the antecedent: if `spec1 ⟹ p`, then `spec1 ∧ spec2 ⟹ p` is standard monotonicity of entailment. This is expected and desirable.

### φ3: entails_and_false_spec
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** unsatisfiable specs vacuously entail any conjunction — entails_and_different_temp propagates this without checking satisfiability, potentially hiding specification errors
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A false/unsatisfiable spec vacuously entailing anything is standard ex falso quodlibet. This is a fundamental property of implication, not a spec gap.

### φ4: entails_and_self_redundant
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** using spec.and(spec) — which should be equivalent to spec — requires the external_body axiom to resolve the and-closure's intensional gap, and the result hides that spec alone should suffice
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `spec.and(spec).entails(p.and(q))` from `spec.entails(p)` and `spec.entails(q)` is a correct instantiation of `entails_and_different_temp` with spec1=spec2=spec. The result is expected and the `and(spec)` redundancy is harmless.

### φ5: valid_implies_entails_anything
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** if valid(p) holds then spec.entails(p) for any spec — implies_apply resolves the closure bridge to make any valid property trivially entailed, collapsing the distinction between validity and entailment
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** If `p` holds on all executions (`valid(p)`), then any spec trivially entails `p` — this is the definition of validity. The "collapse" between validity and entailment is correct: valid properties are entailed by everything.

