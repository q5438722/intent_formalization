# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__helper_invariants/vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_no_other_pending_request_interferes_with_vrs_reconcile.rs`
**Date:** 2026-03-24T02:46:10Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: unmarshal_status_total
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** unmarshal_status is uninterpreted — no axiom makes it total, so it should be possible for arbitrary Values to fail unmarshalling

### φ2: marshal_spec_constant
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec is uninterpreted — if it were provably constant, all specs would marshal to the same Value, destroying information and contradicting the roundtrip property

### φ3: create_request_key_ignores_obj_kind
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** CreateRequest::key includes obj.kind — requests with same name/namespace but different kinds should produce different ObjectRefs

### φ4: allocate_returns_zero_initially
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** allocate returns the current counter — if the counter is already positive, the returned ID must equal that positive value, not zero

### φ5: create_update_keys_agree_on_same_obj
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** CreateRequest::key uses obj.metadata.name while UpdateRequest::key uses self.name — even with the same obj and namespace, name fields are sourced differently and need not agree

