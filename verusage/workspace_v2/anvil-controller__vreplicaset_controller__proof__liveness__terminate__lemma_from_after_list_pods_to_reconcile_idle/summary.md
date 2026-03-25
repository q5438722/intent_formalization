# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__terminate__lemma_from_after_list_pods_to_reconcile_idle/original.rs`
**Date:** 2026-03-24T05:40:25Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Two true positives found: `marshal_preserves_integrity_external_body` flags a genuine soundness gap where the marshal/unmarshal roundtrip is trusted without proof via `external_body`, and `update_key_ignores_obj_namespace` identifies a real spec concern where `UpdateRequest::key()` derives its namespace from the request wrapper rather than the embedded object's metadata. The remaining three are false positives: unmarshal correctly separates parsing from validation (Kubernetes design), the uninterpreted marshal_spec's non-injectivity is a vacuous observation given the roundtrip axiom, and object_ref ignoring uid matches Kubernetes' intentional identity model.

## True Positives (Spec Issues)

### marshal_preserves_integrity_external_body
- **Confidence:** high
- **Reasoning:** `marshal_preserves_integrity` is `#[verifier::external_body]` with `unimplemented!()` — the marshal/unmarshal roundtrip is axiomatically assumed without proof. All downstream integrity reasoning is trusted, not verified.

### update_key_ignores_obj_namespace
- **Confidence:** medium
- **Reasoning:** `UpdateRequest::key()` uses `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key that doesn't match its own metadata, potentially causing inconsistent lookups.

## All Candidates

### φ1: marshal_preserves_integrity_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_preserves_integrity is #[verifier::external_body] with unimplemented!() — the roundtrip property is assumed without proof, so any property derived from it rests on an unverified trust assumption
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `marshal_preserves_integrity` is `#[verifier::external_body]` with `unimplemented!()` — the marshal/unmarshal roundtrip is axiomatically assumed without proof. All downstream integrity reasoning is trusted, not verified.

### φ2: unmarshal_ignores_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds without checking state_validation, allowing structurally invalid objects to enter the system through deserialization
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is standard Kubernetes design. Unmarshalling is pure deserialization; state validation is a separate admission-time concern performed by the API server after successful unmarshal.

### φ3: marshal_spec_not_injective
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom, so two distinct specs could map to the same Value — the API server cannot distinguish different configurations after storage
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `marshal_spec` is `uninterp spec fn` — Verus treats it as an arbitrary function, which means it *could* be non-injective but the spec makes no claim either way. The `marshal_preserves_integrity` axiom already guarantees roundtrip fidelity (unmarshal ∘ marshal = id), which implies injectivity of the composite marshal. The candidate only shows the *absence* of a standalone injectivity axiom, not an actual inconsistency.

### φ4: update_key_ignores_obj_namespace
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.namespace rather than self.obj.metadata.namespace, so the update target can silently disagree with the object payload's own namespace metadata
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `UpdateRequest::key()` uses `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key that doesn't match its own metadata, potentially causing inconsistent lookups.

### φ5: object_ref_ignores_uid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** object_ref uses only kind/name/namespace and ignores uid, so a deleted-and-recreated resource with a new uid maps to the same ObjectRef — stale reconciliation state could target the wrong instance
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This mirrors Kubernetes' actual design where `ObjectRef` (kind/name/namespace) is the logical identity for resources. UID distinguishes incarnations but is not part of the lookup key — this is intentional and matches the real API server's behavior.

