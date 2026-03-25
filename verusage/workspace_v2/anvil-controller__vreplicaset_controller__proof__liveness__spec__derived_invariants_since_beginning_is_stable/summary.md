# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__liveness/vreplicaset_controller__proof__liveness__spec__derived_invariants_since_beginning_is_stable.rs`
**Date:** 2026-03-24T03:16:15Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five verified properties are false positives. The unmarshal/validation separation is an intentional design pattern where deserialization and semantic validation are distinct concerns, with the API server enforcing both via `each_builtin_object_in_etcd_is_well_formed`. The label selector semantics (None and empty map matching everything) faithfully model Kubernetes behavior, with VRS-specific guards in `_state_validation` preventing misuse. ObjectRef's exclusion of uid is by design — it serves as an API-level identifier while OwnerReferenceView handles uid-based ownership. The CreateRequest namespace divergence models real Kubernetes behavior where the URL path namespace is authoritative. The spec appears well-designed with appropriate separation of concerns and invariants maintaining safety at the system level rather than at individual function boundaries.

## All Candidates

### φ1: unmarshal_skips_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds when kind/spec/status parse but never checks state_validation — pods with spec == None can be unmarshalled despite failing state_validation
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is intentional separation of concerns — unmarshal handles deserialization while state_validation is checked separately by the API server (via `valid_object`). The `each_builtin_object_in_etcd_is_well_formed` invariant ensures objects in etcd satisfy both unmarshallability AND validity. Combining them in unmarshal would conflate parsing with semantic validation.

### φ2: label_selector_none_matches_everything
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** a LabelSelectorView with match_labels == None matches every label set including empty — this means a selector with no constraints selects all pods which may be unintended
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This matches real Kubernetes semantics — a nil/empty selector matches all objects. The Kubernetes API documentation explicitly states that an empty label selector matches everything. The VReplicaSet's `_state_validation` separately enforces that match_labels must be Some and non-empty for VRS resources.

### φ3: label_selector_empty_map_matches_everything
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** a selector with an empty match_labels map vacuously matches all label sets — the forall over an empty domain is trivially true, making Some(empty) and None equivalent
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Same as above — an empty match_labels map vacuously matching everything is correct Kubernetes behavior. The VRS state_validation guards against this by requiring `match_labels->0.len() > 0`. The LabelSelectorView itself is a general-purpose spec used across many resource types, so the permissive semantics are by design.

### φ4: object_ref_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** object_ref is (kind, name, namespace) ignoring uid — two distinct resource incarnations with different uids produce identical ObjectRefs conflating identity across delete-recreate cycles
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** ObjectRef is deliberately defined as (kind, name, namespace) — this is how Kubernetes identifies resources for API operations (get, update, delete all use kind+name+namespace). UID is used for ownership tracking via OwnerReferenceView, which does include uid. The model correctly uses ObjectRef for API lookups and OwnerReferenceView (with uid) for ownership, matching real Kubernetes semantics.

### φ5: create_key_namespace_diverges
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace not self.obj.metadata.namespace — these can diverge so the key does not faithfully represent the object's actual namespace
- **Verdict:** FALSE_POSITIVE (medium)
- **Reasoning:** In Kubernetes, the namespace in the API URL path (modeled by `self.namespace`) is authoritative — the server ignores or overwrites the namespace in the object's metadata. The key correctly uses `self.namespace` because that represents the actual target namespace of the create operation. The API server would reconcile any metadata.namespace mismatch. This models real Kubernetes behavior where the URL namespace takes precedence.

