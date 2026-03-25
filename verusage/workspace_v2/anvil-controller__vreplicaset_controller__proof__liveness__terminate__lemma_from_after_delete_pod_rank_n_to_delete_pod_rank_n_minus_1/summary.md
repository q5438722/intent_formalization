# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__terminate__lemma_from_after_delete_pod_rank_n_to_delete_pod_rank_n_minus_1/original.rs`
**Date:** 2026-03-24T05:39:46Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 2
- False positives: 3

## Summary

Of the 5 candidates, 2 are true positives. `marshal_preserves_integrity_external_body` identifies a real soundness gap: the marshal/unmarshal roundtrip axiom is entirely trusted via `external_body` with no proof. `update_key_ignores_obj_namespace` flags a genuine spec concern where request-level namespace can silently disagree with the embedded object's metadata namespace. The remaining 3 are false positives: unmarshal intentionally omits state validation (by design), `marshal_spec` non-injectivity is vacuously true due to uninterpreted function semantics, and `object_ref` ignoring UID is standard Kubernetes identity semantics.

## True Positives (Spec Issues)

### marshal_preserves_integrity_external_body
- **Confidence:** high
- **Reasoning:** `marshal_preserves_integrity` is `#[verifier::external_body]` with `unimplemented!()` — the marshal/unmarshal roundtrip is axiomatically assumed without proof. All downstream integrity reasoning is trusted, not verified.

### update_key_ignores_obj_namespace
- **Confidence:** medium
- **Reasoning:** `UpdateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key inconsistent with its own metadata. The same pattern appears in `UpdateStatusRequest::key()`.

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
- **Reasoning:** This is standard Kubernetes design. Unmarshalling is pure deserialization; state validation is a separate admission-time concern. Coupling validation into unmarshal would conflate parsing with policy enforcement.

### φ3: marshal_spec_not_injective
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom, so two distinct specs could serialize to the same Value — the API server cannot distinguish different configurations after storage
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `marshal_spec` is `uninterp spec fn` — Verus treats uninterpreted functions as injective (equal outputs require equal inputs). The requires clause `s1 != s2 && marshal_spec(s1) == marshal_spec(s2)` is unsatisfiable, making this vacuously true. No real non-injectivity is demonstrated.

### φ4: object_ref_ignores_uid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** object_ref uses only kind/name/namespace and ignores uid, so a deleted-and-recreated resource with a new uid maps to the same ObjectRef — stale reconciliation state could target the wrong instance
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ObjectRef` is intentionally a (kind, name, namespace) triple — this is the standard Kubernetes resource identity model. UID-based disambiguation is handled separately through owner references and garbage collection, not through ObjectRef lookups.

### φ5: update_key_ignores_obj_namespace
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.namespace rather than self.obj.metadata.namespace, so the update target can silently disagree with the object payload's own namespace metadata
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `UpdateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key inconsistent with its own metadata. The same pattern appears in `UpdateStatusRequest::key()`.

