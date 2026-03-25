# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_rank_step_one.rs`
**Date:** 2026-03-24T04:06:13Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `leads_to_rank_step_one_help` is an external_body axiom performing induction over ranked leads_to steps, combining transitive leads_to composition with intensional closure gap resolution in a single unverified assumption. The remaining three are false positives: reflexive leads_to is a tautology, false-spec entailment is ex falso, and the full chain property is a direct restatement of the verified lemma.

## True Positives (Spec Issues)

### leads_to_rank_step_one_help_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_rank_step_one_help` is external_body performing induction to transitively chain leads_to steps. Each transitive composition requires resolving intensional closure gaps in the nested always/implies/eventually structure — this is an unverified trust assumption combining induction with leads_to transitivity.

## All Candidates

### φ1: leads_to_rank_step_one_help_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_rank_step_one_help is external_body performing induction over n to chain leads_to steps — the transitive composition of leads_to through nested always/implies/eventually closures bridges multiple intensional gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_rank_step_one_help` is external_body performing induction to transitively chain leads_to steps. Each transitive composition requires resolving intensional closure gaps in the nested always/implies/eventually structure — this is an unverified trust assumption combining induction with leads_to transitivity.

### φ2: rank_zero_leads_to_self
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** at n=0 the base case derives p(0).leads_to(p(0)) without any premise — the external_body axiom must handle this reflexive case internally, requiring leads_to reflexivity through unverified closure resolution
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `p(0).leads_to(p(0))` is a standard temporal logic tautology — any property trivially leads to itself (witness at suffix 0). This is expected and desirable.

### φ3: vacuous_premise_any_leads_to
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** instantiating p as a constant function makes all step premises trivially q.leads_to(q) — the external_body axiom derives this for any spec without verifying leads_to reflexivity

### φ4: false_spec_any_rank_leads_to
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously satisfies all step premises and thus entails any ranked leads_to chain — the external_body axiom propagates vacuous truth without detecting spec inconsistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An unsatisfiable spec vacuously entailing anything is standard ex falso quodlibet. The premises are vacuously true because no execution satisfies the spec — this is not a spec gap.

### φ5: single_step_amplified_to_full_chain
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_rank_step_one composes unboundedly many leads_to steps through the external_body helper — each composition trusts the axiom to resolve fresh intensional closure gaps in the transitive leads_to chain
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct restatement of the verified lemma `leads_to_rank_step_one` — chaining single-step leads_to into a full chain to rank 0 is the intended and expected property. No new trust surface beyond φ1.

