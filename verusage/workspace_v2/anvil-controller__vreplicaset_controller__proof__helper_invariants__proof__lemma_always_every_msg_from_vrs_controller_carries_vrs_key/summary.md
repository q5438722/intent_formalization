# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__helper_invariants/vreplicaset_controller__proof__helper_invariants__proof__lemma_always_every_msg_from_vrs_controller_carries_vrs_key.rs`
**Date:** 2026-03-24T02:42:30Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: unmarshal_spec_total
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** unmarshal_spec is uninterpreted — there is no axiom making it total, so arbitrary Values should be rejectable

### φ2: marshal_status_unmarshal_spec_compatible
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_status and unmarshal_spec operate on unrelated domains — a marshalled status Value should NOT be decodable as a valid spec

### φ3: allocate_idempotent
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** allocate increments reconcile_id_counter by 1, so the returned allocator must differ from the original — idempotence would mean IDs are never consumed

### φ4: object_ref_ignores_namespace
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** key() includes namespace — two requests with different namespaces but same name should produce different ObjectRefs, not equal ones

### φ5: marshal_roundtrip_preserves_kind_field
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec is uninterpreted so marshal(unmarshal(obj)).spec = marshal_spec(unmarshal_spec(obj.spec)) which need not equal obj.spec — only the reverse roundtrip (unmarshal∘marshal = id) is guaranteed

