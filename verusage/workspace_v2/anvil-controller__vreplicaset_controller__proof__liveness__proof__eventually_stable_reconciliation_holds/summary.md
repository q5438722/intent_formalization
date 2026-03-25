# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__proof__eventually_stable_reconciliation_holds/original.rs`
**Date:** 2026-03-24T05:24:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 0

## All Candidates

### φ1: unmarshal_roundtrip_no_axiom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec/unmarshal_spec are uninterpreted with no roundtrip axiom — if the SMT solver can prove this without an explicit axiom, it reveals an accidental entailment from the uninterpreted function encoding

### φ2: marshal_spec_injective
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom — if the solver assumes injectivity it would be an implicit over-strong assumption that could mask spec bugs

### φ3: unmarshal_always_succeeds_matching_kind
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** unmarshal checks unmarshal_spec and unmarshal_status which are uninterpreted — if this is provable it means the solver assumes these never fail, making unmarshal vacuously total for matching kinds

### φ4: update_key_obj_kind_mismatch
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.namespace not self.obj.metadata.namespace — if these diverge the key does not faithfully represent the object's actual namespace

### φ5: object_ref_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** object_ref is (kind, name, namespace) ignoring uid — two distinct resource incarnations with different uids produce identical ObjectRefs, conflating identity across deletions and recreations

