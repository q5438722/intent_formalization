# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__liveness/vreplicaset_controller__proof__liveness__terminate__lemma_from_pending_req_in_flight_or_resp__in_flight_at_all_delete_to_delete.rs`
**Date:** 2026-03-24T03:31:50Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. The unmarshal failure property is a tautology. The contains_key guard concern is addressed by the implication structure in pending_req_in_flight_or_resp_in_flight_at_reconcile_state. The object_ref uid omission matches real Kubernetes API semantics where resources are keyed by kind/name/namespace.

## All Candidates

### φ1: unmarshal_local_state_can_fail
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** unmarshal is uninterpreted with no totality axiom, so at_step_closure and unwrap_local_state_closure call .unwrap() on potentially-Err results — any predicate using these closures has undefined behavior on non-unmarshallable local states
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This property just says `true` holds when unmarshal fails — it's a tautology that demonstrates nothing. The underlying concern about unwrap() on Err is valid in principle, but the system maintains invariants ensuring that local_state values stored in ongoing_reconciles are always unmarshallable, so the closures are never invoked on non-unmarshallable values in reachable states.

### φ2: at_step_closure_vacuous_on_bad_state
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** when unmarshal fails, unwrap() produces an arbitrary value, so at_step_closure cannot distinguish any two steps on corrupt local state — all reconcile step predicates collapse to the same truth value

### φ3: has_pending_req_no_contains_key_guard
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** at_expected_reconcile_states guards on contains_key, but has_pending_req_msg does not — if these predicates are composed in contexts where the guard is missing, map indexing on absent keys yields arbitrary values
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This just confirms that at_expected_reconcile_states returns false when the key is absent, which is exactly its intended behavior — the contains_key check is the guard. The concern about has_pending_req_msg lacking a guard is moot because pending_req_in_flight_or_resp_in_flight_at_reconcile_state only evaluates has_pending_req_msg in the consequent of an implication guarded by at_expected_reconcile_states.

### φ4: pending_req_state_vacuous_when_no_pending
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** pending_req_in_flight_or_resp_in_flight_at_reconcile_state is vacuously true when pending_req_msg is None (has_pending_req_msg fails), so any reconcile state without a pending request trivially satisfies the invariant regardless of actual in-flight message status

### φ5: object_ref_ignores_uid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** object_ref uses only kind/name/namespace and ignores uid, so a deleted-and-recreated VReplicaSet with a new uid maps to the same ObjectRef — the liveness lemma's ongoing_reconciles keyed by object_ref could conflate old and new instances
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This faithfully models Kubernetes semantics where ObjectRef is (kind, name, namespace) — the standard resource identifier. UID provides global uniqueness but is not part of the resource key. The system handles stale instances through resource_version checks and reconcile restart logic, not through ObjectRef.

