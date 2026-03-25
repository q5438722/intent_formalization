# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/eventually_propagate_backwards.rs`
**Date:** 2026-03-24T03:59:45Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 4

## Summary

One true positive: `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence, the root unverified trust assumption. The remaining four are false positives — suffix(0) identity, suffix composition, and both eventually introduction variants are semantically correct properties that represent intended uses of the axioms or direct SMT reasoning.

## True Positives (Spec Issues)

### execution_equality_forces_fn_equality
- **Confidence:** high
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality forces intensional equality of spec_fn from pointwise extensional equivalence — two structurally distinct closures are collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `execution_equality` forces intensional `spec_fn` equality from pointwise extensional equivalence. This is the root unverified trust assumption — structurally distinct closures are collapsed to equal without proof.

### φ2: suffix_zero_equals_self
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** suffix(0) creates a fresh closure |i| f(i+0) which is intensionally distinct from f — execution_equality silently bridges this without the SMT solver proving closure identity
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ex.suffix(0) == ex` is semantically correct since `f(i+0) == f(i)` for all `i`. This is a desirable and expected property — the intensional bridge via `execution_equality` is legitimate use of that axiom.

### φ3: suffix_composition_equals_addition
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** suffix(a).suffix(b) nests two closure wrappers while suffix(a+b) has one — execution_equality collapses the intensionally distinct nested closures to equal
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ex.suffix(a).suffix(b) == ex.suffix(a+b)` is semantically correct since `f((i+b)+a) == f(i+(a+b))` by commutativity/associativity of addition. This is a desirable property and the intended use case of `execution_equality`.

### φ4: eventually_unfold_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** eventually_unfold is external_body extracting the existential from eventually's TempPred::new closure — this bridges the intensional gap, and the proof should fail since the requires don't establish eventually(p).satisfied_by(ex)
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures `eventually(p).satisfied_by(ex)` can be proven directly by the SMT solver witnessing the existential with `i` from the requires — it doesn't actually depend on `eventually_unfold` (which has the wrong direction anyway). This is introduction of `eventually` from a witness, which is expected and correct.

### φ5: eventually_propagate_to_zero
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** combining execution_equality to equate suffix(0) with self and suffix(0).suffix(i) with suffix(i) lets us inject witness i into eventually's closure for any execution — the external_body axiom does all the real work
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This demonstrates that `p.satisfied_by(ex.suffix(i))` implies `eventually(p).satisfied_by(ex)`, which is the standard introduction rule for eventually — given a witness at suffix `i`, the existential is satisfied. This is correct and expected.

