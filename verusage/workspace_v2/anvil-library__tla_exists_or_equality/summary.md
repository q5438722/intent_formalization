# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_exists_or_equality.rs`
**Date:** 2026-03-24T04:27:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Two true positives from independent external_body axioms: `tla_exists_proved_by_witness` (existential introduction through closure gap) and `temp_pred_equality` (intensional equality from semantic equivalence), plus a third demonstrating the soundness consequence of deriving false from distinct-but-equivalent preds. Two false positives: the exists-or distribution is a correct logical equivalence, and the arbitrary-witness case is sound existential introduction when the disjunct holds unconditionally.

## True Positives (Spec Issues)

### tla_exists_proved_by_witness_closure_bridge
- **Confidence:** high
- **Reasoning:** `tla_exists_proved_by_witness` is external_body bridging `a_to_p(witness).satisfied_by(ex)` to `tla_exists(a_to_p).satisfied_by(ex)` — requires resolving that `tla_exists`'s `TempPred::new` closure-wrapped existential evaluates consistently with the witness. Unverified trust assumption.

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — separate unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — derives `false` from intensionally distinct but semantically equivalent preds.

## All Candidates

### φ1: tla_exists_proved_by_witness_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_exists_proved_by_witness is external_body bridging a_to_p(witness).satisfied_by(ex) to tla_exists's TempPred::new closure-wrapped existential — resolves closure identity gap without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_exists_proved_by_witness` is external_body bridging `a_to_p(witness).satisfied_by(ex)` to `tla_exists(a_to_p).satisfied_by(ex)` — requires resolving that `tla_exists`'s `TempPred::new` closure-wrapped existential evaluates consistently with the witness. Unverified trust assumption.

### φ2: temp_pred_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body forcing intensional equality from mutual entailment — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — separate unverified trust assumption.

### φ3: mutual_entailment_derives_false_if_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality forces p == q from mutual entailment — if two preds are intensionally distinct but semantically equivalent this derives false
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — derives `false` from intensionally distinct but semantically equivalent preds.

### φ4: tla_exists_or_equality_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_exists_or_equality forces intensional equality between tla_exists(|a| p(a).or(q)) and tla_exists(p).or(q) — structurally distinct nested closures collapsed via both external_body axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `∃a. (p(a) ∨ q) ≡ (∃a. p(a)) ∨ q` is a standard logical equivalence (q doesn't depend on a). This is a verified lemma with no new trust surface beyond φ1 and φ2.

### φ5: arbitrary_witness_in_else_branch
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** when q holds but no witness for a_to_p exists, tla_exists_or_equality still guarantees tla_exists(|a| p(a).or(q)) — the proof uses arbitrary() as witness, relying on the or-branch being true regardless of which a is chosen, but the existential introduction through the closure bridge is unverified
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** When `q` holds, `a_to_p(arbitrary()).or(q)` is true for any witness since the `or`-branch is satisfied. This is semantically correct existential introduction — `∃a. (p(a) ∨ q)` holds when `q` holds. No new trust surface.

