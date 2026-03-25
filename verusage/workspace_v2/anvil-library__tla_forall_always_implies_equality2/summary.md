# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_forall_always_implies_equality2.rs`
**Date:** 2026-03-24T04:32:49Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives from independent external_body axioms: `tla_forall_always_equality_variant` (always/forall commutation with alias lifting) and `tla_forall_implies_equality2` (forall/implies distribution). Three false positives: the chained equality, its directional application, and the identity-alias instantiation are all correct logical consequences with no independent trust surface.

## True Positives (Spec Issues)

### tla_forall_always_equality_variant_intensional_bridge
- **Confidence:** high
- **Reasoning:** `tla_forall_always_equality_variant` is external_body forcing intensional equality between `tla_forall(a_to_always)` and `always(tla_forall(a_to_p))` given pointwise entailment. Unverified trust assumption combining function-level equality lifting with always/forall commutation.

### tla_forall_implies_equality2_intensional_bridge
- **Confidence:** high
- **Reasoning:** `tla_forall_implies_equality2` is external_body forcing unconditional intensional equality between `tla_forall(|a| p.implies(q(a)))` and `p.implies(tla_forall(q))`. Independent unverified trust assumption.

## All Candidates

### φ1: tla_forall_always_equality_variant_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_always_equality_variant is external_body forcing intensional equality between tla_forall(a_to_always) and always(tla_forall(a_to_p)) from pointwise entailment — collapses structurally distinct closures without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_forall_always_equality_variant` is external_body forcing intensional equality between `tla_forall(a_to_always)` and `always(tla_forall(a_to_p))` given pointwise entailment. Unverified trust assumption combining function-level equality lifting with always/forall commutation.

### φ2: tla_forall_implies_equality2_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** tla_forall_implies_equality2 is external_body forcing unconditional intensional equality between tla_forall(|a| p=>q(a)) and p=>tla_forall(q) — structurally distinct nested closures collapsed without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `tla_forall_implies_equality2` is external_body forcing unconditional intensional equality between `tla_forall(|a| p.implies(q(a)))` and `p.implies(tla_forall(q))`. Independent unverified trust assumption.

### φ3: tla_forall_always_implies_equality2_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** chains both external_body axioms to force intensional equality between ∀a.□(p⇒q(a)) and □(p⇒∀a.q(a)) — propagates intensional collapse through always/implies/forall nesting
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `∀a.□(p⇒q(a)) = □(p⇒∀a.q(a))` is a standard temporal logic equivalence. This is a verified lemma with no new trust surface beyond φ1 and φ2.

### φ4: always_forall_implies_from_components
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** uses the chained equality to substitute ∀a.□(p⇒q(a)) with □(p⇒∀a.q(a)) — if the intensional bridge is unsound, this enables deriving always-forall from componentwise always-implies
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct consequence of φ3's equality — substituting one side for the other via `satisfied_by`. No new trust surface; semantically correct direction of the ∀a.□(p⇒q(a)) ⟺ □(p⇒∀a.q(a)) equivalence.

### φ5: variant_axiom_arbitrary_alias
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** tla_forall_always_equality_variant accepts any a_to_always alias — passing the identity |a| always(p(a)) directly shows the axiom subsumes the standard ∀a.□p(a)=□∀a.p(a) equality as a special case, with the full generality of accepting arbitrary aliases being the unverified gap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Passing `|a| always(p(a))` as `a_to_always` trivially satisfies the pointwise entailment precondition (self-entailment). This just recovers the standard ∀a.□p(a) = □∀a.p(a) equality — a correct special case with no new trust surface.

