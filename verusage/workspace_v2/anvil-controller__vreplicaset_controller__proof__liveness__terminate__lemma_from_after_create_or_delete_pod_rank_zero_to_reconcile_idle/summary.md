# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__terminate__lemma_from_after_create_or_delete_pod_rank_zero_to_reconcile_idle/original.rs`
**Date:** 2026-03-24T05:38:14Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 0

## All Candidates

### φ1: marshal_preserves_integrity_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_preserves_integrity is #[verifier::external_body] with unimplemented!() — it assumes the roundtrip axiom without proof, so any property derived from it (including false) is trusted without verification

### φ2: unmarshal_ignores_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds without checking state_validation, allowing structurally invalid objects to enter the system through deserialization

### φ3: marshal_spec_not_injective
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom, so two distinct specs could serialize to the same Value — the API server cannot distinguish different configurations after storage

### φ4: object_ref_ignores_uid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** object_ref uses only kind/name/namespace and ignores uid, so a deleted-and-recreated resource with a new uid maps to the same ObjectRef — stale reconciliation state could target the wrong instance

### φ5: update_key_ignores_obj_namespace
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.namespace rather than self.obj.metadata.namespace, so the update target can silently disagree with the object payload's own namespace metadata

