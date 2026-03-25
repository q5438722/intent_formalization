# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_self_temp.rs`
**Date:** 2026-03-24T04:11:49Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `eventually_proved_by_witness` and `execution_equality` are independent external_body axioms bridging distinct intensional closure gaps — existential witness introduction into eventually's closure wrapper, and extensional-to-intensional spec_fn equality, respectively. The remaining three are false positives: suffix(0) identity and suffix composition are semantically correct consequences of execution_equality, and reflexive leads_to is a standard temporal logic tautology derived by a verified lemma.

## True Positives (Spec Issues)

### eventually_proved_by_witness_intensional_bridge
- **Confidence:** high
- **Reasoning:** `eventually_proved_by_witness` is external_body introducing a concrete witness into `eventually`'s `TempPred::new` existential closure. This bridges the intensional gap between a known `p.satisfied_by(ex.suffix(witness_idx))` and the closure-wrapped exists — an unverified trust assumption.

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. Structurally distinct closures are collapsed to equal without proof — this is the root unverified trust assumption in the file.

## All Candidates

### φ1: eventually_proved_by_witness_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** eventually_proved_by_witness is external_body introducing a concrete witness into eventually's TempPred::new existential closure — bridges the intensional gap between a known witness and the closure-wrapped exists
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `eventually_proved_by_witness` is external_body introducing a concrete witness into `eventually`'s `TempPred::new` existential closure. This bridges the intensional gap between a known `p.satisfied_by(ex.suffix(witness_idx))` and the closure-wrapped exists — an unverified trust assumption.

### φ2: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality forces intensional equality of spec_fn from pointwise extensional equivalence — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. Structurally distinct closures are collapsed to equal without proof — this is the root unverified trust assumption in the file.

### φ3: suffix_zero_equals_self
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** suffix(0) produces a fresh closure |i| f(i + 0) which is intensionally distinct from f — execution_equality silently collapses this gap, deriving structural identity from pointwise equivalence
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ex.suffix(0) == ex` is a semantically correct and desirable property — the suffix at position 0 should be the original execution. This is a correct consequence of `execution_equality`, not a spec gap.

### φ4: suffix_composition_equals_addition
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** suffix(a).suffix(b) nests two closures |i| (|j| f(j+a))(i+b) while suffix(a+b) has |i| f(i+a+b) — execution_equality forces intensional equality of structurally distinct nested closures
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ex.suffix(a).suffix(b) == ex.suffix(a + b)` is a semantically correct and desirable property of execution suffixes — standard associativity. This is an expected consequence of `execution_equality`, not a spec gap.

### φ5: leads_to_self_any_pred
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** leads_to_self_temp derives valid(p.leads_to(p)) relying on both external_body axioms — if either axiom is unsound the reflexive leads_to property could be incorrectly established for any predicate
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Reflexive leads_to (`p ~> p`) is a standard temporal logic tautology. `leads_to_self_temp` is a verified lemma (not external_body) that correctly derives this from the two axioms. No new trust surface beyond φ1 and φ2.

