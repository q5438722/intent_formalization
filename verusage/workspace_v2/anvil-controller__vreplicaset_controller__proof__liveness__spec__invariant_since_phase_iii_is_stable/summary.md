# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__liveness/vreplicaset_controller__proof__liveness__spec__invariant_since_phase_iii_is_stable.rs`
**Date:** 2026-03-24T03:21:02Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The request key namespace/name patterns (φ1–φ3) faithfully model Kubernetes API conventions where URL path parameters are authoritative over object body metadata. The controller_owner_ref omitting namespace (φ4) matches real Kubernetes OwnerReference semantics where references are namespace-scoped and uid provides global uniqueness. The unmarshal/validation separation (φ5) is an intentional design where deserialization and semantic validation are distinct concerns, with system-level invariants ensuring stored objects satisfy both.

## All Candidates

### φ1: create_key_namespace_diverges
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace not self.obj.metadata.namespace, so the key can silently disagree with the object's own namespace
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This models real Kubernetes API behavior where the URL path namespace (self.namespace) is authoritative, not the object body's metadata.namespace. The API server reconciles any mismatch by overwriting the object's namespace with the path namespace.

### φ2: update_key_ignores_obj_name
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.name rather than self.obj.metadata.name, allowing the update to target a different resource than the object payload describes
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Same Kubernetes pattern — the URL path (self.name, self.namespace) identifies the target resource, not the object payload's metadata. The API server uses the path to locate the resource and may overwrite the body's metadata fields. This is intentional modeling.

### φ3: get_then_update_key_ignores_obj_namespace
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** GetThenUpdateRequest::key uses self.namespace not self.obj.metadata.namespace, enabling a cross-namespace object swap
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Identical pattern to CreateRequest and UpdateRequest — the request-level namespace field models the URL path namespace which is authoritative in real Kubernetes. All four request types (Create, Update, UpdateStatus, GetThenUpdate) consistently use the same convention.

### φ4: controller_owner_ref_ignores_namespace
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** controller_owner_ref uses only name and uid but not namespace — two VReplicaSets in different namespaces with the same name and uid produce identical owner references, conflating cross-namespace ownership
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This faithfully models real Kubernetes OwnerReference semantics — owner references contain kind, name, and uid but deliberately exclude namespace because owner references are namespace-scoped (owner must be in the same namespace as the owned object). UID provides global uniqueness, so cross-namespace collision with identical uid is impossible in practice since the API server assigns globally unique UIDs.

### φ5: unmarshal_skips_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds whenever kind/spec/status parse correctly but never checks state_validation — objects that violate domain invariants (e.g. pods with spec == None) can be successfully unmarshalled
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Unmarshal is intentionally separated from validation — it handles deserialization only, while state_validation is checked separately by the API server logic (via InstalledType.valid_object). The invariant `each_builtin_object_in_etcd_is_well_formed` ensures objects in etcd satisfy both unmarshallability and validity.

