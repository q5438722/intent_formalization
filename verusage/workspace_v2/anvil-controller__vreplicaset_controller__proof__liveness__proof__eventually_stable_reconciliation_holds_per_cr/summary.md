# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__proof__eventually_stable_reconciliation_holds_per_cr/original.rs`
**Date:** 2026-03-24T05:25:14Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `CreateRequest::key()` uses `self.namespace` instead of `self.obj.metadata.namespace`, allowing namespace divergence between the key and the object's own metadata. The other two are false positives — unmarshal intentionally separates parsing from validation, and UpdateRequest deriving kind from the embedded object is standard Kubernetes semantics.

## True Positives (Spec Issues)

### create_key_namespace_diverges_from_obj
- **Confidence:** medium
- **Reasoning:** `CreateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the key identifies a different namespace than the object claims, potentially storing the object under the wrong key.

## All Candidates

### φ1: unmarshal_roundtrip_no_axiom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec/unmarshal_spec are uninterpreted with no roundtrip axiom — if the solver proves this it reveals an accidental entailment that unmarshal always succeeds on marshalled objects without justification

### φ2: marshal_spec_injective
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom — if provable the solver implicitly assumes injectivity which is a stronger property than declared

### φ3: create_key_namespace_diverges_from_obj
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace not self.obj.metadata.namespace — these can diverge so the key does not faithfully represent the object's namespace
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `CreateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the key identifies a different namespace than the object claims, potentially storing the object under the wrong key.

### φ4: unmarshal_skips_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds whenever kind/spec/status parse but never checks state_validation — invalid objects (e.g. pods with spec == None) are successfully unmarshalled
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Unmarshal is intentionally a parsing/deserialization step separate from validation. In Kubernetes, state validation is applied by the API server as a distinct admission phase after unmarshalling.

### φ5: update_key_kind_from_obj_body
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key takes kind from self.obj.kind rather than having an independent kind field — a request with mismatched obj.kind could target a different resource type than the caller intended
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This directly restates the definition of `UpdateRequest::key()`. Deriving the kind from the embedded object matches Kubernetes API semantics where the request's identity comes from the object being updated.

