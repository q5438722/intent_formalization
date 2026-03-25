# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__helper_invariants/vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_no_pending_interfering_update_request.rs`
**Date:** 2026-03-24T05:15:53Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `marshal_preserves_integrity` is `external_body` across all ResourceView macro implementations, leaving the marshal/unmarshal roundtrip entirely unverified. The remaining four candidates are false positives reflecting intentional Kubernetes API modeling decisions (key derivation from object, nat-based IDs, name-based ObjectRef identity).

## True Positives (Spec Issues)

### marshal_preserves_integrity_external_body
- **Confidence:** high
- **Reasoning:** `marshal_preserves_integrity` is `external_body` across all ResourceView implementors via the macro. The marshal/unmarshal roundtrip is unverified — a real soundness gap.

## All Candidates

### φ1: marshal_preserves_integrity_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_preserves_integrity is external_body — unverified roundtrip assumption for all ResourceView implementors
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `marshal_preserves_integrity` is `external_body` across all ResourceView implementors via the macro. The marshal/unmarshal roundtrip is unverified — a real soundness gap.

### φ2: update_request_key_kind_from_obj
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** UpdateRequest::key() takes kind from obj rather than a separate field — a mismatched obj.kind silently changes the key's kind
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the intentional definition of `UpdateRequest::key()` — it derives the key from the embedded object. This is standard Kubernetes semantics where the request's identity comes from the object being updated.

### φ3: update_status_request_key_kind_from_obj
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Same issue as UpdateRequest — UpdateStatusRequest::key() derives kind from obj instead of independent field
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Same as above — intentional Kubernetes API design where the object carries its own kind.

### φ4: reconcile_id_allocator_monotonic_unbounded
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** ReconcileId is nat with no upper bound — allocator can grow unboundedly with no overflow or exhaustion check
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ReconcileId` is `nat` (mathematical natural number), so unbounded growth is by design — no overflow is possible. This is standard for spec-level modeling.

### φ5: object_ref_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** ObjectRef identity ignores UID — two objects with different UIDs but same name/namespace/kind are indistinguishable by ObjectRef
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This mirrors real Kubernetes semantics where ObjectRef identifies resources by name/namespace/kind, not UID. UID distinguishes incarnations, not identity. Intentional design.

