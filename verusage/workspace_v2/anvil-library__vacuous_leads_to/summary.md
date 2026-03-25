# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/vacuous_leads_to.rs`
**Date:** 2026-03-24T04:42:05Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive from the external_body axiom `implies_apply` (modus ponens through closure gap). Three false positives: `false_pred` unsatisfiability is SMT-resolvable, equality propagation through `satisfied_by` follows from intensional equality, and ex falso quodlibet for `false_pred.leads_to(q)` is a correct vacuous truth.

## True Positives (Spec Issues)

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — a `TempPred::new` closure-wrapped implication — to modus ponens. Unverified trust assumption.

## All Candidates

### φ1: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q)'s TempPred::new closure-wrapped implication to modus ponens — resolves closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — a `TempPred::new` closure-wrapped implication — to modus ponens. Unverified trust assumption.

### φ2: false_pred_unsatisfiable
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** false_pred = not(true_pred()) should be unsatisfiable — if this fails to verify, the closure nesting in not/true_pred/lift_state prevents SMT from resolving it, exposing a closure opacity issue
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `false_pred() = not(true_pred())` unfolds through `not`/`true_pred`/`lift_state` to `!true == false`. SMT resolves this directly — no trust gap, just a correct property.

### φ3: vacuous_leads_to_requires_equality
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** vacuous_leads_to relies on p.and(r) == false_pred() as intensional equality — if two semantically-false-but-structurally-different TempPreds are passed, the equality precondition gates soundness

### φ4: false_pred_equality_gates_soundness
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** if p.and(r) == false_pred() holds intensionally, then for any execution it should be unsatisfiable — but the equality is between structurally distinct closures, so this tests whether intensional equality propagates correctly through satisfied_by
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** If `p.and(r) == false_pred()` holds intensionally, then `p.and(r).satisfied_by(ex) == false_pred().satisfied_by(ex) == false` follows from substitution. This is a correct consequence of intensional equality — no new trust surface.

### φ5: any_leads_to_from_false_antecedent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** derives that false_pred leads to anything under any spec entailing always(true) — tests whether the intensional equality false_pred.and(true_pred) == false_pred holds and whether ex falso quodlibet propagates through the leads_to machinery
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Ex falso quodlibet: `false_pred` is never satisfied, so `false_pred.leads_to(q)` holds vacuously. The intensional equality precondition is the only non-trivial part, and it's assumed. No new trust surface.

