# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__spec__assumption_and_invariants_of_all_phases_is_stable/original.rs`
**Date:** 2026-03-24T05:34:42Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

Of the 3 candidates, 1 is a true positive: `create_key_namespace_diverges` highlights that `CreateRequest::key()` uses `self.namespace` instead of `self.obj.metadata.namespace`, allowing the storage key to diverge from the object's own metadata. The other 2 are false positives — `unmarshal_skips_state_validation` reflects the intentional separation of deserialization from admission validation, and `object_ref_ignores_uid` reflects the standard Kubernetes design where `ObjectRef` is a namespace-scoped name key, not a UID-based incarnation identifier.

## True Positives (Spec Issues)

### create_key_namespace_diverges
- **Confidence:** medium
- **Reasoning:** `CreateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key that doesn't match its own metadata, which could lead to inconsistent lookups or stale references.

## All Candidates

### φ1: unmarshal_roundtrip_no_axiom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec/unmarshal_spec are uninterpreted with no roundtrip axiom — if provable the solver assumes an unjustified bijection between typed specs and serialized Values

### φ2: marshal_spec_injective
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec is uninterpreted with no declared injectivity — if provable the solver implicitly assumes injectivity stronger than what the spec provides

### φ3: unmarshal_skips_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds when kind/spec/status parse correctly but never checks state_validation — semantically invalid objects can be successfully unmarshalled
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Unmarshal is intentionally a parsing/deserialization step separate from validation. In Kubernetes, state validation is performed at admission time, not during unmarshalling — this separation is by design.

### φ4: create_key_namespace_diverges
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace not self.obj.metadata.namespace — these can diverge so the key does not faithfully represent the object's actual namespace
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `CreateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key that doesn't match its own metadata, which could lead to inconsistent lookups or stale references.

### φ5: object_ref_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** object_ref is (kind, name, namespace) ignoring uid — two distinct resource incarnations with different uids produce identical ObjectRefs conflating identity across delete-recreate cycles
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ObjectRef` is intentionally defined as (kind, name, namespace) to serve as a logical key for resource lookup. UID-based identity tracking is handled separately via `metadata.uid` and owner references — `ObjectRef` is a namespace-scoped name, not a unique incarnation identifier.

