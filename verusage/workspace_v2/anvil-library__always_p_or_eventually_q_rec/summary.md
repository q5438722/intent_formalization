# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_p_or_eventually_q_rec.rs`
**Date:** 2026-03-24T03:49:01Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `execution_equality` forces intensional spec_fn equality from extensional equivalence, the sole unverified external_body axiom in this file. The remaining four candidates are false positives — suffix(0) identity, suffix composition, the inductive step check, and the base case dependency are all semantically correct properties or redundant instances of the same root axiom concern.

## True Positives (Spec Issues)

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption underlying the entire file.

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body forcing intensional equality of spec_fn from pointwise extensional equivalence — two structurally distinct closures are collapsed to equal
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption underlying the entire file.

### φ2: suffix_zero_equals_self
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** ex.suffix(0) creates closure |i| f(i+0) intensionally distinct from ex — execution_equality silently equates these without proof
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ex.suffix(0) == ex` is semantically correct and desirable. It's a concrete instantiation of φ1's axiom on a trivially correct property — no new trust surface.

### φ3: suffix_composition_equals_addition
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** doubly-nested suffix creates a closure intensionally distinct from single-offset suffix — execution_equality collapses them without proof, and the recursive proof depends on this at the base case
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Suffix composition is a semantically correct and expected property. The intensional concern is entirely subsumed by φ1's identification of `execution_equality` as the root axiom.

### φ4: rec_induction_missing_step_check
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** the inductive step relies on suffix(i-1) stepping to suffix(i) but the proof never explicitly invokes the step precondition — the SMT solver must silently resolve that p.satisfied_by(ex.suffix(i-1)) + next + !q implies p.satisfied_by(ex.suffix(i)) through the intensional suffix gap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is just two calls to `always_p_or_eventually_q_rec` at consecutive indices. The inductive step in the original proof is correctly handled by the SMT solver using the preconditions — the step precondition is available and instantiated automatically. No new issue beyond what's already in the verified proof.

### φ5: rec_base_case_depends_on_execution_equality
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** the base case i=0 of the induction transfers p from ex to ex.suffix(0) solely through execution_equality — without this external_body axiom, the base case would fail due to intensional closure mismatch
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a specific instance of φ2 (suffix_zero_equals_self) applied to transfer `p` across the `ex == ex.suffix(0)` equality. The dependency on `execution_equality` is real but already captured by φ1 — no new trust surface.

