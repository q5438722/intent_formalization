# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/a_to_temp_pred_equality.rs`
**Date:** 2026-03-24T03:36:00Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 0

## True Positives (Spec Issues)

### temp_pred_equality_collapses_distinct
- **Confidence:** high
- **Reasoning:** The external_body axiom asserts intensional equality (==) from extensional equivalence (mutual entailment). In Verus, spec_fn closures are compared intensionally by default, so this axiom introduces a soundness assumption that two distinct closure objects are equal if they agree on all inputs. This is a real trust gap — if any downstream proof depends on distinguishing closures that happen to be extensionally equivalent, this axiom could introduce unsoundness.

### entails_antisymmetry_forces_equality
- **Confidence:** high
- **Reasoning:** This is the same underlying issue as φ1 but stated more precisely: mutual entailment forces `p.pred == q.pred` at the spec_fn level. This directly demonstrates the conflation of extensional and intensional equality — the core soundness concern of the external_body axiom.

### a_to_temp_pred_equality_forces_fn_equality
- **Confidence:** medium
- **Reasoning:** This escalates the external_body assumption one level higher: pointwise TempPred equivalence forces equality of `spec_fn(A) -> TempPred<T>` functions. While the proof is sound given temp_pred_equality plus extensionality, it amplifies the trust surface — any two higher-order functions producing extensionally equivalent TempPreds are collapsed to equal.

## All Candidates

### φ1: temp_pred_equality_collapses_distinct
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body and equates structurally distinct TempPred values (different closure allocations) based only on extensional equivalence — this axiom could unsoundly collapse distinct spec_fn identities that Verus normally distinguishes intensionally
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The external_body axiom asserts intensional equality (==) from extensional equivalence (mutual entailment). In Verus, spec_fn closures are compared intensionally by default, so this axiom introduces a soundness assumption that two distinct closure objects are equal if they agree on all inputs. This is a real trust gap — if any downstream proof depends on distinguishing closures that happen to be extensionally equivalent, this axiom could introduce unsoundness.

### φ2: entails_antisymmetry_forces_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** mutual entailment only guarantees extensional equivalence of predicates, but temp_pred_equality forces intensional equality of the underlying spec_fn fields — this conflates logical equivalence with structural identity
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This is the same underlying issue as φ1 but stated more precisely: mutual entailment forces `p.pred == q.pred` at the spec_fn level. This directly demonstrates the conflation of extensional and intensional equality — the core soundness concern of the external_body axiom.

### φ3: a_to_temp_pred_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** extensional equivalence of TempPred outputs is lifted to full equality of higher-order functions, which could equate functions that differ on non-TempPred-observable behavior or internal representation
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** This escalates the external_body assumption one level higher: pointwise TempPred equivalence forces equality of `spec_fn(A) -> TempPred<T>` functions. While the proof is sound given temp_pred_equality plus extensionality, it amplifies the trust surface — any two higher-order functions producing extensionally equivalent TempPreds are collapsed to equal.

### φ4: trivial_mutual_entailment
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** two independently constructed always-false predicates are forced equal, demonstrating the axiom equates any two extensionally equivalent closures regardless of provenance — could mask distinct failure modes

### φ5: valid_false_implies_anything
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** if valid(false) were ever provable (e.g. via unsound external_body chain), every temporal property would follow — the spec has no guard preventing valid from being instantiated on contradictory predicates

