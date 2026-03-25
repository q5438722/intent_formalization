# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__spec__next_with_wf_is_stable/original.rs`
**Date:** 2026-03-24T05:35:16Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate is a false positive. `reconcile_id_allocator_no_uniqueness` merely demonstrates that a pure deterministic function produces the same output given the same input — this is a tautology of value semantics, not a spec gap. Global ID uniqueness is an invariant maintained at the cluster level by ensuring only one `ReconcileIdAllocator` exists, which is outside this spec's scope.

## All Candidates

### φ1: marshal_unmarshal_no_roundtrip
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec and unmarshal_spec are uninterpreted with no roundtrip axiom, so distinct specs can marshal to values that unmarshal to the same result — violating injectivity and losing data silently

### φ2: unmarshal_ignores_state_validation
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** unmarshal succeeds without checking state_validation, so invalid VReplicaSet objects (e.g. negative replicas, missing template) can enter the system through deserialization

### φ3: marshal_spec_not_injective
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom, so two distinct specs could marshal to the same Value, making the API server unable to distinguish different VReplicaSet configurations

### φ4: reconcile_id_allocator_no_uniqueness
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** two independently created ReconcileIdAllocators with the same counter produce identical IDs — there is no global uniqueness guarantee across controllers, so concurrent reconciliations from different controllers can have colliding reconcile IDs
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Two allocators with identical state producing identical output is expected pure-functional behavior. Global uniqueness of reconcile IDs is ensured architecturally by maintaining a single allocator instance in the cluster state, not by the allocator's local spec.

### φ5: object_ref_ignores_uid
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** object_ref uses only kind/name/namespace and ignores uid, so a deleted-and-recreated VReplicaSet with a new uid maps to the same ObjectRef — stale reconciliation state from the old instance could be applied to the new one

