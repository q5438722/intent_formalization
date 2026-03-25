# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__terminate__reconcile_eventually_terminates_on_vrs_object/original.rs`
**Date:** 2026-03-24T05:41:29Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three findings: one soundness gap (`marshal_preserves_integrity_external_body` — roundtrip axiom trusted without proof) and two namespace divergence issues (`update_request_key_namespace_disagrees_with_obj` and `update_status_request_key_namespace_disagrees_with_obj` — both request types derive their key namespace from the wrapper rather than the embedded object's metadata). The remaining two are false positives: unmarshal correctly separates parsing from validation per Kubernetes design, and the marshal_spec non-injectivity candidate is vacuous (ensures `true`).

## True Positives (Spec Issues)

### marshal_preserves_integrity_external_body
- **Confidence:** high
- **Reasoning:** `marshal_preserves_integrity` is `#[verifier::external_body]` with `unimplemented!()` — the marshal/unmarshal roundtrip is axiomatically assumed without proof. All downstream integrity reasoning is trusted, not verified.

### update_request_key_namespace_disagrees_with_obj
- **Confidence:** medium
- **Reasoning:** `UpdateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key inconsistent with its own metadata.

### update_status_request_key_namespace_disagrees_with_obj
- **Confidence:** medium
- **Reasoning:** Same issue as `UpdateRequest` — `UpdateStatusRequest::key()` uses `self.namespace` instead of `self.obj.metadata.namespace`, allowing the status update to target a different namespace than the object payload claims.

## All Candidates

### φ1: marshal_preserves_integrity_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_preserves_integrity is #[verifier::external_body] with unimplemented!() — the roundtrip axiom is assumed without proof, so any property built on it is trusted, not verified
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `marshal_preserves_integrity` is `#[verifier::external_body]` with `unimplemented!()` — the marshal/unmarshal roundtrip is axiomatically assumed without proof. All downstream integrity reasoning is trusted, not verified.

### φ2: unmarshal_ignores_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds without checking state_validation, so structurally invalid objects can enter the system through deserialization
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is standard Kubernetes design. Unmarshalling is pure deserialization; state validation is performed separately at admission time by the API server, not during parsing.

### φ3: update_request_key_namespace_disagrees_with_obj
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.namespace rather than self.obj.metadata.namespace, so the update target can silently disagree with the object's own namespace metadata
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `UpdateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key inconsistent with its own metadata.

### φ4: update_status_request_key_namespace_disagrees_with_obj
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateStatusRequest::key has the same namespace divergence as UpdateRequest — the key targets a different namespace than the object payload claims
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** Same issue as `UpdateRequest` — `UpdateStatusRequest::key()` uses `self.namespace` instead of `self.obj.metadata.namespace`, allowing the status update to target a different namespace than the object payload claims.

### φ5: marshal_spec_no_injectivity
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom, so two distinct specs could serialize to the same Value — the system cannot distinguish different configurations after storage
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The candidate ensures `true` — it proves nothing. The precondition assumes two distinct specs map to the same Value, but `marshal_preserves_integrity` already guarantees roundtrip fidelity, which implies the composite marshal is injective. The absence of a standalone injectivity axiom on `marshal_spec` alone is not a spec gap.

