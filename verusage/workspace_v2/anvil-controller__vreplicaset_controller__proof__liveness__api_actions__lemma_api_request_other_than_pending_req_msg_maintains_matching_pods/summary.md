# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__api_actions__lemma_api_request_other_than_pending_req_msg_maintains_matching_pods/original.rs`
**Date:** 2026-03-24T05:22:51Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `CreateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`, allowing divergence between the request's key and the embedded object's namespace. The other two are false positives reflecting intentional Kubernetes design — unmarshal is separate from validation, and ObjectRef intentionally excludes UID.

## True Positives (Spec Issues)

### create_key_namespace_diverges
- **Confidence:** medium
- **Reasoning:** `CreateRequest::key()` uses `self.namespace` while the embedded object carries its own `metadata.namespace`. If these diverge, the key identifies a different namespace than the object claims, which could cause the object to be stored under the wrong namespace key.

## All Candidates

### φ1: unmarshal_roundtrip_no_axiom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec/unmarshal_spec are uninterpreted with no roundtrip axiom — if provable the solver assumes an unjustified bijection between typed specs and serialized Values

### φ2: marshal_spec_injective
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec is uninterpreted with no declared injectivity — if provable the solver implicitly assumes injectivity which is stronger than what the spec provides

### φ3: unmarshal_skips_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds when kind/spec/status parse correctly but never checks state_validation — invalid objects like pods with spec == None can be successfully unmarshalled
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Unmarshal is intentionally a parsing operation separate from validation. In Kubernetes, deserialization succeeds if the wire format is valid; state validation is a separate admission step applied by the API server.

### φ4: create_key_namespace_diverges
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace not self.obj.metadata.namespace — these can diverge so the key does not faithfully represent the object's actual namespace
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `CreateRequest::key()` uses `self.namespace` while the embedded object carries its own `metadata.namespace`. If these diverge, the key identifies a different namespace than the object claims, which could cause the object to be stored under the wrong namespace key.

### φ5: object_ref_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** object_ref is (kind, name, namespace) ignoring uid — two distinct resource incarnations with different uids produce identical ObjectRefs, conflating identity across delete-recreate cycles
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This mirrors real Kubernetes semantics where ObjectRef identifies resources by (kind, name, namespace), not UID. UID distinguishes incarnations but is not part of the logical identity used for lookups.

