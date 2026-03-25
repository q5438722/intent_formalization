# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_rank_step_one_help.rs`
**Date:** 2026-03-24T04:07:34Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `leads_to_self_temp` and `leads_to_trans` are independent external_body axioms bridging distinct intensional closure gaps — reflexive leads_to through nested always/implies/eventually wrapping, and transitive composition of two leads_to chains, respectively. The remaining three are false positives: unbounded chaining is a verified inductive composition, false-spec entailment is ex falso, and spec-independent reflexivity follows from validity.

## True Positives (Spec Issues)

### leads_to_self_temp_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_self_temp` is external_body deriving `valid(p.leads_to(p))` which requires resolving nested `always(p.implies(eventually(p)))` closure gaps — the reflexive leads_to through `TempPred::new` wrapping is an unverified trust assumption.

### leads_to_trans_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_trans` is external_body composing two leads_to properties into a transitive chain. This requires resolving the nested `always/implies/eventually` closure structure to prove the composed leads_to — a separate unverified trust assumption.

## All Candidates

### φ1: leads_to_self_temp_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_self_temp is external_body deriving valid(p.leads_to(p)) which makes any spec entail p.leads_to(p) — the reflexive leads_to requires resolving nested always/implies/eventually closure gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_self_temp` is external_body deriving `valid(p.leads_to(p))` which requires resolving nested `always(p.implies(eventually(p)))` closure gaps — the reflexive leads_to through `TempPred::new` wrapping is an unverified trust assumption.

### φ2: leads_to_trans_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_trans is external_body composing two leads_to into a transitive chain — each leads_to involves nested always/implies/eventually closures and the composition bridges intensional gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_trans` is external_body composing two leads_to properties into a transitive chain. This requires resolving the nested `always/implies/eventually` closure structure to prove the composed leads_to — a separate unverified trust assumption.

### φ3: leads_to_trans_chain_unbounded
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chaining leads_to_trans inductively over unbounded n composes arbitrarily many external_body axiom invocations — each step trusts leads_to_trans to resolve fresh intensional closure gaps
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct restatement of the verified lemma `leads_to_rank_step_one_help` which uses induction over `n` with `leads_to_trans` and `leads_to_self_temp`. No new trust surface beyond φ1 and φ2.

### φ4: false_spec_any_leads_to
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously entails any leads_to chain — leads_to_trans propagates vacuous truth without detecting spec inconsistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An unsatisfiable spec vacuously entailing anything is standard ex falso quodlibet. This is not a spec gap.

### φ5: leads_to_self_any_spec
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** leads_to_self_temp produces valid(p.leads_to(p)) with no preconditions — any spec (including unrelated or unsatisfiable ones) entails reflexive leads_to, making the spec parameter irrelevant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct consequence of φ1 — `valid(p.leads_to(p))` means every execution satisfies it, so any spec trivially entails it. The spec-independence is expected for valid properties.

