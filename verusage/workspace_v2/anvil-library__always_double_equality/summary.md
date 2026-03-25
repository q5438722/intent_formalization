# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_double_equality.rs`
**Date:** 2026-03-24T03:39:33Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 0

## All Candidates

### φ1: execution_equality_collapses_distinct_fns
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** execution_equality is external_body and forces intensional equality of spec_fn from mere extensional equivalence — two distinct closures that agree pointwise are collapsed to structurally equal

### φ2: temp_pred_equality_forces_pred_equality
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** temp_pred_equality is external_body forcing intensional equality of the underlying spec_fn pred fields from mutual entailment — conflating logical equivalence with structural identity

### φ3: suffix_zero_equals_self
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** ex.suffix(0) creates a new closure |i| f(i+0) which is intensionally distinct from ex — execution_equality silently equates these without proof, and always_double_equality depends on this at the suffix(0) step

### φ4: suffix_composition_equals_addition
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** ex.suffix(i).suffix(j) produces a doubly-nested closure intensionally distinct from ex.suffix(i+j) — execution_equality collapses this without proof, and this composition is the key step enabling always_double_equality

### φ5: mutual_entailment_derives_false_if_distinct
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** if any two mutually-entailing TempPreds are intensionally distinct (p != q), temp_pred_equality forces p == q creating a contradiction — the axiom implicitly asserts no two distinct closures can be extensionally equivalent

