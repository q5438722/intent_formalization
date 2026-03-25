# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_garbage_collector_does_not_delete_vrs_pods/original.rs`
**Date:** 2026-03-24T05:19:05Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives — standard consequences of intentional spec design choices (unbounded nat IDs and Kubernetes-style key derivation from embedded objects).

## All Candidates

### φ1: unmarshal_marshal_roundtrip
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec/unmarshal_spec are uninterpreted with no roundtrip axiom — if this is entailed, it reveals the spec accidentally assumes marshalling preserves metadata identity without an explicit axiom

### φ2: unmarshal_spec_status_independent
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** unmarshal checks spec and status independently — any combination of individually-valid spec and status values produces a valid object, which may not hold if there are cross-field invariants

### φ3: allocator_ids_never_collide
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** allocate returns the current counter then increments — successive allocations yield different IDs, but this relies on nat not wrapping; if entailed it confirms the spec assumes unbounded allocation without modeling resource exhaustion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** ReconcileId is `nat` (unbounded), so `a.reconcile_id_counter != a.reconcile_id_counter + 1` always holds. This is the intended design for spec-level modeling.

### φ4: update_key_kind_from_obj_not_request
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key takes kind from self.obj.kind rather than having an independent kind field — a malformed request with mismatched obj.kind could target a different resource type than intended
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This directly restates the definition of `UpdateRequest::key()`. The kind comes from the embedded object by design, matching Kubernetes API semantics.

### φ5: object_ref_ignores_uid
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** object_ref is (kind, name, namespace) with no uid — two distinct resource incarnations with different uids produce identical ObjectRefs, conflating their identity throughout the spec

