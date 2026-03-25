# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/next_preserves_inv_rec.rs`
**Date:** 2026-03-24T04:15:01Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `execution_equality` is the sole external_body axiom forcing intensional spec_fn equality from pointwise extensional equivalence. The remaining three are false positives — suffix(0) identity, suffix composition, and universal invariant preservation are all semantically correct and desirable properties that follow from the axiom or the inductive lemma.

## True Positives (Spec Issues)

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. Structurally distinct closures collapsed to equal without proof — the root unverified trust assumption.

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality forces intensional equality of spec_fn from pointwise extensional equivalence — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. Structurally distinct closures collapsed to equal without proof — the root unverified trust assumption.

### φ2: suffix_zero_equals_self
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** suffix(0) produces a fresh closure |i| f(i + 0) which is intensionally distinct from f — execution_equality silently collapses this gap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ex.suffix(0) == ex` is a semantically correct and desirable property. This is an expected consequence of `execution_equality`, not a spec gap itself.

### φ3: suffix_composition_equals_addition
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** suffix(a).suffix(b) nests two closures |i| (|j| f(j+a))(i+b) while suffix(a+b) has |i| f(i+a+b) — execution_equality forces intensional equality of structurally distinct nested closures
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ex.suffix(a).suffix(b) == ex.suffix(a + b)` is semantically correct suffix associativity. This is an expected consequence of `execution_equality`, not a spec gap.

### φ4: inv_preserved_for_all_indices
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** next_preserves_inv_rec generalizes to a universal quantifier over all indices — if execution_equality is unsound, the entire inductive invariant preservation is compromised
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Universal invariant preservation is the intended and correct generalization of the inductive lemma. This is the whole point of `next_preserves_inv_rec` — not a spec gap.

### φ5: trivial_inv_any_execution
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** instantiating inv=next makes the inductive step trivially true from the first premise alone — the lemma degenerates into reproving a premise, exercising execution_equality without meaningful invariant reasoning

