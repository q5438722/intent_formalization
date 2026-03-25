# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/leads_to_by_borrowing_inv.rs`
**Date:** 2026-03-24T04:04:53Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two true positives: `instantiate_entailed_always` and `instantiate_entailed_leads_to` are independent external_body axioms each bridging multiple intensional closure gaps — the former resolves implies-modus-ponens plus always-forall extraction in one step, the latter resolves implies plus always-forall plus leads_to's nested closure structure. The remaining two are false positives: ex falso from always(false) and reflexive leads_to are standard logical properties.

## True Positives (Spec Issues)

### instantiate_entailed_always_intensional_bridge
- **Confidence:** high
- **Reasoning:** `instantiate_entailed_always` is external_body bridging `spec.implies(always(p)).satisfied_by(ex)` through nested `TempPred::new` closures — modus ponens on the implies closure, extraction of the forall from always's closure, and instantiation at index `i`. Multiple intensional closure gaps resolved without proof.

### instantiate_entailed_leads_to_intensional_bridge
- **Confidence:** high
- **Reasoning:** `instantiate_entailed_leads_to` is external_body bridging `spec.implies(p.leads_to(q)).satisfied_by(ex)` through four layers of `TempPred::new` closure nesting (implies, always, implies, eventually) to extract `p.implies(eventually(q)).satisfied_by(ex.suffix(i))`. This is a separate unverified trust assumption resolving multiple intensional gaps in a single axiom.

## All Candidates

### φ1: instantiate_entailed_always_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** instantiate_entailed_always is external_body bridging spec.implies(always(p)).satisfied_by(ex) through nested TempPred::new closures and extracting a forall-over-suffix to a specific index — resolves multiple intensional closure gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `instantiate_entailed_always` is external_body bridging `spec.implies(always(p)).satisfied_by(ex)` through nested `TempPred::new` closures — modus ponens on the implies closure, extraction of the forall from always's closure, and instantiation at index `i`. Multiple intensional closure gaps resolved without proof.

### φ2: instantiate_entailed_leads_to_intensional_bridge
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** instantiate_entailed_leads_to is external_body bridging spec.implies(leads_to).satisfied_by(ex) through always/implies/eventually closure nesting to extract a specific suffix index — resolves multiple intensional closure gaps without proof
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `instantiate_entailed_leads_to` is external_body bridging `spec.implies(p.leads_to(q)).satisfied_by(ex)` through four layers of `TempPred::new` closure nesting (implies, always, implies, eventually) to extract `p.implies(eventually(q)).satisfied_by(ex.suffix(i))`. This is a separate unverified trust assumption resolving multiple intensional gaps in a single axiom.

### φ3: borrow_false_inv_weakens_leads_to
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** if spec entails always(false) then spec is vacuously inconsistent — leads_to_by_borrowing_inv derives any leads_to from such a spec without flagging the inconsistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** If spec entails `always(false)`, then spec is unsatisfiable and vacuously entails anything. This is standard ex falso quodlibet, not a spec gap.

### φ4: borrow_trivial_inv_any_leads_to
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** if inv is always true under spec but inv implies false universally, then any leads_to is derivable — the borrowing pattern propagates vacuous inconsistency without detecting it

### φ5: leads_to_self_trivial
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** p.leads_to(p) should be trivially true without needing any invariant — that the proof requires borrowing an invariant and invoking two external_body axioms reveals the intensional closure gaps in the framework
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct instantiation of `leads_to_by_borrowing_inv` — given the preconditions are met, deriving `p.leads_to(p)` is expected. That the framework requires external_body axioms for this is already captured by φ1 and φ2; the property itself is desirable.

