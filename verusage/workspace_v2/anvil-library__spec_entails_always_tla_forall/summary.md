# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/spec_entails_always_tla_forall.rs`
**Date:** 2026-03-24T04:21:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## True Positives (Spec Issues)

### tla_forall_always_equality_intensional_bridge
- **Confidence:** high
- **Reasoning:** `tla_forall_always_equality_variant` is external_body forcing intensional equality between `tla_forall(|a| always(p(a)))` and `always(tla_forall(p))` — structurally distinct nested `TempPred::new` closures collapsed to `==` without proof.

### spec_entails_tla_forall_closure_bridge
- **Confidence:** high
- **Reasoning:** `spec_entails_tla_forall` is external_body combining pointwise entailments into entailment of `tla_forall` — requires resolving that `tla_forall`'s `TempPred::new` closure-wrapped forall evaluates consistently with individual `a_to_p(a)` results. Independent unverified trust assumption.

## All Candidates

### φ1: tla_forall_always_equality_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_always_equality_variant is external_body forcing intensional equality between tla_forall(always∘p) and always(tla_forall(p)) — structurally distinct nested TempPred::new closures collapsed to equal without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_forall_always_equality_variant` is external_body forcing intensional equality between `tla_forall(|a| always(p(a)))` and `always(tla_forall(p))` — structurally distinct nested `TempPred::new` closures collapsed to `==` without proof.

### φ2: spec_entails_tla_forall_closure_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** spec_entails_tla_forall is external_body combining pointwise entailments into entailment of tla_forall — requires resolving that tla_forall's TempPred::new closure-wrapped forall evaluates consistently with individual a_to_p(a) results
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `spec_entails_tla_forall` is external_body combining pointwise entailments into entailment of `tla_forall` — requires resolving that `tla_forall`'s `TempPred::new` closure-wrapped forall evaluates consistently with individual `a_to_p(a)` results. Independent unverified trust assumption.

### φ3: spec_entails_always_tla_forall_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** spec_entails_always_tla_forall chains both external_body axioms — commuting always inside tla_forall relies on intensional equality of tla_forall(always∘p) with always(tla_forall(p)) plus closure bridge for forall introduction
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct composition of φ1 and φ2 via the verified `spec_entails_always_tla_forall` lemma. No new trust surface beyond the two root axioms.

### φ4: tla_forall_always_swap_any_pred
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** the always/tla_forall commutation swaps ∀a.□p(a) with □∀a.p(a) at the execution level — the semantic equivalence is correct but the intensional closure equality enabling direct substitution is unverified
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `∀a.□p(a) ⟺ □∀a.p(a)` is a semantically correct temporal logic equivalence. This is an expected consequence of φ1, not a new spec gap.

### φ5: false_spec_entails_always_tla_forall
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** an unsatisfiable spec vacuously entails all always-tla_forall combinations — propagates vacuous truth through the axiom chain without detecting spec inconsistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An unsatisfiable spec vacuously entailing anything is standard ex falso quodlibet. Not a spec gap.

