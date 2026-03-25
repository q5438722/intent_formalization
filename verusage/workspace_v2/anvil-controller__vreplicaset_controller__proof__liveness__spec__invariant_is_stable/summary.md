# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__spec__invariant_is_stable/original.rs`
**Date:** 2026-03-24T05:35:02Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Of the 4 candidates, 3 are true positives: `create_key_namespace_diverges`, `update_key_ignores_obj_namespace`, and `update_key_ignores_obj_name` all highlight the same design pattern where request routing keys (`key()`) use top-level request fields rather than the embedded object's metadata, allowing divergence between where a resource is stored/targeted and what the object payload claims. The remaining candidate (`reconcile_id_allocator_no_uniqueness`) is a false positive — identical state producing identical output is expected functional behavior, with global uniqueness enforced by singleton allocation at the cluster level.

## True Positives (Spec Issues)

### create_key_namespace_diverges
- **Confidence:** medium
- **Reasoning:** `CreateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key that doesn't match its own metadata, which could lead to inconsistent lookups.

### update_key_ignores_obj_namespace
- **Confidence:** medium
- **Reasoning:** `UpdateRequest::key()` uses `self.namespace` instead of `self.obj.metadata.namespace`. Same pattern as CreateRequest — the request's routing key can diverge from the object payload's namespace, allowing an update to target a different namespace than the object describes.

### update_key_ignores_obj_name
- **Confidence:** medium
- **Reasoning:** `UpdateRequest::key()` uses `self.name` instead of `self.obj.metadata.name`. This means an update request can target resource "A" while carrying the payload of resource "B", silently overwriting with mismatched data.

## All Candidates

### φ1: create_key_namespace_diverges
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace instead of self.obj.metadata.namespace, so the key can silently disagree with the object's own namespace metadata
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `CreateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key that doesn't match its own metadata, which could lead to inconsistent lookups.

### φ2: update_key_ignores_obj_namespace
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.namespace and self.name rather than the object's own metadata, allowing the key to reference a different resource than what the object describes
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `UpdateRequest::key()` uses `self.namespace` instead of `self.obj.metadata.namespace`. Same pattern as CreateRequest — the request's routing key can diverge from the object payload's namespace, allowing an update to target a different namespace than the object describes.

### φ3: update_key_ignores_obj_name
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.name not self.obj.metadata.name, so an update request can target a different name than the object payload carries — the object could overwrite a resource it doesn't represent
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `UpdateRequest::key()` uses `self.name` instead of `self.obj.metadata.name`. This means an update request can target resource "A" while carrying the payload of resource "B", silently overwriting with mismatched data.

### φ4: marshal_unmarshal_no_roundtrip
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec/unmarshal_spec are uninterpreted with no roundtrip axiom — the solver can satisfy this vacuously by making unmarshal always fail, meaning no actual roundtrip guarantee exists

### φ5: reconcile_id_allocator_no_uniqueness
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** two independent ReconcileIdAllocators with the same counter produce identical reconcile IDs — there is no global uniqueness mechanism, so concurrent controllers can allocate colliding IDs
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is an expected consequence of value semantics — two allocators with identical state naturally produce identical outputs. Global uniqueness is ensured by having a single allocator instance in the cluster state, not by the allocator's spec itself.

