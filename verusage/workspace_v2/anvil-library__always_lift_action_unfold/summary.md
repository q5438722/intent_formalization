# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_lift_action_unfold.rs`
**Date:** 2026-03-24T03:42:22Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive identified: `always_unfold` is an unverified external_body axiom bridging an intensional closure gap. The other two candidates are false positives — suffix/head arithmetic equality is correctly resolved by SMT, and constant executions satisfying constant-preserving action predicates is standard temporal logic behavior.

## True Positives (Spec Issues)

### always_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** The external_body `always_unfold` bridges the intensional gap between the `forall` quantifier embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures. This is an unverified trust assumption that the SMT solver cannot check through the closure boundary.

## All Candidates

### φ1: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging the forall inside always's TempPred::new closure and a bare forall in the ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body `always_unfold` bridges the intensional gap between the `forall` quantifier embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures. This is an unverified trust assumption that the SMT solver cannot check through the closure boundary.

### φ2: lift_action_ignores_future_states
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** lift_action only relates consecutive states (head, head_next) — always(lift_action(p)) says nothing about non-consecutive state pairs, so safety properties spanning more than one step could be silently missed

### φ3: suffix_head_next_vs_suffix_plus_one_head
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** ex.suffix(i).head_next() evaluates via a nested closure |j| f(j+i) at j=1 while ex.suffix(i+1).head() evaluates via |j| f(j+(i+1)) at j=0 — if Verus proves these equal it confirms the SMT solver resolves the intensional closure gap, but if it doesn't, always_lift_action_unfold's ensures clause is weaker than expected
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Both sides reduce to `(ex.nat_to_state)(i + 1)` after unfolding: `ex.suffix(i).head_next()` = `(|j| f(j+i))(1)` = `f(1+i)` and `ex.suffix(i+1).head()` = `(|j| f(j+(i+1)))(0)` = `f(0+(i+1))` = `f(i+1)`. The SMT solver resolves this arithmetic equality directly — this is a desirable and expected property.

### φ4: always_lift_action_consecutive_chain
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** the second conjunct requires ex.suffix(1).head_next() == ex.suffix(2).head() through nested closures — if this verifies, the external_body axiom is strong enough to resolve multi-level suffix composition without an explicit composition lemma

### φ5: always_lift_action_vacuous_on_constant
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** a constant execution trivially satisfies always(lift_action(p)) for any p that holds on (v,v) — the spec framework cannot distinguish a genuinely invariant system from a degenerate constant trace, potentially masking liveness issues
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A constant execution satisfying `always(lift_action(|s1,s2| s1==v && s2==v))` is correct and expected — the action predicate genuinely holds at every step. This is standard TLA behavior; liveness concerns are orthogonal to safety/action specifications.

