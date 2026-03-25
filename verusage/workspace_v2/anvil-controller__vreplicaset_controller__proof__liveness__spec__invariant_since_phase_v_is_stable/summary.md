# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__liveness/vreplicaset_controller__proof__liveness__spec__invariant_since_phase_v_is_stable.rs`
**Date:** 2026-03-24T03:22:32Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. The request key namespace/name patterns (φ1–φ2) model standard Kubernetes API conventions where URL path parameters are authoritative. The Pod-only scoping (φ3) is intentional since VRS only manages Pods. The namespace-free OwnerReference (φ4) matches real Kubernetes semantics with uid providing global uniqueness. The vacuous update-status check for absent keys (φ5) is harmless since such requests will fail at the API server anyway.

## All Candidates

### φ1: create_key_namespace_diverges
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace not self.obj.metadata.namespace, so the key can silently disagree with the object's own namespace metadata
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This models real Kubernetes API behavior where the URL path namespace is authoritative. The API server uses the request-level namespace, not the object body's metadata.namespace, to determine resource placement.

### φ2: update_key_ignores_obj_name
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.name rather than self.obj.metadata.name, allowing the key to target a different resource than what the object payload describes
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Same Kubernetes convention — the URL path parameters (name, namespace) identify the target resource, not the object payload's metadata. The API server reconciles any mismatch by using the path-level identifiers.

### φ3: create_interference_vacuous_non_pod
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** the create interference check is vacuously satisfied for non-Pod kinds — another controller could create a non-Pod resource with VRS owner references and the invariant would not flag it
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The VReplicaSet controller only manages Pods, so the interference predicates intentionally scope their protection to PodKind. Non-Pod resources with VRS owner references are irrelevant to VRS reconciliation correctness and are handled by other controllers' own invariants.

### φ4: controller_owner_ref_ignores_namespace
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** controller_owner_ref does not include namespace — two VReplicaSets in different namespaces with same name and uid produce identical owner references, so the interference predicates cannot distinguish cross-namespace ownership
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This faithfully models Kubernetes OwnerReference semantics which include kind, name, and uid but not namespace. Owner references are namespace-scoped (owner must be in the same namespace), and uid provides global uniqueness, so cross-namespace collision with identical uid is impossible under the API server's uid allocation invariant.

### φ5: update_status_interference_missing_key_check
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** the update-status interference predicate accesses s.resources()[req.key()] without guarding on contains_key — when the key is absent, the negation of the inner block is trivially satisfiable, so updates to nonexistent pods pass the interference check
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** When the key is absent from etcd, an UpdateStatusRequest will fail at the API server (ObjectNotFound), so it cannot actually interfere with any existing VRS-owned pod. The predicate being vacuously true for nonexistent resources is correct — there is no pod to protect.

