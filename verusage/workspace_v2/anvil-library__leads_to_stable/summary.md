# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_stable.rs`
**Date:** 2026-03-24T04:13:06Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 5
- False positives: 0

## Summary

Five true positives: `leads_to_unfold`, `implies_apply`, `next_preserves_inv_rec`, `execution_equality`, and `always_propagate_forwards` are independent external_body axioms each bridging distinct intensional closure gaps — always-forall extraction, TempPred modus ponens, inductive suffix-chain preservation, spec_fn extensional-to-intensional equality, and always-suffix propagation, respectively. This file has the highest density of external_body axioms seen so far, with `leads_to_stable` depending on all six of them.

## True Positives (Spec Issues)

### leads_to_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s definition — `always(p.implies(eventually(q)))` with nested `TempPred::new` closures — to a bare forall in the ensures. Unverified trust assumption resolving intensional closure gap.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh `TempPred::new` closure — to the direct modus ponens conclusion. Separate unverified trust assumption.

### next_preserves_inv_rec_intensional_bridge
- **Confidence:** high
- **Reasoning:** `next_preserves_inv_rec` is external_body performing induction over suffix chains, requiring suffix composition resolution (`ex.suffix(idx).suffix(1) == ex.suffix(idx+1)`) at each step. Combines induction with intensional closure gap resolution in a single unverified assumption.

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. Structurally distinct closures collapsed to equal without proof — the root unverified trust assumption.

### always_propagate_forwards_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_propagate_forwards` is external_body shifting `always(p)` to a suffix, requiring that `forall |j| p.satisfied_by(ex.suffix(i).suffix(j))` follows from `forall |j| p.satisfied_by(ex.suffix(j))` — bridges the suffix composition intensional gap without proof.

## All Candidates

### φ1: leads_to_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_unfold is external_body bridging the forall inside always's TempPred::new closure to a bare forall in the ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s definition — `always(p.implies(eventually(q)))` with nested `TempPred::new` closures — to a bare forall in the ensures. Unverified trust assumption resolving intensional closure gap.

### φ2: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping — resolves closure identity mismatch without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh `TempPred::new` closure — to the direct modus ponens conclusion. Separate unverified trust assumption.

### φ3: next_preserves_inv_rec_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** next_preserves_inv_rec is external_body performing induction on suffix chains — bridges intensional gaps in suffix composition and closure evaluation at each inductive step without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `next_preserves_inv_rec` is external_body performing induction over suffix chains, requiring suffix composition resolution (`ex.suffix(idx).suffix(1) == ex.suffix(idx+1)`) at each step. Combines induction with intensional closure gap resolution in a single unverified assumption.

### φ4: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality forces intensional equality of spec_fn from pointwise extensional equivalence — structurally distinct closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. Structurally distinct closures collapsed to equal without proof — the root unverified trust assumption.

### φ5: always_propagate_forwards_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_propagate_forwards is external_body shifting always(p) to a suffix — requires resolving that always's forall over ex.suffix(i).suffix(j) equals forall over ex.suffix(i+j), bridging intensional closure gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_propagate_forwards` is external_body shifting `always(p)` to a suffix, requiring that `forall |j| p.satisfied_by(ex.suffix(i).suffix(j))` follows from `forall |j| p.satisfied_by(ex.suffix(j))` — bridges the suffix composition intensional gap without proof.

