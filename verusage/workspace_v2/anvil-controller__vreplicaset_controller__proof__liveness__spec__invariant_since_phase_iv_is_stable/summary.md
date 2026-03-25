# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-controller/verified/vreplicaset_controller__proof__liveness/vreplicaset_controller__proof__liveness__spec__invariant_since_phase_iv_is_stable.rs`
**Date:** 2026-03-24T03:21:46Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

Three of the four candidates are false positives. The GC invariant allowing non-delete mutations (φ1) is a tautology with `ensures true` that doesn't reveal a real gap — other invariants handle update constraints. The controller_owner_ref namespace omission (φ3) and owner_references_contains None behavior (φ4) both faithfully model real Kubernetes semantics. The only true positive is the external_body soundness concern (φ5): `always_p_is_stable` is mathematically correct but unverified by Verus, creating a trusted assumption at the foundation of all temporal stability proofs in the system.

## True Positives (Spec Issues)

### always_p_is_stable_external_body_unsound
- **Confidence:** high
- **Reasoning:** The `#[verifier::external_body]` annotation means the lemma's postcondition `valid(stable(always(p)))` is assumed for any `p` without proof. While mathematically correct (always(p) is indeed stable for any p — if always(p) holds at some point, it holds at all future points), the external_body means this correctness is trusted, not verified. Any bug in this reasoning would silently compromise all temporal stability proofs built on it.

## All Candidates

### φ1: gc_invariant_allows_non_delete_mutations
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** the garbage collector invariant only constrains delete requests — the builtin controller can freely send update requests that modify or remove owner references on VRS-owned pods, bypassing the protection entirely
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The invariant is specifically about the garbage collector's delete behavior, as its name states. Update requests from the builtin controller are constrained by other invariants elsewhere in the codebase (e.g., no_pending_interfering_update_request from phase III). The ensures clause is literally `true`, making this a tautology that demonstrates nothing about the spec.

### φ2: gc_invariant_vacuous_for_non_builtin
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** the invariant only restricts messages from BuiltinController — PodMonkey or other sources can freely delete VRS-owned pods without any constraint

### φ3: controller_owner_ref_ignores_namespace
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** controller_owner_ref uses only name and uid but not namespace — two VReplicaSets in different namespaces with the same name and uid produce identical owner references, conflating cross-namespace ownership
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This faithfully models real Kubernetes OwnerReference semantics — owner references contain kind, name, and uid but not namespace, because ownership is always within the same namespace. UID provides global uniqueness across the cluster, so two objects with the same uid are the same object regardless of namespace.

### φ4: owner_references_contains_ignores_none
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** owner_references_contains returns false for None owner_references rather than failing — an object with missing owner_references silently appears unowned, which could cause the GC invariant to not protect pods whose metadata is incomplete
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Returning false for None owner_references is correct — an object with no owner references list does not contain any owner reference. This is standard Option handling. The GC invariant's protection correctly excludes such objects since they are genuinely unowned.

### φ5: always_p_is_stable_external_body_unsound
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** always_p_is_stable has #[verifier::external_body] so its postcondition is assumed without proof — it can be instantiated with any predicate including falsehood, declaring that "always false" is stable
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The `#[verifier::external_body]` annotation means the lemma's postcondition `valid(stable(always(p)))` is assumed for any `p` without proof. While mathematically correct (always(p) is indeed stable for any p — if always(p) holds at some point, it holds at all future points), the external_body means this correctness is trusted, not verified. Any bug in this reasoning would silently compromise all temporal stability proofs built on it.

