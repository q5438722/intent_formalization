# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_rank_step_one_usize_help.rs`
**Date:** 2026-03-24T04:08:15Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Two definite true positives: `leads_to_self_temp` and `leads_to_trans` are independent external_body axioms bridging intensional closure gaps in reflexive and transitive leads_to reasoning. One medium-confidence true positive: the usize-indexed rank function silently bounds the ranking argument to platform word size, unlike a nat-indexed variant. The remaining two are false positives: the n=0 base case and usize::MAX chain are correct instantiations of the verified inductive lemma.

## True Positives (Spec Issues)

### leads_to_self_temp_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_self_temp` is external_body deriving `valid(p.leads_to(p))` which requires resolving nested `always(p.implies(eventually(p)))` closure gaps. This is an unverified trust assumption.

### leads_to_trans_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_trans` is external_body composing two leads_to properties into a transitive chain through nested always/implies/eventually closures. This is a separate unverified trust assumption.

### usize_rank_limited_vs_nat
- **Confidence:** medium
- **Reasoning:** Using `usize` instead of `nat` as the rank index silently bounds the ranking argument to `usize::MAX`. If a system requires a rank function exceeding platform word size, this lemma cannot express it — a subtle limitation compared to the nat-indexed variant that could mask liveness proof incompleteness.

## All Candidates

### φ1: leads_to_self_temp_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_self_temp is external_body deriving valid(p.leads_to(p)) — resolving nested always/implies/eventually closure gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_self_temp` is external_body deriving `valid(p.leads_to(p))` which requires resolving nested `always(p.implies(eventually(p)))` closure gaps. This is an unverified trust assumption.

### φ2: leads_to_trans_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_trans is external_body composing two leads_to through nested always/implies/eventually closures — bridges intensional gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_trans` is external_body composing two leads_to properties into a transitive chain through nested always/implies/eventually closures. This is a separate unverified trust assumption.

### φ3: usize_rank_overflow_at_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** the usize-indexed rank function at n=0 relies on leads_to_self_temp — but using usize instead of nat means the step premise n > 0 with (n-1) as usize could interact with usize overflow semantics in surprising ways at boundary
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** At n=0 the else branch fires and calls `leads_to_self_temp(p(0))` directly. There is no overflow — `n > 0` is false so `(n-1) as usize` is never evaluated. This is correct base case behavior.

### φ4: usize_max_rank_step
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** chaining usize::MAX steps of leads_to_trans through the inductive helper — the decreases clause on usize must handle the full range, and each step invokes the external_body axiom on a fresh intensional closure
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Chaining usize::MAX steps is a correct instantiation of the verified inductive lemma. The decreases clause on usize is well-founded, and each step correctly applies. No new trust surface beyond φ1 and φ2.

### φ5: usize_rank_limited_vs_nat
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** quantifying over all usize values produces a leads_to chain limited to usize::MAX steps — unlike the nat-indexed variant, this cannot express rank functions exceeding platform word size, silently bounding the ranking argument
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Using `usize` instead of `nat` as the rank index silently bounds the ranking argument to `usize::MAX`. If a system requires a rank function exceeding platform word size, this lemma cannot express it — a subtle limitation compared to the nat-indexed variant that could mask liveness proof incompleteness.

