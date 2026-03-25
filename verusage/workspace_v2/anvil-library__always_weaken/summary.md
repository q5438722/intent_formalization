# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_weaken.rs`
**Date:** 2026-03-24T03:52:25Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives: `implies_apply` and `entails_preserved_by_always` are independent external_body axioms each bridging distinct intensional closure gaps — modus ponens through TempPred::new wrapping, and lifting entailment through always's forall-over-suffix closure, respectively. The remaining three are false positives: φ1 likely doesn't verify without `execution_equality` in this file, φ4 is standard ex falso, and φ5 restates the verified lemma itself.

## True Positives (Spec Issues)

### implies_apply_closure_bridge
- **Confidence:** high
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is an unverified trust assumption resolving the intensional closure gap.

### entails_preserved_by_always_intensional_bridge
- **Confidence:** high
- **Reasoning:** `entails_preserved_by_always` is an independent external_body axiom lifting entailment through `always`, bridging `p.entails(q)` into the forall-over-suffix closure inside `always`. This is a separate unverified trust assumption from `implies_apply`.

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** if the SMT solver can derive intensional spec_fn equality from pointwise equivalence through Execution wrapping, it would collapse the extensional/intensional distinction without any external_body axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This file does not contain `execution_equality` as an external_body axiom. Without it, the SMT solver cannot derive `ex1 == ex2` from pointwise equality of closures — Verus maintains the intensional/extensional distinction. This φ almost certainly does not verify against this source file.

### φ2: implies_apply_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** implies_apply is external_body bridging p.implies(q).satisfied_by(ex) through TempPred::new closure wrapping and direct implication — silently resolves closure identity mismatch
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `implies_apply` is external_body bridging `p.implies(q).satisfied_by(ex)` — which evaluates a fresh closure from `TempPred::new` — to the direct modus ponens conclusion. This is an unverified trust assumption resolving the intensional closure gap.

### φ3: entails_preserved_by_always_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** entails_preserved_by_always is external_body lifting entailment through always — bridges the forall-over-suffix closure inside always with the entailment closure without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `entails_preserved_by_always` is an independent external_body axiom lifting entailment through `always`, bridging `p.entails(q)` into the forall-over-suffix closure inside `always`. This is a separate unverified trust assumption from `implies_apply`.

### φ4: always_weaken_vacuous_spec
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously entails always(q) for any q — always_weaken propagates this without checking spec satisfiability, potentially hiding specification errors
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A false antecedent entailing anything is standard logic (ex falso quodlibet). This is not a spec gap — it's a fundamental property of implication that holds in every entailment framework.

### φ5: always_weaken_strengthens_to_valid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_weaken composes two external_body axioms (implies_apply + entails_preserved_by_always) to derive always(q) from always(p) under a pointwise implication — the composition amplifies the unverified trust surface across both axioms
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is literally the statement of `always_weaken` itself — a semantically correct and desirable theorem that `always` is monotone under pointwise implication. No new trust surface beyond the already-identified axioms.

