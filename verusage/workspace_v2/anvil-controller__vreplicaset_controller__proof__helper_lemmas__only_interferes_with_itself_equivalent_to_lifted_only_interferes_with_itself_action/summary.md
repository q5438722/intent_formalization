# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__helper_lemmas/vreplicaset_controller__proof__helper_lemmas__only_interferes_with_itself_equivalent_to_lifted_only_interferes_with_itself_action.rs`
**Date:** 2026-03-24T02:50:29Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 4
- False positives: 1

## Summary

Four of five candidates are true positives revealing a systematic spec weakness: the identity and interference model is name-based (via ObjectRef) rather than uid-based. ObjectRef ignores uid entirely (φ5), which propagates into both the create and delete interference checks (φ2, φ3) that cannot distinguish between different incarnations of a VReplicaSet with the same name. The CreateRequest key namespace divergence (φ1) is a separate but real concern about internal consistency between request fields. The only false positive is the stuttering step property (φ4), which is expected TLA+-style behavior. The core finding is that the spec's name-based identity model creates a potential soundness gap when resources are deleted and recreated with the same name but different uids.

## True Positives (Spec Issues)

### create_key_namespace_disagrees_with_obj
- **Confidence:** medium
- **Reasoning:** CreateRequest::key uses self.namespace rather than self.obj.metadata.namespace, so these can diverge. This means the key used for storage lookups may not match the object's own namespace metadata, which is a real inconsistency risk. However, in practice the API server likely normalizes this — the concern is that the spec doesn't enforce agreement.

### interference_check_ignores_uid
- **Confidence:** high
- **Reasoning:** The interference predicate matches on owner_ref.name and kind but never checks uid. In Kubernetes, a resource can be deleted and recreated with the same name but a different uid, making uid the true identity. This means the interference check cannot distinguish between the old and new VReplicaSet, which is a genuine spec weakness.

### create_interference_ignores_uid
- **Confidence:** high
- **Reasoning:** Same root cause as the delete case — the create interference predicate checks owner_ref.name but not owner_ref.uid. A recreated VRS with the same name would pass the interference check for a different VRS instance, allowing cross-generation interference. This is the same uid-blindness issue manifesting in the create path.

### object_ref_name_eq_ignores_uid
- **Confidence:** high
- **Reasoning:** ObjectRef is defined as (kind, name, namespace) without uid, making it unable to distinguish between different incarnations of a resource with the same name. This is the structural root cause behind the interference check issues — the spec's identity model is name-based rather than uid-based, which conflates distinct resource instances throughout the verification.

## All Candidates

### φ1: create_key_namespace_disagrees_with_obj
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** CreateRequest::key uses self.namespace while the obj carries its own metadata.namespace — if these can differ, the key does not faithfully represent the object's actual namespace
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** CreateRequest::key uses self.namespace rather than self.obj.metadata.namespace, so these can diverge. This means the key used for storage lookups may not match the object's own namespace metadata, which is a real inconsistency risk. However, in practice the API server likely normalizes this — the concern is that the spec doesn't enforce agreement.

### φ2: interference_check_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** the interference predicate checks owner_ref.name and kind but not uid — two distinct VReplicaSets with the same name/namespace would satisfy each other's interference check, breaking isolation
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The interference predicate matches on owner_ref.name and kind but never checks uid. In Kubernetes, a resource can be deleted and recreated with the same name but a different uid, making uid the true identity. This means the interference check cannot distinguish between the old and new VReplicaSet, which is a genuine spec weakness.

### φ3: create_interference_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** like the delete case, create interference checks owner_ref.name but not uid — a recreated VRS with the same name but different uid would pass the check, allowing cross-instance interference
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Same root cause as the delete case — the create interference predicate checks owner_ref.name but not owner_ref.uid. A recreated VRS with the same name would pass the interference check for a different VRS instance, allowing cross-generation interference. This is the same uid-blindness issue manifesting in the create path.

### φ4: lifted_action_trivially_holds_on_stuttering
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** lifted_vrs checks the state predicate on both s and s_prime independently — on a stuttering execution this is trivially satisfied, meaning the action formulation adds no transition-specific constraint beyond the state invariant
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** In TLA+-style specifications, action predicates are expected to hold on stuttering steps (where the state doesn't change). The lifted action checks the state invariant on both s and s_prime independently, which is standard — stuttering steps preserving invariants is a fundamental TLA+ property, not a weakness.

### φ5: object_ref_name_eq_ignores_uid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** object_ref is determined solely by kind/name/namespace and ignores uid — two distinct resources (different uid) with the same name produce identical ObjectRefs, which can conflate identities throughout the spec
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** ObjectRef is defined as (kind, name, namespace) without uid, making it unable to distinguish between different incarnations of a resource with the same name. This is the structural root cause behind the interference check issues — the spec's identity model is name-based rather than uid-based, which conflates distinct resource instances throughout the verification.

