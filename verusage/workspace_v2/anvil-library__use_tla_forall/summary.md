# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/use_tla_forall.rs`
**Date:** 2026-03-24T04:38:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive from the external_body axiom `entails_apply`, which bridges `p.entails(q)` (nested `valid`/`implies`/`TempPred::new` closures) to pointwise modus ponens without proof. Four false positives: universal instantiation, entailment transitivity, forall extraction at a specific execution, and a restatement of the same axiom are all correct logical consequences with no independent trust surface.

## True Positives (Spec Issues)

### entails_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `entails_apply` is external_body bridging `p.entails(q)` — which unfolds through `valid`/`implies`/`TempPred::new` closures — to pointwise modus ponens. Unverified trust assumption.

## All Candidates

### φ1: entails_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** entails_apply is external_body bridging p.entails(q) — which unfolds through valid/implies/TempPred::new closures — to modus ponens, resolving closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `entails_apply` is external_body bridging `p.entails(q)` — which unfolds through `valid`/`implies`/`TempPred::new` closures — to pointwise modus ponens. Unverified trust assumption.

### φ2: use_tla_forall_instantiation
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chains entails_apply to instantiate tla_forall — if the closure bridge is unsound, universal instantiation through nested TempPred wrappers propagates unsoundly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Universal instantiation (`spec ⊨ ∀a.p(a)` implies `spec ⊨ p(a)`) is a standard logical consequence. This is a verified lemma with no new trust surface beyond φ1.

### φ3: entails_transitive_via_apply
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** derives entailment transitivity by chaining entails_apply — if the closure bridge resolves incorrectly, this enables chaining arbitrary entailment claims
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Entailment transitivity is a standard logical property. This chains `entails_apply` twice but introduces no new trust surface beyond φ1.

### φ4: tla_forall_vacuous_any_pred
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** extracts any a_to_p(a) from a single execution satisfying spec — relies on entails_apply resolving tla_forall's closure-wrapped forall to bare universal, which is unverified
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Extracting `a_to_p(a)` at a specific execution from `spec ⊨ ∀a.p(a)` and `spec(ex)` is standard universal instantiation plus modus ponens. No new trust surface beyond φ1.

### φ5: entails_apply_self_implies
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** entails_apply resolves p.entails(q) = valid(p.implies(q)) — which nests valid's forall inside implies' TempPred::new closure — to pointwise modus ponens, bridging two closure layers without proof
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This restates φ1 with `p.entails(q)` expanded to `valid(p.implies(q))`. Same axiom, same trust surface — not an independent finding.

