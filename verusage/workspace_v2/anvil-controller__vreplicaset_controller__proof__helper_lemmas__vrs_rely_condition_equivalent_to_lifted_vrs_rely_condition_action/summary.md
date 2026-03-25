# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__helper_lemmas/vreplicaset_controller__proof__helper_lemmas__vrs_rely_condition_equivalent_to_lifted_vrs_rely_condition_action.rs`
**Date:** 2026-03-24T02:53:37Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

Three of the four candidates are false positives reflecting correct rely-guarantee design patterns: the action formulation correctly checks state predicates on both pre/post states, the rely intentionally excludes the controller's own model, and an empty cluster correctly has a vacuous rely. The one true positive (φ2) identifies that the create rely permits pods with no owner references, which could be subsequently adopted by a VRS — this is a gap in the create-time interference check that may allow indirect ownership transfer if pod adoption is not separately guarded.

## True Positives (Spec Issues)

### rely_create_allows_none_owner_refs
- **Confidence:** medium
- **Reasoning:** The rely checks whether the created pod carries a VRS controller owner reference, but when owner_references is None, the negated conjunction is trivially satisfied. This means other controllers can freely create ownerless pods that a VRS could later adopt, bypassing the create-time interference protection. Whether this is intentional depends on whether pod adoption is modeled elsewhere, but the rely alone does not prevent it.

## All Candidates

### φ1: lifted_action_no_transition_constraint
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** the "action" formulation checks the rely on s and s_prime independently with no relational constraint between them — it adds nothing beyond two state predicates and cannot express transition-specific interference properties
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is standard TLA+-style encoding. The rely condition is a state predicate (constraining in-flight messages), not a transition relation. Checking it on both s and s_prime independently is the correct way to express that the rely holds before and after each step, which is exactly what always(lift_state(...)) means. The "action" wrapper is just a packaging convenience for the equivalence proof.

### φ2: rely_create_allows_none_owner_refs
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** a controller can create pods with no owner references at all and still satisfy the rely — these orphan pods could later be adopted by a VRS, bypassing the create-time interference check entirely
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** The rely checks whether the created pod carries a VRS controller owner reference, but when owner_references is None, the negated conjunction is trivially satisfied. This means other controllers can freely create ownerless pods that a VRS could later adopt, bypassing the create-time interference protection. Whether this is intentional depends on whether pod adoption is modeled elsewhere, but the rely alone does not prevent it.

### φ3: rely_symmetric_across_controllers
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** the lifted rely for controller id1 universally quantifies over all other_id in the model map — since id2 != id1, the rely for id1 checks vrs_rely(id2), but since id1 is also in the map minus id2, controller id2's lifted rely would check vrs_rely(id1) — this means each controller must satisfy the VRS rely even if it is not a VRS controller

### φ4: equivalence_proof_ignores_cluster_param
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** the lifted rely only uses cluster.controller_models.remove(controller_id) — two clusters that differ only in the VRS controller's own model entry produce identical rely conditions, meaning the VRS controller's own model is entirely unconstrained by the rely
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is by design in rely-guarantee reasoning. The rely constrains *other* controllers' behavior, not the VRS controller itself. The VRS controller's own model entry is irrelevant to the rely condition — its behavior is constrained by the guarantee side, not the rely side. Two clusters differing only in the VRS controller's own model correctly produce identical rely conditions.

### φ5: empty_cluster_rely_trivially_true
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** with an empty controller_models map, remove(controller_id) is also empty, making the forall vacuously true — the rely imposes zero constraints in a cluster with no registered controllers, which may mask interference from unmodeled components
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With no other controllers registered, there are no other controllers to constrain, so a vacuously true rely is correct. The rely-guarantee framework only constrains registered controller models; unmodeled components are outside its scope by design. A cluster with zero controllers has no interference sources to guard against.

