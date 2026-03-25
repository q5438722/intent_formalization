# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/pack_conditions_to_spec.rs`
**Date:** 2026-03-24T04:18:19Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives: `leads_to_unfold`, `implies_apply`, and `always_propagate_forwards` are independent external_body axioms each bridging distinct intensional closure gaps — always-forall extraction, TempPred modus ponens, and always-suffix propagation, respectively. Two false positives: condition packing is a correct temporal logic transformation, and false-spec entailment is ex falso.

## True Positives (Spec Issues)

### leads_to_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s definition — `always(p.implies(eventually(q)))` with nested `TempPred::new` closures — to a bare forall in the ensures. Unverified trust assumption resolving the intensional closure gap.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh `TempPred::new` closure — to the direct modus ponens conclusion. Separate unverified trust assumption.

### always_propagate_forwards_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_propagate_forwards` is external_body shifting `always(p)` to a suffix, requiring that `forall |j| p.satisfied_by(ex.suffix(i).suffix(j))` follows from `forall |j| p.satisfied_by(ex.suffix(j))` — bridges the suffix composition intensional gap without proof.

## All Candidates

### φ1: leads_to_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** leads_to_unfold is external_body bridging always's TempPred::new closure-wrapped forall to a bare forall in ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `leads_to_unfold` is external_body bridging `leads_to`'s definition — `always(p.implies(eventually(q)))` with nested `TempPred::new` closures — to a bare forall in the ensures. Unverified trust assumption resolving the intensional closure gap.

### φ2: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping — resolves closure identity mismatch without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh `TempPred::new` closure — to the direct modus ponens conclusion. Separate unverified trust assumption.

### φ3: always_propagate_forwards_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_propagate_forwards is external_body shifting always(p) to a suffix — requires resolving that forall over ex.suffix(i).suffix(j) equals forall over ex.suffix(i+j) through intensional closure gaps
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_propagate_forwards` is external_body shifting `always(p)` to a suffix, requiring that `forall |j| p.satisfied_by(ex.suffix(i).suffix(j))` follows from `forall |j| p.satisfied_by(ex.suffix(j))` — bridges the suffix composition intensional gap without proof.

### φ4: pack_conditions_weakens_spec
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** pack_conditions_to_spec strengthens the spec with always(c) to remove c from the leads_to antecedent — if always(c) is unsatisfiable this vacuously proves any leads_to, silently discarding the condition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Moving a condition `c` from the leads_to antecedent into `always(c)` on the spec side is a correct and standard temporal logic transformation. The spec strengthening is the intended behavior, not a gap.

### φ5: false_spec_any_leads_to
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously satisfies all entailment premises — pack_conditions_to_spec propagates vacuous truth producing arbitrary leads_to conclusions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An unsatisfiable spec vacuously entailing anything is standard ex falso quodlibet. This is not a spec gap.

