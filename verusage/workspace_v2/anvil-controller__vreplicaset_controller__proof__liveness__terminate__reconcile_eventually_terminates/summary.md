# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__terminate__reconcile_eventually_terminates/original.rs`
**Date:** 2026-03-24T05:41:01Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `update_key_ignores_obj_namespace` identifies a real concern where `UpdateRequest::key()` uses the request-level namespace instead of the embedded object's metadata namespace, allowing silent divergence. The other three are false positives: unmarshal correctly separates parsing from validation per Kubernetes design, the uninterpreted marshal_spec non-injectivity observation is vacuous (ensures `true`), and object_ref ignoring uid faithfully models Kubernetes' (kind, namespace, name) identity.

## True Positives (Spec Issues)

### update_key_ignores_obj_namespace
- **Confidence:** medium
- **Reasoning:** `UpdateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key inconsistent with its own metadata, which could cause silent misrouting of updates.

## All Candidates

### φ1: marshal_preserves_integrity_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_preserves_integrity is #[verifier::external_body] with unimplemented!() — the roundtrip axiom is assumed without proof, so any downstream property built on it is trusted, not verified

### φ2: unmarshal_ignores_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds without checking state_validation, so structurally invalid objects can enter the system through deserialization
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is standard Kubernetes design. Unmarshalling is pure deserialization; state validation is performed separately at admission time by the API server, not during parsing.

### φ3: update_key_ignores_obj_namespace
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key uses self.namespace rather than self.obj.metadata.namespace, so the update target can silently disagree with the object's own namespace
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `UpdateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key inconsistent with its own metadata, which could cause silent misrouting of updates.

### φ4: marshal_spec_not_injective
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom, so two distinct specs could serialize to the same Value — the system cannot distinguish different configurations after storage
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This candidate merely demonstrates that `marshal_spec` being `uninterp` means Verus doesn't *assert* injectivity — but neither does it assert non-injectivity. The precondition `s1 != s2 && marshal_spec(s1) == marshal_spec(s2)` is satisfiable only in the SMT solver's abstract model; the ensures is trivially `true`. This proves nothing about an actual spec gap.

### φ5: object_ref_ignores_uid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** object_ref uses only kind/name/namespace and ignores uid, so a deleted-and-recreated resource with a new uid maps to the same ObjectRef — stale reconciliation could target the wrong instance
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This mirrors Kubernetes' actual design where resource identity is (kind, namespace, name). UID distinguishes incarnations but is intentionally excluded from the lookup key — the API server and all controllers use this same identity model.

