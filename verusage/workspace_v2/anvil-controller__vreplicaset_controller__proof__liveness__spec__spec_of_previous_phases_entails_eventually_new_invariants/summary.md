# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__spec__spec_of_previous_phases_entails_eventually_new_invariants/original.rs`
**Date:** 2026-03-24T05:36:45Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 0

## All Candidates

### φ1: marshal_unmarshal_no_roundtrip
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec/unmarshal_spec are uninterpreted with no roundtrip axiom, so the spec could be lost or corrupted through a marshal-unmarshal cycle

### φ2: unmarshal_ignores_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds without checking state_validation, allowing structurally invalid objects (e.g. negative replicas) to enter the system through deserialization

### φ3: marshal_spec_not_injective
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom, so two distinct specs could serialize to the same Value, making storage unable to distinguish different configurations

### φ4: create_key_namespace_diverges
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace not self.obj.metadata.namespace, so the storage key can silently disagree with the object's own namespace metadata

### φ5: object_ref_ignores_uid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** object_ref uses only kind/name/namespace and ignores uid, so a deleted-and-recreated resource with a new uid maps to the same ObjectRef — stale reconciliation state from the old instance could target the new one

