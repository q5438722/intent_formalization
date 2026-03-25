# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/always_distributed_by_and.rs`
**Date:** 2026-03-24T03:37:58Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 2
- False positives: 0

## Summary

Both candidates are true positives identifying real trust assumptions. φ1 directly demonstrates the unverified intensional-to-extensional bridge in `always_unfold`. φ2 shows that this axiom's power extends beyond its stated purpose — it enables the SMT solver to re-pack quantified facts into new closures (the converse direction), making the external_body assumption stronger than it appears at first glance.

## True Positives (Spec Issues)

### always_unfold_extensional_gap
- **Confidence:** high
- **Reasoning:** This directly exercises the external_body `always_unfold` axiom, which bridges an intensional gap between the `forall` embedded inside `always`'s closure and a bare `forall` in the ensures. The axiom is unverified and assumes these intensionally distinct formulations are equivalent — a real trust assumption in the spec.

### always_and_reverse_not_proven
- **Confidence:** high
- **Reasoning:** This proves `always(p) ∧ always(q) → always(p∧q)` by using `always_unfold` to extract bare quantifiers, then relying on SMT to re-pack them into the `always(p.and(q))` closure. The fact that this verifies demonstrates the external_body axiom is powerful enough to close intensional gaps in both directions — not just unpacking but implicitly enabling repacking into new closures, amplifying the unverified trust surface.

## All Candidates

### φ1: always_unfold_extensional_gap
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_unfold is external_body bridging an intensional closure gap — the forall inside always's TempPred::new closure vs the bare forall in the ensures are intensionally distinct, and this axiom papers over that without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This directly exercises the external_body `always_unfold` axiom, which bridges an intensional gap between the `forall` embedded inside `always`'s closure and a bare `forall` in the ensures. The axiom is unverified and assumes these intensionally distinct formulations are equivalent — a real trust assumption in the spec.

### φ2: always_and_reverse_not_proven
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** the converse of always_distributed_by_and (always(p) ∧ always(q) → always(p∧q)) relies on always_unfold to extract the quantifiers, then re-wrapping into a new closure — if this verifies, the external_body axiom is strong enough to close intensional gaps in both directions
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This proves `always(p) ∧ always(q) → always(p∧q)` by using `always_unfold` to extract bare quantifiers, then relying on SMT to re-pack them into the `always(p.and(q))` closure. The fact that this verifies demonstrates the external_body axiom is powerful enough to close intensional gaps in both directions — not just unpacking but implicitly enabling repacking into new closures, amplifying the unverified trust surface.

### φ3: always_suffix_shift
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** always(p) on ex should imply always(p) on any suffix, but proving this requires re-packing quantified suffix facts into the always closure on a shifted execution — if always_unfold is powerful enough to close this, it may be overly strong

### φ4: always_vacuous_on_false
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** always(false) trivially implies anything including false itself — but if valid(always(false).implies(q)) holds for arbitrary q via explosion, the spec framework could silently derive any property from an inconsistent always assumption

### φ5: suffix_suffix_composition
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** ex.suffix(i).suffix(j) creates a doubly-nested closure (|k| (|m| nat_to_state(m+i))(k+j)) that should equal ex.suffix(i+j) but is intensionally distinct — if always_unfold is strong enough to prove p holds on this double-suffix without an explicit composition lemma, the axiom may be unsoundly collapsing closure identities

