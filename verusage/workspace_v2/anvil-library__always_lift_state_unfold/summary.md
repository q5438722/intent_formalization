# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_lift_state_unfold.rs`
**Date:** 2026-03-24T03:45:02Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive: `always_unfold` is an unverified external_body axiom bridging an intensional closure gap. The constant-execution property is a false positive — it's standard temporal logic behavior.

## True Positives (Spec Issues)

### always_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** The external_body `always_unfold` bridges the intensional gap between the `forall` embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures. This is an unverified trust assumption.

## All Candidates

### φ1: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging the forall inside always's TempPred::new closure and a bare forall in the ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body `always_unfold` bridges the intensional gap between the `forall` embedded inside `always`'s `TempPred::new` closure and a bare `forall` in the ensures. This is an unverified trust assumption.

### φ2: lift_state_suffix_head_equivalence
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** ex.suffix(i).head() evaluates via a nested closure |j| f(j+i) at j=0, which should equal f(i) but requires the SMT solver to resolve through the closure — if this verifies, the axiom chain silently resolves the intensional gap

### φ3: always_lift_state_at_zero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** accessing suffix(0).head() should equal ex.head() but suffix(0) creates a new closure |i| f(i+0) intensionally distinct from ex — the external_body axiom silently resolves this without proof

### φ4: always_preserved_across_suffix
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** always(lift_state(p)) on ex should imply the same on ex.suffix(k), but proving this requires re-packing quantified suffix facts into the always closure on a shifted execution — if always_unfold is powerful enough to close this, it may be overly strong

### φ5: always_lift_state_vacuous_on_false
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** a constant execution trivially satisfies always(lift_state(p)) for any p that holds on the constant value — the framework cannot distinguish a genuinely invariant system from a degenerate constant trace
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A constant execution satisfying `always(lift_state(|s| s == v))` is correct and expected — the predicate genuinely holds at every step. Distinguishing "real" invariance from constant traces is not the spec framework's responsibility.

