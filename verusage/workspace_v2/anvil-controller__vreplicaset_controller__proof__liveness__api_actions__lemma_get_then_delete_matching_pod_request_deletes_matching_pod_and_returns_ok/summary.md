# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__api_actions__lemma_get_then_delete_matching_pod_request_deletes_matching_pod_and_returns_ok/original.rs`
**Date:** 2026-03-24T05:23:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

One true positive: `CreateRequest::key()` uses `self.namespace` instead of `self.obj.metadata.namespace`, allowing namespace divergence between the key and the embedded object. The remaining three are false positives — Result totality is a tautology, unbounded nat allocation is intentional spec-level modeling, and unmarshal deliberately separates parsing from validation per Kubernetes semantics.

## True Positives (Spec Issues)

### create_key_namespace_from_request_not_obj
- **Confidence:** medium
- **Reasoning:** `CreateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a namespace key that doesn't match its own metadata, which could cause lookup inconsistencies.

## All Candidates

### φ1: marshal_spec_not_injective
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_spec is uninterpreted with no injectivity axiom — if this is provable it means the SMT solver assumes injectivity by default, which would be an unsound implicit axiom

### φ2: unmarshal_spec_total
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** unmarshal_spec is uninterpreted — if the SMT solver can prove totality (always Ok or Err) it's trivially true for Result, but tests whether Verus treats uninterpreted functions as total over their return type
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Result` is an enum with exactly two variants (`Ok` and `Err`), so any value of type `Result` is trivially one or the other. This is a tautology about the return type, not a spec gap.

### φ3: create_key_namespace_from_request_not_obj
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace while the object carries its own metadata.namespace — these can diverge, so the key may not represent the object's actual namespace
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `CreateRequest::key()` derives namespace from `self.namespace` rather than `self.obj.metadata.namespace`. If these diverge, the object is stored under a namespace key that doesn't match its own metadata, which could cause lookup inconsistencies.

### φ4: allocator_monotone_unbounded
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** successive allocations always produce strictly increasing IDs with no upper bound — the spec assumes nat never wraps, modeling infinite resource availability without exhaustion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ReconcileId` is `nat`, which is intentionally unbounded in the spec-level model. Strict monotonicity of successive allocations is the desired design — resource exhaustion is a runtime concern, not a spec-level property.

### φ5: unmarshal_no_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds whenever kind/spec/status parse correctly but never calls state_validation — objects violating domain invariants (e.g. pod with spec == None) can be successfully unmarshalled
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Unmarshal is intentionally a parsing/deserialization step separate from validation. In Kubernetes, the API server applies state validation as a distinct admission phase after unmarshalling.

