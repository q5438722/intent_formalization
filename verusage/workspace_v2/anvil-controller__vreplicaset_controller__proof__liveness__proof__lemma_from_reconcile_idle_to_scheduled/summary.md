# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__proof__lemma_from_reconcile_idle_to_scheduled/original.rs`
**Date:** 2026-03-24T05:25:44Z
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
- **Why flagged:** marshal_spec/unmarshal_spec are uninterpreted with no roundtrip axiom — if provable the SMT solver is assuming an unjustified bijection between spec values and their serialized form

### φ2: marshal_spec_injective
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom — if the solver proves this it reveals an implicit over-strong assumption about uninterpreted functions

### φ3: unmarshal_skips_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds whenever kind/spec/status parse but never checks state_validation — invalid objects (e.g. pods with spec == None) can be successfully unmarshalled

### φ4: update_key_kind_from_obj_body
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.namespace not self.obj.metadata.namespace — these can diverge so the key does not faithfully represent the object's actual namespace

### φ5: object_ref_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** object_ref is (kind, name, namespace) ignoring uid — two distinct resource incarnations with different uids produce identical ObjectRefs, conflating identity across delete-recreate cycles

