# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_rank_step_one_usize.rs`
**Date:** 2026-03-24T04:08:58Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `leads_to_rank_step_one_usize_help` is an external_body axiom combining induction with transitive leads_to composition and reflexivity through unverified intensional closure resolution, and the usize-indexed rank function silently bounds the ranking argument to platform word size unlike a nat-indexed variant. The remaining three are false positives: ex falso from unsatisfiable spec, reflexive leads_to tautology, and a correct boundary instantiation of the verified lemma.

## True Positives (Spec Issues)

### leads_to_rank_step_one_usize_help_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_rank_step_one_usize_help` is external_body performing induction over usize to transitively chain leads_to steps. Each chain link requires resolving intensional closure gaps in nested always/implies/eventually — this combines induction, leads_to reflexivity, and leads_to transitivity in a single unverified trust assumption.

### usize_rank_limited_vs_nat
- **Confidence:** medium
- **Reasoning:** Using `usize` instead of `nat` silently bounds the ranking argument to `usize::MAX`. If a system requires a rank function exceeding platform word size, this lemma cannot express it — a subtle limitation compared to the nat-indexed variant that could mask liveness proof incompleteness.

## All Candidates

### φ1: leads_to_rank_step_one_usize_help_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_rank_step_one_usize_help is external_body performing induction and transitive leads_to composition — the chaining of always/implies/eventually closures bridges multiple intensional gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_rank_step_one_usize_help` is external_body performing induction over usize to transitively chain leads_to steps. Each chain link requires resolving intensional closure gaps in nested always/implies/eventually — this combines induction, leads_to reflexivity, and leads_to transitivity in a single unverified trust assumption.

### φ2: usize_rank_limited_vs_nat
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** quantifying over usize silently bounds the ranking argument to usize::MAX — unlike a nat-indexed variant, rank functions exceeding platform word size cannot be expressed, potentially masking liveness proof incompleteness
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Using `usize` instead of `nat` silently bounds the ranking argument to `usize::MAX`. If a system requires a rank function exceeding platform word size, this lemma cannot express it — a subtle limitation compared to the nat-indexed variant that could mask liveness proof incompleteness.

### φ3: false_spec_any_rank_leads_to
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously satisfies all step premises and thus entails any ranked leads_to chain — the external_body axiom propagates vacuous truth without detecting spec inconsistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An unsatisfiable spec vacuously entailing anything is standard ex falso quodlibet. This is not a spec gap.

### φ4: rank_zero_leads_to_self
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** at n=0 the external_body axiom must internally derive leads_to reflexivity through nested always/implies/eventually closures — this base case requires unverified intensional closure resolution
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `p(0).leads_to(p(0))` is a standard temporal logic tautology — any property trivially leads to itself. This is expected and desirable.

### φ5: usize_max_full_chain
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** chaining usize::MAX steps of transitive leads_to composition through the external_body helper — the decreases clause on usize handles the full range with each step invoking unverified intensional closure resolution
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct instantiation of the inductive lemma at usize::MAX. No new trust surface beyond φ1 — the decreases clause on usize is well-founded.

