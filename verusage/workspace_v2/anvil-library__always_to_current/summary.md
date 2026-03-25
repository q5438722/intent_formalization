# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_to_current.rs`
**Date:** 2026-03-24T03:51:09Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `execution_equality` forces intensional spec_fn equality from extensional equivalence, the sole unverified external_body axiom in this file. The remaining four are false positives — suffix(0) identity, always-to-current extraction, arbitrary suffix extraction, and suffix composition are all semantically correct properties that merely instantiate the root axiom on desirable cases.

## True Positives (Spec Issues)

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption in the file.

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body forcing intensional equality of spec_fn from pointwise extensional equivalence — two structurally distinct closures are collapsed to equal
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption in the file.

### φ2: suffix_zero_equals_self
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** ex.suffix(0) creates closure |i| f(i+0) intensionally distinct from ex — execution_equality silently equates these without proof
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ex.suffix(0) == ex` is semantically correct and desirable. It's a concrete instantiation of φ1's axiom on a trivially correct property — no new trust surface.

### φ3: always_to_current_no_axiom_needed
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_to_current extracts p from always(p) by routing through execution_equality at suffix(0) — if this verifies, the external_body axiom is the sole trust anchor for a fundamental temporal logic property
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Extracting `p` from `always(p)` is a fundamental and desirable temporal logic property. The fact that it routes through `execution_equality` is an implementation detail; the property itself is expected.

### φ4: always_at_arbitrary_suffix
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** always(p).satisfied_by(ex) embeds forall|i| p.satisfied_by(ex.suffix(i)) inside a TempPred::new closure — extracting p at arbitrary suffix i requires bridging the intensional gap between the closure's forall and a bare forall
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The proof body `execution_equality(ex.suffix(i), ex.suffix(i))` is a trivial self-equality that contributes nothing — the real work is the SMT solver extracting the `forall` from `always`'s closure. If this verifies, it shows the SMT resolves the intensional gap without needing an external_body axiom for this direction, which is expected behavior.

### φ5: suffix_composition_equals_addition
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** doubly-nested suffix creates a closure intensionally distinct from single-offset suffix — execution_equality collapses them without proof
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Suffix composition is a semantically correct and desirable property. The intensional concern is entirely subsumed by φ1's identification of `execution_equality` as the root axiom.

