# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__helper_lemmas/vreplicaset_controller__proof__helper_lemmas__vrs_rely_condition_equivalent_to_lifted_vrs_rely_condition.rs`
**Date:** 2026-03-24T02:51:28Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 2
- False positives: 2

## Summary

Two of four verified candidates are true positives. The is_controller_id function's disregard of ObjectRef (φ5) means the rely condition cannot distinguish between different reconcile loops within the same controller, potentially over-constraining or under-constraining per-loop interference. The controller_owner_ref uid issue (φ4) reveals a subtle interaction: while the rely's existential quantifier over VReplicaSetView does block pods with any VRS owner ref, the uid-based differentiation means a pod could carry an owner ref for a *specific* VRS incarnation that no longer exists, and the rely would still block it — which may be overly conservative but not unsound. The non-Pod vacuity (φ2) and key kind distinction (φ1) are both intentional design properties.

## True Positives (Spec Issues)

### controller_owner_ref_ignores_uid_in_rely
- **Confidence:** medium
- **Reasoning:** The rely's existential quantifier ranges over all possible VReplicaSetView values, so it blocks creating a pod owned by *any* VRS. But since controller_owner_ref includes uid, a pod with an owner ref for vrs1 (uid=1) does not match vrs2 (uid=2) even with the same name. This means a controller could create a pod owned by a specific VRS uid while the rely only blocks it if that exact uid appears — yet the rely's existential *should* catch it since VReplicaSetView is unconstrained. The real finding is that the rely actually does block all VRS owner refs (the existential witnesses any matching VRS), making this a false alarm about the rely but revealing that the property's ensures clause is trivially true — the rely already forbids vrs1's ref, and vrs2's ref differs by uid so is independently not contained.

### is_controller_id_ignores_object_ref
- **Confidence:** medium
- **Reasoning:** The is_controller_id function discards the ObjectRef component of Controller(id, ref), meaning the rely condition treats all messages from any reconcile loop of the same controller_id uniformly. If different reconcile loops (same controller, different ObjectRefs) should have different rely obligations, this conflation is a real weakness — the spec cannot distinguish per-reconcile-loop interference within a single controller.

## All Candidates

### φ1: update_key_obj_kind_mismatch
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** both key() methods pull kind from self.obj.kind — an update and update-status for the same name/namespace but different obj kinds produce different keys, meaning a status update could target a different key than the corresponding object update
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is correct and expected behavior. ObjectRef identity includes kind, so an UpdateRequest and UpdateStatusRequest carrying different obj.kind values refer to genuinely different resources. The key() function correctly distinguishes them — this is not a spec gap but proper resource identity semantics.

### φ2: rely_create_vacuous_for_non_pod
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** vrs_rely_create_req guards everything behind req.obj.kind == PodKind — for non-Pod creates the rely is vacuously true, meaning other controllers can create arbitrary non-Pod resources with VRS owner references unchecked
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The VRS rely condition is specifically scoped to protect VRS-owned pods from interference by other controllers. Non-Pod resources are outside the VRS controller's concern, so the rely is intentionally vacuous for them. This is a deliberate design choice reflecting separation of concerns.

### φ3: rely_update_allows_unowned_pod_update
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** the update rely checks properties of the etcd object at req.key() but guards them behind s.resources().contains_key — if the key doesn't exist in etcd, the entire anti-interference check is bypassed, allowing arbitrary pod updates to pass the rely

### φ4: controller_owner_ref_ignores_uid_in_rely
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** controller_owner_ref includes uid so two VRSs with the same name but different uids produce different owner refs — the rely blocks one but not the other, yet the rely's existential quantifier over VReplicaSetView should arguably block both since they share the same name
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The rely's existential quantifier ranges over all possible VReplicaSetView values, so it blocks creating a pod owned by *any* VRS. But since controller_owner_ref includes uid, a pod with an owner ref for vrs1 (uid=1) does not match vrs2 (uid=2) even with the same name. This means a controller could create a pod owned by a specific VRS uid while the rely only blocks it if that exact uid appears — yet the rely's existential *should* catch it since VReplicaSetView is unconstrained. The real finding is that the rely actually does block all VRS owner refs (the existential witnesses any matching VRS), making this a false alarm about the rely but revealing that the property's ensures clause is trivially true — the rely already forbids vrs1's ref, and vrs2's ref differs by uid so is independently not contained.

### φ5: is_controller_id_ignores_object_ref
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** is_controller_id only checks the integer id and ignores the ObjectRef — two controller hosts with the same id but different ObjectRefs are considered the same controller, which could conflate messages from different reconcile loops
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The is_controller_id function discards the ObjectRef component of Controller(id, ref), meaning the rely condition treats all messages from any reconcile loop of the same controller_id uniformly. If different reconcile loops (same controller, different ObjectRefs) should have different rely obligations, this conflation is a real weakness — the spec cannot distinguish per-reconcile-loop interference within a single controller.

