# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_and_equality.rs`
**Date:** 2026-03-24T03:37:11Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Two significant true positives were found. The most critical is φ5 (`valid_false_from_external_body_chain`), which shows that `temp_pred_equality` can derive `false` when given two intensionally distinct but extensionally equivalent TempPreds — a direct soundness gap. φ1 identifies the root cause: the external_body axiom conflates extensional and intensional equality of `spec_fn`. φ2 flags a similar but lower-severity trust assumption in `always_unfold`. φ3 is a benign derived consequence that adds no new risk beyond the axioms it depends on.

## True Positives (Spec Issues)

### temp_pred_equality_collapses_extensional
- **Confidence:** high
- **Reasoning:** The external_body axiom forces `p == q` (and hence `p.pred == q.pred`) from mutual entailment alone. This asserts intensional equality of `spec_fn` from extensional equivalence — a genuinely unverified assumption that could be unsound if Verus's SMT encoding ever distinguishes closures that agree on all inputs.

### always_unfold_suffix_equivalence
- **Confidence:** medium
- **Reasoning:** The `always` definition wraps a `forall` inside a new closure via `TempPred::new`, producing a different `spec_fn` than a bare quantifier over `p.satisfied_by(ex.suffix(i))`. The external_body `always_unfold` bridges this intensional gap without proof. While semantically correct, it is an unverified trust assumption.

### valid_false_from_external_body_chain
- **Confidence:** high
- **Reasoning:** This is the sharpest finding. The precondition `p != q` is satisfiable for extensionally equivalent but intensionally distinct TempPreds (e.g., two `TempPred::new` calls with different but logically equivalent closures). Combined with `temp_pred_equality` forcing `p == q`, this derives `false` — a direct soundness hole. The axiom implicitly asserts that no two distinct closures can be extensionally equivalent, which is false in general.

## All Candidates

### φ1: temp_pred_equality_collapses_extensional
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body and forces intensional equality of spec_fn from mere extensional equivalence — conflating logical equivalence with structural identity
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body axiom forces `p == q` (and hence `p.pred == q.pred`) from mutual entailment alone. This asserts intensional equality of `spec_fn` from extensional equivalence — a genuinely unverified assumption that could be unsound if Verus's SMT encoding ever distinguishes closures that agree on all inputs.

### φ2: always_unfold_suffix_equivalence
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging the gap between two equivalent-but-intensionally-distinct formulations of suffix quantification — if the SMT solver ever distinguishes the closure in always from the direct forall, this axiom papers over a real intensional mismatch
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The `always` definition wraps a `forall` inside a new closure via `TempPred::new`, producing a different `spec_fn` than a bare quantifier over `p.satisfied_by(ex.suffix(i))`. The external_body `always_unfold` bridges this intensional gap without proof. While semantically correct, it is an unverified trust assumption.

### φ3: always_and_commutativity_propagates
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chaining always_and_equality with temp_pred_equality yields equality of always(p∧q) and always(q∧p) as TempPred objects — the external_body axioms compose to equate structurally distinct closures, amplifying the trust surface
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a straightforward consequence of the already-proven `always_and_equality` plus commutativity of `and`. Given the axioms in the file, this is an expected and desirable derived property — it adds no new trust surface beyond what φ1 already identifies.

### φ4: always_idempotent_via_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** always_unfold and temp_pred_equality combine to prove always(p) == always(always(p)) as object equality — the idempotency should hold semantically but object-level equality relies entirely on unverified external_body axioms

### φ5: valid_false_from_external_body_chain
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** if any two mutually-entailing TempPreds are structurally distinct (p != q), temp_pred_equality forces p == q creating a contradiction — this means the axiom implicitly asserts no two distinct closures can be extensionally equivalent, which is false in general
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This is the sharpest finding. The precondition `p != q` is satisfiable for extensionally equivalent but intensionally distinct TempPreds (e.g., two `TempPred::new` calls with different but logically equivalent closures). Combined with `temp_pred_equality` forcing `p == q`, this derives `false` — a direct soundness hole. The axiom implicitly asserts that no two distinct closures can be extensionally equivalent, which is false in general.

