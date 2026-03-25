# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__liveness/vreplicaset_controller__proof__liveness__spec__invariant_since_phase_ii_is_stable.rs`
**Date:** 2026-03-24T03:19:58Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The resp_msg_matches_req_msg src/dst swap property is exactly the intended semantics. The deletion_timestamp redundancy in the schedule predicate is defensive but correct since unmarshal preserves metadata. The xor invariant's vacuous truth for None pending messages is intentional per its naming and design. The lack of status checking in the reconcile identity predicate reflects correct Kubernetes controller semantics where status divergence is expected. The object_ref ignoring uid matches real Kubernetes API addressing, with uid checks handled separately where needed.

## All Candidates

### φ1: resp_matches_req_is_symmetric
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** resp_msg_matches_req_msg swaps src/dst and preserves rpc_id but does not enforce that req and resp are on different hosts — a controller could match its own request as a response if content variants align
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The property merely confirms that resp_msg_matches_req_msg enforces dst/src swap and rpc_id equality, which is exactly what the spec intends. The "same host" concern is a non-issue — the content variant matching (APIResponse vs APIRequest, or ExternalResponse vs ExternalRequest) plus the system's message routing invariants prevent self-matching in practice.

### φ2: scheduled_deletion_timestamp_redundant
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** the predicate checks deletion_timestamp on both the raw DynamicObjectView and the unmarshalled VReplicaSetView, but unmarshal copies metadata verbatim — these are always equal, so the second check is redundant and masks the fact that no deeper validation occurs
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The redundancy is real but harmless — unmarshal copies metadata verbatim, so checking deletion_timestamp on both raw and unmarshalled forms is indeed equivalent. This is defensive specification style, not a spec gap. The predicate correctly ensures no deletion timestamp exists on scheduled objects.

### φ3: pending_req_xor_allows_both_absent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** the "xor" invariant is vacuously true when pending_req_msg is None — it provides no constraint on reconciles with no pending request, so a controller in a broken state with no pending message silently satisfies the invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The invariant is explicitly named `_if_has_pending_req_msg` — vacuous truth when pending_req_msg is None is intentional by design. The predicate guards on `has_pending_req_msg` which requires `pending_req_msg is Some`, so the None case is deliberately excluded from the constraint. Reconcile states with no pending message are handled by other invariants.

### φ4: reconcile_spec_uid_no_status_check
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** the_object_in_reconcile_has_spec_and_uid_as checks spec and uid match but completely ignores status — a reconciliation could operate on an object whose status has diverged arbitrarily from the expected CR
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional — the predicate is specifically named `has_spec_and_uid_as`, not `has_spec_status_and_uid_as`. In Kubernetes controller reconciliation, the controller reacts to spec changes and drives status to match; status is expected to diverge and is not relevant for triggering reconciliation identity.

### φ5: object_ref_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** object_ref is (kind, name, namespace) and ignores uid — two different incarnations of a resource after delete-recreate share the same ObjectRef, so maps keyed by ObjectRef conflate distinct resource instances
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This faithfully models Kubernetes semantics where API operations use (kind, name, namespace) as the resource key. UID is used separately for ownership (OwnerReferenceView includes uid) and identity checks (the_object_in_reconcile_has_spec_and_uid_as checks uid). The ObjectRef type correctly serves as the API-level key.

