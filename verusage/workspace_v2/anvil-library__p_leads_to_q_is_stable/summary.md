# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/p_leads_to_q_is_stable.rs`
**Date:** 2026-03-24T04:17:39Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 2
- False positives: 0

## Summary

Two true positives: `always_p_is_stable` is the root external_body axiom bridging `always(p) ==> always(always(p))` through nested closure gaps, and the leads_to stability lemma adds a secondary trust surface by relying on intensional identity between `p.leads_to(q)` and the `always(p.implies(eventually(q)))` substituted into the axiom's stable wrapper.

## True Positives (Spec Issues)

### always_p_is_stable_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_p_is_stable` is external_body deriving `valid(stable(always(p)))` which requires resolving `always(p) ==> always(always(p))` through nested `TempPred::new` closures — stable's closure, implies's closure, and always's closure all create intensional gaps resolved without proof.

### leads_to_stable_intensional_bridge
- **Confidence:** medium
- **Reasoning:** `p_leads_to_q_is_stable` calls `always_p_is_stable(p.implies(eventually(q)))`, which yields `valid(stable(always(p.implies(eventually(q)))))`. The verified lemma relies on Verus equating `p.leads_to(q)` (defined as `always(p.implies(eventually(q)))`) with the `always(p.implies(eventually(q)))` inside the stable wrapper — this is an intensional closure identity that SMT resolves only because both sides reduce to the same `TempPred::new` construction, but `stable` wraps an additional implies/always layer creating fresh closure nesting beyond φ1.

## All Candidates

### φ1: always_p_is_stable_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_p_is_stable is external_body deriving valid(stable(always(p))) — stable wraps always(p).implies(always(always(p))) in nested closures creating intensional gaps resolved without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_p_is_stable` is external_body deriving `valid(stable(always(p)))` which requires resolving `always(p) ==> always(always(p))` through nested `TempPred::new` closures — stable's closure, implies's closure, and always's closure all create intensional gaps resolved without proof.

### φ2: leads_to_stable_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** p_leads_to_q_is_stable relies on always_p_is_stable with p.implies(eventually(q)) — the substitution equates p.leads_to(q) with always(p.implies(eventually(q))) intensionally through closure identity
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `p_leads_to_q_is_stable` calls `always_p_is_stable(p.implies(eventually(q)))`, which yields `valid(stable(always(p.implies(eventually(q)))))`. The verified lemma relies on Verus equating `p.leads_to(q)` (defined as `always(p.implies(eventually(q)))`) with the `always(p.implies(eventually(q)))` inside the stable wrapper — this is an intensional closure identity that SMT resolves only because both sides reduce to the same `TempPred::new` construction, but `stable` wraps an additional implies/always layer creating fresh closure nesting beyond φ1.

### φ3: always_always_from_always
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** always_p_is_stable gives always(p) ==> always(always(p)) for all executions — this derives always(always(p)) from always(p), requiring intensional resolution of nested always closures without proof

### φ4: stable_implies_always_preservation
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** stable(leads_to) applied at ex upgrades leads_to at current state to always(leads_to) — the stable closure bridges implies and always wrapping without verifying the nested closure chain

### φ5: any_always_self_reinforcing
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** always(false) is vacuously satisfiable only on empty domains — always_p_is_stable still derives always(always(false)) from always(false), propagating the vacuous case through intensional closure gaps

