# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/not_eventually_by_always_not.rs`
**Date:** 2026-03-24T04:15:42Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `always_unfold` is the sole external_body axiom bridging the intensional closure gap between `always`'s `TempPred::new`-wrapped forall and a bare forall. The remaining four are false positives — all are semantically correct temporal logic consequences (forall-not implies not-exists, universal instantiation, substitution, and contradiction from inconsistent premises) with no new trust surface beyond the root axiom.

## True Positives (Spec Issues)

### always_unfold_intensional_bridge
- **Confidence:** high
- **Reasoning:** `always_unfold` is external_body bridging `always(p).satisfied_by(ex)` — which evaluates a `TempPred::new` closure containing a forall — to a bare forall over `p.satisfied_by(ex.suffix(i))`. Unverified trust assumption resolving the intensional closure gap.

## All Candidates

### φ1: always_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging always's TempPred::new closure-wrapped forall to a bare forall in ensures — intensionally distinct formulations equated without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `always_unfold` is external_body bridging `always(p).satisfied_by(ex)` — which evaluates a `TempPred::new` closure containing a forall — to a bare forall over `p.satisfied_by(ex.suffix(i))`. Unverified trust assumption resolving the intensional closure gap.

### φ2: not_eventually_from_always_not
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** not_eventually_by_always_not bridges always(not(p)) to not(eventually(p)) — these are intensionally distinct TempPred closures (forall-of-not vs not-of-exists) equated through always_unfold without verifying closure identity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `always(¬p) ⟹ ¬eventually(p)` is a standard temporal logic tautology (∀i.¬p(i) ⟹ ¬∃i.p(i)). This is a correct consequence of `always_unfold` — no new trust surface beyond φ1.

### φ3: always_not_blocks_any_witness
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** always_unfold allows extracting negation at any specific witness index — if the closure bridge is unsound, this could incorrectly block valid witnesses
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Extracting `¬p` at a specific index from `always(¬p)` is a direct instantiation of φ1 — standard universal elimination. No new trust surface.

### φ4: double_not_eventually_collapse
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** applying always(not(not(p))) => not(eventually(not(p))) chains two levels of not-closure wrapping — if closure identity resolution is unsound, double negation could produce incorrect temporal conclusions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `always(¬(¬p)) ⟹ ¬eventually(¬p)` is a correct instantiation of φ2 with `not(p)` substituted for `p`. Standard temporal logic, no new trust surface.

### φ5: always_not_self_contradicts_eventually
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** combining always(not(p)) and eventually(p) derives false — if either the always_unfold bridge or the not/eventually closure resolution is unsound, this contradiction could be reached from consistent premises
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `always(¬p) ∧ eventually(p) ⟹ false` is a standard temporal logic tautology — these premises are genuinely contradictory. This is expected behavior, not a spec gap.

