# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_exists_intro.rs`
**Date:** 2026-03-24T04:05:33Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 3
- False positives: 0

## Summary

Three true positives: `implies_apply`, `tla_forall_leads_to_equality1`, and `spec_entails_tla_forall` are independent external_body axioms each bridging distinct intensional closure gaps — modus ponens through TempPred::new wrapping, structural equality of deeply nested forall-leads_to vs leads_to-exists closure trees, and pointwise-to-bundled entailment lifting through tla_forall's closure wrapper, respectively.

## True Positives (Spec Issues)

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is an unverified trust assumption resolving the intensional closure gap.

### tla_forall_leads_to_equality_intensional
- **Confidence:** high
- **Reasoning:** `tla_forall_leads_to_equality1` is external_body forcing `==` (intensional equality) between two `TempPred`s built from deeply nested, structurally distinct closures — `tla_forall(|a| a_to_p(a).leads_to(q))` vs `tla_exists(a_to_p).leads_to(q)`. This is a substantial unverified trust assumption equating complex closure trees.

### spec_entails_tla_forall_intensional_bridge
- **Confidence:** high
- **Reasoning:** `spec_entails_tla_forall` is external_body bridging pointwise entailment to entailment of `tla_forall`, where the `tla_forall` closure wrapping introduces an intensional gap between `forall |a| spec.entails(a_to_p(a))` and `spec.entails(tla_forall(a_to_p))`. This is a separate unverified trust assumption.

## All Candidates

### φ1: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping and direct implication — silently resolves closure identity mismatch
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is an unverified trust assumption resolving the intensional closure gap.

### φ2: tla_forall_leads_to_equality_intensional
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_leads_to_equality1 is external_body equating two TempPreds built from deeply nested closures (forall-of-leads_to vs leads_to-of-exists) — intensionally distinct structures forced equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_forall_leads_to_equality1` is external_body forcing `==` (intensional equality) between two `TempPred`s built from deeply nested, structurally distinct closures — `tla_forall(|a| a_to_p(a).leads_to(q))` vs `tla_exists(a_to_p).leads_to(q)`. This is a substantial unverified trust assumption equating complex closure trees.

### φ3: spec_entails_tla_forall_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** spec_entails_tla_forall is external_body bridging pointwise entailment (forall a, spec entails a_to_p(a)) to entailment of tla_forall — the tla_forall closure wrapping introduces an intensional gap
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `spec_entails_tla_forall` is external_body bridging pointwise entailment to entailment of `tla_forall`, where the `tla_forall` closure wrapping introduces an intensional gap between `forall |a| spec.entails(a_to_p(a))` and `spec.entails(tla_forall(a_to_p))`. This is a separate unverified trust assumption.

### φ4: leads_to_exists_false_antecedent
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** when every a_to_p(a) is unsatisfiable, the individual leads_to are vacuously true, and the conclusion becomes tla_exists(false).leads_to(q) — vacuous truth propagates through the external_body axioms without detection

### φ5: leads_to_exists_any_spec
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** if each a_to_p(a) pointwise entails q (valid implication), then any spec — including unrelated or unsatisfiable ones — entails tla_exists(a_to_p).leads_to(q), making the spec parameter irrelevant

