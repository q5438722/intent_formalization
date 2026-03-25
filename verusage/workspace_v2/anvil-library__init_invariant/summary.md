# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/init_invariant.rs`
**Date:** 2026-03-24T04:01:19Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Three true positives: `init_invariant_rec`, `always_unfold`, and `implies_apply` are independent external_body axioms each bridging distinct intensional closure gaps — induction over suffix-indexed predicates, extracting a forall from always's closure wrapper, and modus ponens through TempPred::new wrapping, respectively. The remaining one is a false positive: deriving always(false) from an unsatisfiable init is standard ex falso.

## True Positives (Spec Issues)

### init_invariant_rec_intensional_bridge
- **Confidence:** high
- **Reasoning:** `init_invariant_rec` is external_body performing induction over suffix positions with `decreases i` but no verified proof body. This is an unverified trust assumption — the inductive reasoning over suffix-indexed predicates is assumed correct without SMT verification.

### always_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_unfold` is external_body bridging the forall embedded inside `always`'s `TempPred::new` closure to a bare forall in the ensures. This is an unverified trust assumption resolving the intensional closure gap.

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is an unverified trust assumption resolving the intensional closure gap.

## All Candidates

### φ1: init_invariant_rec_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** init_invariant_rec is external_body performing induction over suffix positions — it bridges the intensional gap between suffix-indexed state predicates and the inductive hypothesis without verified decreases reasoning
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `init_invariant_rec` is external_body performing induction over suffix positions with `decreases i` but no verified proof body. This is an unverified trust assumption — the inductive reasoning over suffix-indexed predicates is assumed correct without SMT verification.

### φ2: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging the forall inside always's TempPred::new closure to a bare forall in the ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_unfold` is external_body bridging the forall embedded inside `always`'s `TempPred::new` closure to a bare forall in the ensures. This is an unverified trust assumption resolving the intensional closure gap.

### φ3: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping and direct implication — silently resolves closure identity mismatch
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is an unverified trust assumption resolving the intensional closure gap.

### φ4: invariant_from_false_init
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** an unsatisfiable init predicate makes init_invariant prove any invariant for any spec — the vacuously-true premises propagate through without checking satisfiability of init

### φ5: invariant_ignores_next_action
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** if init implies false (no valid initial state), then init_invariant derives always(false) for any spec claiming to start from init — the spec is vacuously inconsistent but this is never flagged
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** If init implies false, then `spec.entails(lift_state(init))` means spec is unsatisfiable, so spec vacuously entails anything. This is standard ex falso quodlibet, not a spec gap.

