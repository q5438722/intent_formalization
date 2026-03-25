# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/or_leads_to_combine.rs`
**Date:** 2026-03-24T04:16:22Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives: `leads_to_unfold` and `implies_apply` are independent external_body axioms bridging distinct intensional closure gaps — always-forall extraction and TempPred modus ponens, respectively. The remaining two are false positives: ex falso from unsatisfiable spec and reflexive disjunction leads_to are both standard logical consequences.

## True Positives (Spec Issues)

### leads_to_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s definition — `always(p.implies(eventually(q)))` with nested `TempPred::new` closures — to a bare forall in the ensures. Unverified trust assumption resolving the intensional closure gap.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh `TempPred::new` closure — to the direct modus ponens conclusion. Separate unverified trust assumption.

## All Candidates

### φ1: leads_to_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_unfold is external_body bridging the forall inside always's TempPred::new closure to a bare forall in the ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s definition — `always(p.implies(eventually(q)))` with nested `TempPred::new` closures — to a bare forall in the ensures. Unverified trust assumption resolving the intensional closure gap.

### φ2: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping — resolves closure identity mismatch without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh `TempPred::new` closure — to the direct modus ponens conclusion. Separate unverified trust assumption.

### φ3: or_leads_to_combine_false_spec
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously entails any leads_to — the lemma propagates vacuous truth without detecting spec inconsistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An unsatisfiable spec vacuously entailing anything is standard ex falso quodlibet. This is not a spec gap.

### φ4: or_leads_to_self
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** or_leads_to_combine can derive reflexive leads_to for any disjunction given component leads_to premises — if the component premises are themselves derived from unsound axioms this amplifies the unsoundness
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Reflexive leads_to for a disjunction given component leads_to premises is a correct temporal logic consequence. No new trust surface beyond φ1 and φ2.

### φ5: leads_to_unfold_at_zero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** combining leads_to_unfold at suffix(0) with implies_apply extracts eventually(q) from p at the current state — chains two external_body axioms to derive a temporal liveness conclusion

