# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_p_or_eventually_q.rs`
**Date:** 2026-03-24T03:46:16Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives identified. φ1 is the most fundamental: `execution_equality` forces intensional spec_fn equality from extensional equivalence. φ2 reveals that `not_eventually_unfold` enables reverse packing into `always(not(p))`, a separate intensional bridge. φ3 flags `always_p_or_eventually_q_rec` as a trusted induction axiom with no verified base case or step. φ4 and φ5 are false positives — suffix composition is a correct property subsumed by φ1, and double-suffix access is a straightforward composition of known axioms with no new trust surface.

## True Positives (Spec Issues)

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the foundational unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

### not_eventually_implies_always_not
- **Confidence:** medium
- **Reasoning:** `not_eventually_unfold` extracts a bare `forall |i| !p.satisfied_by(ex.suffix(i))`, and the SMT solver repacks this into `always(not(p))` by closing the intensional gap between the `always` closure and the bare quantifier. This is a separate unverified bridge from `always_unfold` — it works in the reverse direction (packing rather than unpacking).

### always_p_or_eventually_q_rec_no_bound
- **Confidence:** medium
- **Reasoning:** `always_p_or_eventually_q_rec` is external_body with a `decreases i` clause but the induction (base case and step) is entirely unverified. The axiom trusts that the preconditions suffice for induction without any checked proof, making it a separate trust assumption beyond the intensional bridge axioms.

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body forcing intensional equality of spec_fn from pointwise extensional equivalence — two structurally distinct closures are collapsed to equal
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the foundational unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

### φ2: not_eventually_implies_always_not
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** not_eventually_unfold extracts a bare forall of negations, and repacking into always(not(p)) requires the SMT solver to close the intensional gap between the always closure and the bare quantifier — if this verifies, the axiom is strong enough to enable duality rewrites
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `not_eventually_unfold` extracts a bare `forall |i| !p.satisfied_by(ex.suffix(i))`, and the SMT solver repacks this into `always(not(p))` by closing the intensional gap between the `always` closure and the bare quantifier. This is a separate unverified bridge from `always_unfold` — it works in the reverse direction (packing rather than unpacking).

### φ3: always_p_or_eventually_q_rec_no_bound
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** always_p_or_eventually_q_rec is external_body with a decreases clause on i but no base case verification — the induction is trusted without proof, and composing it over all i yields unbounded invariance from a single-step induction axiom
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `always_p_or_eventually_q_rec` is external_body with a `decreases i` clause but the induction (base case and step) is entirely unverified. The axiom trusts that the preconditions suffice for induction without any checked proof, making it a separate trust assumption beyond the intensional bridge axioms.

### φ4: suffix_composition_via_execution_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** ex.suffix(i).suffix(j) produces a doubly-nested closure intensionally distinct from ex.suffix(i+j) — execution_equality collapses them without proof, and always_p_or_eventually_q depends on this at every inductive step
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Suffix composition `ex.suffix(i).suffix(j) == ex.suffix(i + j)` is a semantically correct and desirable property. The intensional concern is entirely subsumed by φ1's identification of `execution_equality` as the root unverified axiom — this is just a concrete instantiation.

### φ5: always_propagate_then_unfold_double_suffix
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chaining always_propagate_forwards with always_unfold yields p on doubly-nested suffix ex.suffix(i).suffix(j) which is intensionally distinct from ex.suffix(i+j) — the two external_body axioms compose to resolve arbitrary suffix nesting without an explicit composition lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This composes two already-identified axioms (`always_propagate_forwards` and `always_unfold`) on a double suffix. The result is semantically correct — `always(p)` should indeed yield `p` at any reachable suffix. No new trust surface is exposed beyond the individual axioms.

