# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_forall_leads_to_equality1.rs`
**Date:** 2026-03-24T04:36:02Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives from independent external_body axioms: `tla_forall_always_equality_variant` (always/forall commutation with alias lifting) and `tla_forall_implies_equality1` (forall-implies to exists-implies distribution). Three false positives: the leads-to equality, its directional application, and the identity-alias instantiation are all correct logical consequences with no independent trust surface.

## True Positives (Spec Issues)

### tla_forall_always_equality_variant_intensional_bridge
- **Confidence:** high
- **Reasoning:** `tla_forall_always_equality_variant` is external_body forcing intensional equality between `tla_forall(a_to_always)` and `always(tla_forall(a_to_p))` given pointwise entailment. Unverified trust assumption combining function-level equality lifting with always/forall commutation.

### tla_forall_implies_equality1_intensional_bridge
- **Confidence:** high
- **Reasoning:** `tla_forall_implies_equality1` is external_body forcing unconditional intensional equality between `tla_forall(|a| p(a).implies(q))` and `tla_exists(p).implies(q)`. Independent unverified trust assumption.

## All Candidates

### φ1: tla_forall_always_equality_variant_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_always_equality_variant is external_body forcing intensional equality between tla_forall(a_to_always) and always(tla_forall(a_to_p)) from pointwise entailment — collapses structurally distinct closures without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_forall_always_equality_variant` is external_body forcing intensional equality between `tla_forall(a_to_always)` and `always(tla_forall(a_to_p))` given pointwise entailment. Unverified trust assumption combining function-level equality lifting with always/forall commutation.

### φ2: tla_forall_implies_equality1_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_implies_equality1 is external_body forcing unconditional intensional equality between tla_forall(|a| p(a)=>q) and tla_exists(p)=>q — structurally distinct nested closures collapsed without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_forall_implies_equality1` is external_body forcing unconditional intensional equality between `tla_forall(|a| p(a).implies(q))` and `tla_exists(p).implies(q)`. Independent unverified trust assumption.

### φ3: tla_forall_leads_to_equality1_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chains both external_body axioms to force intensional equality between ∀a.(p(a)~>q) and (∃a.p(a))~>q — propagates intensional collapse through leads_to/always/eventually/implies nesting
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `∀a.(p(a)~>q) = (∃a.p(a))~>q` is a standard temporal logic equivalence. This is a verified lemma with no new trust surface beyond φ1 and φ2.

### φ4: leads_to_from_components
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** uses forced equality to derive that componentwise leads-to implies existential leads-to — if the intensional bridges are unsound this enables unsound temporal reasoning
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of φ3's equality — substituting one side for the other via `satisfied_by`. Semantically correct with no new trust surface.

### φ5: variant_axiom_identity_alias
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** tla_forall_always_equality_variant accepts any a_to_always alias — passing identity |a| always(p(a)) shows the axiom subsumes standard ∀a.□p(a)=□∀a.p(a) equality with the full generality of arbitrary aliases being unverified
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Passing `|a| always(p(a))` as `a_to_always` trivially satisfies the pointwise entailment precondition (self-entailment). This just recovers the standard `∀a.□p(a) = □∀a.p(a)` equality — a correct special case with no new trust surface.

