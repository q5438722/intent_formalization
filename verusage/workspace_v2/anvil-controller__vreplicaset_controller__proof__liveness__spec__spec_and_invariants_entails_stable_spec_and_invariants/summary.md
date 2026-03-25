# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__spec__spec_and_invariants_entails_stable_spec_and_invariants/original.rs`
**Date:** 2026-03-24T05:35:46Z
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
- **Why flagged:** marshal_spec and unmarshal_spec are uninterpreted with no roundtrip axiom, so the solver cannot rule out that unmarshal(marshal(s)) produces a different spec — data could be silently corrupted through serialization

### φ2: unmarshal_ignores_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds without calling state_validation, so structurally invalid objects (e.g. negative replicas, missing required fields) can silently enter the system through deserialization

### φ3: marshal_spec_not_injective
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom, so two distinct specs can marshal to the same Value — the API server would be unable to distinguish different VReplicaSet configurations after storage

### φ4: create_key_namespace_diverges
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace rather than self.obj.metadata.namespace, so the storage key can silently disagree with the object's own namespace metadata

### φ5: object_ref_ignores_uid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** object_ref uses only kind/name/namespace and ignores uid, so a deleted-and-recreated resource with a new uid maps to the same ObjectRef — stale reconciliation state could be applied to the wrong instance

