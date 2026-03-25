# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__liveness/vreplicaset_controller__proof__liveness__spec__invariant_since_phase_i_is_stable.rs`
**Date:** 2026-03-24T03:18:37Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

Of three verified candidates, only `stable_always_external_body_unsound` is a true positive — it demonstrates that the `#[verifier::external_body]` annotations on temporal logic lemmas (`always_p_is_stable`, `stable_and_temp`) create an unverified trust boundary. While the mathematical claims are correct, the external_body mechanism means any implementation error would silently propagate unsoundness through the entire stability proof chain. The other two candidates are false positives: `crash_disabled_ignores_other_controllers` has a tautological ensures clause proving nothing, and `object_ref_collision_in_schedule` reflects intentional Kubernetes API semantics where resources are uniquely identified by kind+name+namespace.

## True Positives (Spec Issues)

### stable_always_external_body_unsound
- **Confidence:** high
- **Reasoning:** This is a real soundness gap. `always_p_is_stable` with `#[verifier::external_body]` lets you prove `valid(stable(always(lift_state(|s| false))))` — that falsehood is stably always true. While the lemma's postcondition is mathematically correct (always(p) is indeed stable for any p), the external_body means Verus trusts this without checking, and the same pattern applies to `stable_and_temp`. Any bug in these assumed lemmas would silently compromise the entire temporal logic proof chain.

## All Candidates

### φ1: crash_disabled_ignores_other_controllers
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** crash_disabled only constrains a single controller_id — other controllers can still crash freely, meaning the "failure-free" assumption is only local and the system can still lose state from other controller crashes
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures clause is a tautology (`P || !P`), so this proves nothing about the spec. The observation that crash_disabled is per-controller is intentional — the liveness proof's `invariants_since_phase_i` only needs to assume the specific controller doesn't crash, matching how Kubernetes controllers operate independently.

### φ2: schedule_spec_match_ignores_metadata_diff
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** the_object_in_schedule_has_spec_and_uid_as only constrains spec and uid — labels, annotations, resource_version, and other metadata fields can differ between the scheduled object and the reference vrs, so reconciliation may operate on stale metadata

### φ3: stable_always_external_body_unsound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_p_is_stable has #[verifier::external_body] so its postcondition is assumed without proof — if the implementation is incorrect, any temporal property wrapped in always() can be declared stable, enabling unsound stability chains
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This is a real soundness gap. `always_p_is_stable` with `#[verifier::external_body]` lets you prove `valid(stable(always(lift_state(|s| false))))` — that falsehood is stably always true. While the lemma's postcondition is mathematically correct (always(p) is indeed stable for any p), the external_body means Verus trusts this without checking, and the same pattern applies to `stable_and_temp`. Any bug in these assumed lemmas would silently compromise the entire temporal logic proof chain.

### φ4: invariants_phase_i_no_etcd_constraint
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** invariants_since_phase_i constrains failure modes and schedule consistency but places zero constraints on etcd contents — the resource store can be arbitrarily mutated by other controllers or API server transitions while these invariants hold

### φ5: object_ref_collision_in_schedule
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** two VReplicaSetViews with different uids and different specs produce the same object_ref (kind+name+namespace), so the scheduled_reconciles map can hold at most one — if a delete-recreate occurs, the schedule silently loses the old incarnation's entry
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This mirrors real Kubernetes semantics where resources are identified by (kind, name, namespace) — the API server enforces uniqueness at this level, so two live objects with the same kind/name/namespace cannot coexist. UID distinguishes incarnations across delete-recreate, but the scheduling map keyed by ObjectRef correctly reflects that at any point in time, only one incarnation exists for a given key. The `the_object_in_schedule_has_spec_and_uid_as` predicate additionally checks uid to ensure the scheduled object matches the expected incarnation.

