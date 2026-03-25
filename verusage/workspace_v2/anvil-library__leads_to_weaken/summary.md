# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_weaken.rs`
**Date:** 2026-03-24T04:14:23Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `always_implies_to_leads_to` and `leads_to_trans` are independent external_body axioms bridging distinct intensional closure gaps — the former lifting always-implication to leads_to (requiring eventually introduction), the latter composing two leads_to chains transitively. The remaining three are false positives: reflexive leads_to is a tautology, false-spec entailment is ex falso, and identity weakening is a correct no-op.

## True Positives (Spec Issues)

### always_implies_to_leads_to_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_implies_to_leads_to` is external_body lifting `always(p.implies(q))` to `p.leads_to(q)` which equals `always(p.implies(eventually(q)))`. This requires resolving that `q.satisfied_by(ex)` implies `eventually(q).satisfied_by(ex)` through the eventually closure — an unverified trust assumption.

### leads_to_trans_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_trans` is external_body composing two leads_to properties through nested always/implies/eventually closures. This is a separate unverified trust assumption bridging intensional closure gaps in the transitive composition.

## All Candidates

### φ1: always_implies_to_leads_to_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_implies_to_leads_to is external_body lifting always(p=>q) to p.leads_to(q) — the leads_to definition wraps always/implies/eventually in nested closures creating intensional gaps resolved without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_implies_to_leads_to` is external_body lifting `always(p.implies(q))` to `p.leads_to(q)` which equals `always(p.implies(eventually(q)))`. This requires resolving that `q.satisfied_by(ex)` implies `eventually(q).satisfied_by(ex)` through the eventually closure — an unverified trust assumption.

### φ2: leads_to_trans_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_trans is external_body composing two leads_to through nested always/implies/eventually closures — bridges intensional gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_trans` is external_body composing two leads_to properties through nested always/implies/eventually closures. This is a separate unverified trust assumption bridging intensional closure gaps in the transitive composition.

### φ3: implication_strengthens_to_leads_to
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** always_implies_to_leads_to converts a trivial tautology always(p=>p) into p.leads_to(p) which asserts eventual reachability — the axiom silently upgrades logical implication to temporal liveness without requiring a witness
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `p.leads_to(p)` is a standard temporal logic tautology — any property trivially leads to itself (witness at suffix 0). The upgrade from `always(p=>p)` to `p.leads_to(p)` is semantically correct.

### φ4: false_spec_any_leads_to
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously entails any always-implication and thus any leads_to — the external_body axiom propagates vacuous truth without detecting spec inconsistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An unsatisfiable spec vacuously entailing anything is standard ex falso quodlibet. This is not a spec gap.

### φ5: weaken_to_trivial
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** leads_to_weaken with identity implications should be a no-op but exercises both external_body axioms — if either axiom has unintended side effects this would expose them
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `leads_to_weaken` with identity implications is indeed a no-op producing the same conclusion as its premise. This is expected behavior, not a spec gap.

