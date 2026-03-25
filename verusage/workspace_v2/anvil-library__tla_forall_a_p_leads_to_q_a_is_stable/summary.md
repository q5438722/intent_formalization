# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/tla_forall_a_p_leads_to_q_a_is_stable.rs`
**Date:** 2026-03-24T04:27:51Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. This file contains no external_body axioms — the proof is fully verified by Verus's SMT solver. The stability of tla_forall from component stability is a correct temporal logic result, stable(X) + X correctly yields always(X) via modus ponens, and vacuous truth of stable when the predicate fails is standard propositional logic.

## All Candidates

### φ1: stable_target_any_execution
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** the verified proof derives stability of tla_forall(leads_to) from component stability entirely through SMT closure unfolding — if SMT incorrectly resolves nested TempPred::new/always/eventually/implies closures, this propagates unsound stability
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct invocation of the fully verified lemma `tla_forall_a_p_leads_to_q_a_is_stable` which contains no external_body axioms. The proof is entirely SMT-checked; stability of tla_forall from component stability is a correct temporal logic result.

### φ2: leads_to_preserved_at_suffix
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** using the lemma with a constant a_to_q derives that a single leads_to is preserved at any suffix — if the closure identity between |_a| q and the inner tla_forall projection is resolved incorrectly by SMT, this is unsound

### φ3: always_leads_to_from_stable
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** stable(X).satisfied_by(ex) should mean X(ex) ==> always(X)(ex), but extracting always from the nested closure wrapping requires SMT to resolve stable's implies/always closure chain — if this resolves incorrectly it gives always for free
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `stable(X).satisfied_by(ex)` unfolds to `X.satisfied_by(ex) ==> always(X).satisfied_by(ex)`. Given `X.satisfied_by(ex)` as a precondition, modus ponens yields `always(X).satisfied_by(ex)`. This is correct logic with no external_body dependency.

### φ4: tla_forall_leads_to_from_components
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** chaining stability with componentwise satisfaction yields always(tla_forall(leads_to)) — this lifts infinitely many temporal properties into a single always-forall through SMT-resolved closure nesting without external_body but relying on deep intensional reasoning

### φ5: vacuous_stable_any_pred
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** stable(X) = X ==> always(X), so when X is false the implication is vacuously true — stable is trivially satisfied for any execution where the predicate fails, which means valid(stable(X)) gives no information about executions where X doesn't hold
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `stable(X) = X ==> always(X)`, so when `!X.satisfied_by(ex)`, the implication is vacuously true. This is standard propositional logic, not a spec gap — `valid(stable(X))` is not expected to provide information when X doesn't hold.

