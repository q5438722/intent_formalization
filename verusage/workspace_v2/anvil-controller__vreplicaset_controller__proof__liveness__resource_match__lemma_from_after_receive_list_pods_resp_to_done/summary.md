# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__resource_match__lemma_from_after_receive_list_pods_resp_to_done/original.rs`
**Date:** 2026-03-24T05:28:39Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 4

## Summary

Of the 5 candidates across the two files, 1 is a true positive: `create_key_namespace_diverges` highlights that `CreateRequest::key()` uses `self.namespace` instead of `self.obj.metadata.namespace`, allowing key-object namespace mismatches. The remaining 4 are false positives: `kernel_mem_end_l4_allows_user_at_zero` reflects an intentional kernel memory layout; `page_index2ptr_overflow` is safe because the arithmetic fits in 64-bit usize; `unmarshal_skips_state_validation` reflects the deliberate Kubernetes separation of parsing from admission validation; and `object_ref_ignores_uid` matches Kubernetes's standard (kind, name, namespace) identification scheme.

## True Positives (Spec Issues)

### create_key_namespace_diverges
- **Confidence:** medium
- **Reasoning:** `CreateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key that disagrees with the object's own metadata, potentially placing the object in the wrong namespace.

## All Candidates

### φ1: unmarshal_roundtrip_no_axiom
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec/unmarshal_spec are uninterpreted with no roundtrip axiom at the ResourceView level — marshal_preserves_integrity exists only on Marshallable, not ResourceView, so if provable the solver assumes an unjustified bijection

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
- **Reasoning:** `CreateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a key that disagrees with the object's own metadata, potentially placing the object in the wrong namespace.

### φ5: object_ref_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** object_ref is (kind, name, namespace) ignoring uid — two distinct resource incarnations with different uids produce identical ObjectRefs conflating identity across delete-recreate cycles
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ObjectRef` is intentionally defined as (kind, name, namespace) — this is Kubernetes's standard resource identification scheme. UID is a separate identity concept used for ownership tracking and garbage collection, not for resource lookup keys.

