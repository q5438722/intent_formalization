# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__liveness__api_actions__lemma_create_matching_pod_request_adds_matching_pod_and_returns_ok/original.rs`
**Date:** 2026-03-24T05:23:26Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 0

## True Positives (Spec Issues)

### marshal_preserves_integrity_external_body
- **Confidence:** high
- **Reasoning:** `marshal_preserves_integrity` is `#[verifier::external_body]` across all ResourceView implementors via the macro. The roundtrip property over uninterpreted `marshal_spec`/`unmarshal_spec` is entirely unverified — a real soundness gap.

## All Candidates

### φ1: marshal_preserves_integrity_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** marshal_preserves_integrity is #[verifier::external_body] — it's trusted without proof, so this axiom could be unsound if marshal_spec/unmarshal_spec don't actually roundtrip (they're uninterpreted)
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `marshal_preserves_integrity` is `#[verifier::external_body]` across all ResourceView implementors via the macro. The roundtrip property over uninterpreted `marshal_spec`/`unmarshal_spec` is entirely unverified — a real soundness gap.

### φ2: submap_finite_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** a_submap_of_a_finite_map_is_finite is #[verifier::external_body] — trusted without proof, so if vstd's Map axioms don't actually guarantee this property it could be a soundness gap

### φ3: create_key_namespace_disagrees_with_obj
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace while the obj carries its own metadata.namespace — these can diverge, meaning the key does not faithfully represent the object's actual namespace

### φ4: unmarshal_ignores_state_validation
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** unmarshal succeeds whenever kind/spec/status parse correctly but never checks state_validation — invalid objects (e.g. pods with spec == None) can be successfully unmarshalled

### φ5: object_ref_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** object_ref is (kind, name, namespace) without uid — two distinct resource incarnations with different uids produce identical ObjectRefs, conflating identity across deletions and recreations

