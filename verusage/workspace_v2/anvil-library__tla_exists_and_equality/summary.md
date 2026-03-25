# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_exists_and_equality.rs`
**Date:** 2026-03-24T04:25:25Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Two true positives from independent external_body axioms: `tla_exists_proved_by_witness` (existential introduction through closure gap) and `temp_pred_equality` (intensional equality from semantic equivalence), plus a third demonstrating the soundness consequence of deriving false from distinct-but-equivalent preds. Two false positives: the exists-and distribution is a correct logical equivalence, and choose-witness extraction is sound SMT reasoning.

## True Positives (Spec Issues)

### tla_exists_proved_by_witness_closure_bridge
- **Confidence:** high
- **Reasoning:** `tla_exists_proved_by_witness` is external_body bridging `a_to_p(witness).satisfied_by(ex)` to `tla_exists(a_to_p).satisfied_by(ex)` — requires resolving that `tla_exists`'s `TempPred::new` closure-wrapped existential evaluates consistently with the witness. Unverified trust assumption.

### temp_pred_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `temp_pred_equality` is external_body forcing intensional `==` from mutual entailment. Structurally distinct closures collapsed to equal without proof — separate unverified trust assumption.

### mutual_entailment_derives_false_if_distinct
- **Confidence:** high
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — if two TempPreds are intensionally distinct but semantically equivalent, this derives `false`.

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
- **Reasoning:** Direct soundness consequence of `temp_pred_equality` — if two TempPreds are intensionally distinct but semantically equivalent, this derives `false`.

### φ4: tla_exists_and_equality_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_exists_and_equality forces intensional equality between tla_exists(|a| p(a).and(q)) and tla_exists(a_to_p).and(q) — structurally distinct nested closures collapsed via the chain of both external_body axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `∃a. p(a) ∧ q ≡ (∃a. p(a)) ∧ q` is a standard logical equivalence (q doesn't depend on a). This is a verified lemma with no new trust surface beyond φ1 and φ2.

### φ5: tla_exists_choose_witness_soundness
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** tla_exists_choose_witness uses choose on the raw existential — but tla_exists wraps the existential in a TempPred::new closure, so extracting a witness from tla_exists.satisfied_by requires the closure bridge to access the inner existential
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Extracting a witness from `tla_exists` via `choose` is sound — `tla_exists`'s definition directly contains the existential, and SMT can resolve this without external_body axioms. No new trust surface.

