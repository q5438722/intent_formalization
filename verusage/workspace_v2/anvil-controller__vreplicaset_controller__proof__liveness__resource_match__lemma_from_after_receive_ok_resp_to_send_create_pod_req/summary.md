# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_ok_resp_to_send_create_pod_req/original.rs`
**Date:** 2026-03-24T05:31:07Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 0

## All Candidates

### φ1: unmarshal_roundtrip_no_axiom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_preserves_integrity exists on Marshallable but ResourceView's marshal_spec/unmarshal_spec are uninterpreted with no analogous roundtrip axiom — if provable the solver assumes an unjustified bijection

### φ2: marshal_spec_injective
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec is uninterpreted with no declared injectivity — if provable the solver implicitly assumes injectivity stronger than what the spec provides

### φ3: unmarshal_skips_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds when kind/spec/status parse correctly but never checks state_validation — semantically invalid objects can be successfully unmarshalled

### φ4: create_key_namespace_diverges
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace not self.obj.metadata.namespace — these can diverge so the key does not faithfully represent the object's actual namespace

### φ5: object_ref_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** object_ref is (kind, name, namespace) ignoring uid — two distinct resource incarnations with different uids produce identical ObjectRefs conflating identity across delete-recreate cycles

