# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_to_always_later.rs`
**Date:** 2026-03-24T03:51:48Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives: `execution_equality` forces intensional spec_fn equality from extensional equivalence (root axiom), `entails_trans` bridges transitivity across TempPred closures, and `always_propagate_forwards` transfers always across suffix operations — each an independent external_body trust assumption. Two false positives: suffix composition and always/later commutativity are semantically correct properties that merely instantiate or combine the identified axioms.

## True Positives (Spec Issues)

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

### entails_trans_intensional_bridge
- **Confidence:** high
- **Reasoning:** `entails_trans` is an independent external_body axiom bridging transitivity across chained `implies`/`TempPred::new` closures. This is a separate unverified trust assumption from `execution_equality`, operating on TempPred closures rather than Execution closures.

### always_propagate_forwards_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_propagate_forwards` is an independent external_body axiom transferring `always(p)` to `ex.suffix(i)`, which involves an intensionally distinct closure. This is a separate trust assumption that bridges the always closure across suffix operations.

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body forcing intensional equality of spec_fn from pointwise extensional equivalence — two structurally distinct closures are collapsed to equal
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

### φ2: suffix_composition_equals_addition
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** doubly-nested suffix creates a closure intensionally distinct from single-offset suffix — execution_equality collapses them without proof, and always_to_always_later depends on this
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Suffix composition is a semantically correct and desirable property. The intensional concern is entirely subsumed by φ1's identification of `execution_equality` as the root axiom.

### φ3: entails_trans_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** entails_trans is external_body bridging transitivity across chained implies/TempPred::new closures — resolves intensional closure identity without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `entails_trans` is an independent external_body axiom bridging transitivity across chained `implies`/`TempPred::new` closures. This is a separate unverified trust assumption from `execution_equality`, operating on TempPred closures rather than Execution closures.

### φ4: always_later_commutativity
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always and later commute freely through the external_body axioms — if always(later(p)) and later(always(p)) are both derivable from always(p), the temporal operators lose independent semantic content
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `always(p) ⟹ always(later(p))` and `always(p) ⟹ later(always(p))` are both standard and expected temporal logic properties. The commutativity is semantically correct — `always` at every suffix implies both shifting then checking all, and checking all then shifting.

### φ5: always_propagate_forwards_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_propagate_forwards is external_body transferring always(p) to ex.suffix(i) which has an intensionally distinct closure — the axiom silently equates these without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_propagate_forwards` is an independent external_body axiom transferring `always(p)` to `ex.suffix(i)`, which involves an intensionally distinct closure. This is a separate trust assumption that bridges the always closure across suffix operations.

