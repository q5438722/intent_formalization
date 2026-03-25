# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_p_is_stable.rs`
**Date:** 2026-03-24T03:45:39Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `always_propagate_forwards` is an unverified external_body axiom bridging an intensional closure gap when transferring `always(p)` to suffixed executions. The remaining three are false positives — vacuous stability on false predicates/antecedents is standard TLA semantics, and the suffix(0) case is a redundant instance of the same axiom concern.

## True Positives (Spec Issues)

### always_propagate_forwards_intensional_bridge
- **Confidence:** high
- **Reasoning:** The external_body `always_propagate_forwards` bridges the intensional gap between the `forall` inside `always`'s closure on `ex` and the same on `ex.suffix(i)`, which involves nested closures that are intensionally distinct. This is an unverified trust assumption.

## All Candidates

### φ1: always_propagate_forwards_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_propagate_forwards is external_body asserting always(p) transfers to ex.suffix(i), but ex.suffix(i) creates a new closure intensionally distinct from ex — the axiom silently equates these without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body `always_propagate_forwards` bridges the intensional gap between the `forall` inside `always`'s closure on `ex` and the same on `ex.suffix(i)`, which involves nested closures that are intensionally distinct. This is an unverified trust assumption.

### φ2: stable_always_vacuously_true_when_not_satisfied
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** stable(always(p)) is defined as always(p) ==> always(always(p)), so when always(p) is false the implication is vacuously true — stable claims hold for every execution regardless of whether the predicate actually holds
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the standard definition of stability in TLA — `p ==> always(p)` is vacuously true when `p` is false. This is expected and desirable; stability is only meaningful when the predicate holds.

### φ3: valid_stable_says_nothing_about_p
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** valid(stable(always(p))) does not guarantee always(p) holds on any execution — it only guarantees the implication, so it provides no actual safety assurance about p

### φ4: suffix_zero_intensional_gap
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** ex.suffix(0) creates closure |i| f(i+0) intensionally distinct from ex — the external_body axiom equates always(p) on these without proof, and this is the i=0 base case showing the axiom is needed even for trivial shifts
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is just a concrete instance of φ1 at i=0. It identifies no new trust surface beyond what `always_propagate_forwards` already provides — it's a specific instantiation, not a distinct issue.

### φ5: stable_any_pred_vacuous
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** stable(false_pred) is vacuously satisfied on every execution since the antecedent of the implication is always false — stable provides no guarantee for predicates that never hold, making it a weak specification pattern
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `stable(false)` being valid is standard TLA behavior — an always-false predicate is trivially stable because the implication's antecedent never holds. This is expected, not a spec gap.

