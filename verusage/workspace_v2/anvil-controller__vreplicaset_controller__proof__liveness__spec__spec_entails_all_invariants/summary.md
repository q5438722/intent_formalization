# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__spec__spec_entails_all_invariants/original.rs`
**Date:** 2026-03-24T05:36:16Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 0

## All Candidates

### φ1: create_key_namespace_disagrees_with_obj
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace rather than self.obj.metadata.namespace, so the key can silently disagree with the object's own namespace metadata

### φ2: update_key_namespace_disagrees_with_obj
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.namespace rather than self.obj.metadata.namespace, so the update target can silently disagree with the object payload's own namespace

### φ3: unmarshal_ignores_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds without checking state_validation, allowing structurally invalid objects to enter the system through deserialization

### φ4: marshal_spec_no_injectivity
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom, so two distinct specs could serialize to the same Value — the system cannot distinguish different configurations after storage

### φ5: object_ref_ignores_uid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** object_ref uses only kind/name/namespace and ignores uid, so a deleted-and-recreated resource with a new uid maps to the same ObjectRef — stale reconciliation could target the wrong instance

