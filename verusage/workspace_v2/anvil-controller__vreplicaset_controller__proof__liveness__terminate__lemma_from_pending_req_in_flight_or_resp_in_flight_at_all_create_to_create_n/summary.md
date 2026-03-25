# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__liveness/vreplicaset_controller__proof__liveness__terminate__lemma_from_pending_req_in_flight_or_resp_in_flight_at_all_create_to_create_n.rs`
**Date:** 2026-03-24T03:32:48Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. The lemma specialization (φ1) and vacuous implication (φ2) both reflect standard logical patterns in conditional invariants — narrowing an implication's antecedent preserves validity, and implications are vacuously true when the guard fails. The object_ref uid omission (φ3) faithfully models Kubernetes API semantics where resource identity is (kind, name, namespace).

## All Candidates

### φ1: lemma_specializes_any_n_without_constraint
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** the lemma instantiates for any n including values far beyond the actual replicas count, so the "pending req or resp in flight" property is claimed for AfterCreatePod(999999) which should never be a reachable reconcile state
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The lemma's postcondition is an implication — at AfterCreatePod(999999), the at_expected_reconcile_states antecedent is false (the system never reaches that step), so the property holds vacuously. This is standard logical strengthening of an implication by narrowing the antecedent; no false conclusion is derivable.

### φ2: pending_req_vacuous_when_not_at_state
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** pending_req_in_flight_or_resp_in_flight_at_reconcile_state is vacuously true whenever the key has no ongoing reconcile, so the "always" invariant in the lemma's precondition is trivially satisfiable for non-existent reconciles — the lemma provides no guarantee when the reconcile is absent
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the intended design — pending_req_in_flight_or_resp_in_flight_at_reconcile_state is structured as an implication guarded by at_expected_reconcile_states, which requires contains_key. When no reconcile exists, the guard is false and the implication is vacuously true. This is standard for conditional invariants.

### φ3: unmarshal_unwrap_undefined_on_bad_state
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** unmarshal is uninterpreted so .unwrap() on Err yields an arbitrary but fixed-per-input value — at_step_closure cannot meaningfully distinguish bad states, meaning the reconcile step predicates are undefined on corrupt local state

### φ4: at_step_closure_conflates_steps_on_unmarshal_collision
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** when unmarshal fails, unwrap produces the same arbitrary value regardless of which step is being tested — all step predicates collapse to the same boolean, making step-specific reasoning unsound on non-unmarshallable states

### φ5: object_ref_ignores_uid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** object_ref uses only kind/name/namespace and ignores uid, so a deleted-and-recreated VReplicaSet with a new uid maps to the same ObjectRef — ongoing_reconciles keyed by object_ref could conflate old and new resource instances
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This matches real Kubernetes semantics where resources are identified by kind/name/namespace. UID is not part of the resource key. The system handles stale-instance concerns through resource_version checks and reconcile restarts, not through ObjectRef identity.

