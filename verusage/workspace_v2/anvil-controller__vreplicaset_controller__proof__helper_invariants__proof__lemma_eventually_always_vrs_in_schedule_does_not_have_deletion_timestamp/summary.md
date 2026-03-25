# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__helper_invariants/vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_vrs_in_schedule_does_not_have_deletion_timestamp.rs`
**Date:** 2026-03-24T02:49:07Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: unmarshal_marshal_roundtrip
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** no marshal_preserves_integrity proof or axiom exists in this version — marshal_spec/unmarshal_spec are fully uninterpreted with zero roundtrip guarantees, so this should be unprovable

### φ2: unmarshal_spec_and_status_jointly_total
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** both unmarshal functions are uninterpreted with no totality axiom — matching kind should not guarantee that arbitrary spec and status Values can be successfully decoded

### φ3: get_then_update_key_matches_get_then_delete
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** GetThenUpdateRequest::key uses self.name while GetThenDeleteRequest::key returns self.key which has its own name — matching kind and namespace does NOT imply the name components agree

### φ4: allocate_ids_never_collide
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** a1 allocates twice yielding counter+1, a2 allocates once yielding its counter — if a1.counter+1 == a2.counter they collide, so distinct initial counters do NOT prevent collision after different numbers of allocations

### φ5: update_key_determines_obj_kind
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** key() only constrains obj.kind, namespace, and name — equal keys say nothing about the obj.metadata fields which are independent struct members

