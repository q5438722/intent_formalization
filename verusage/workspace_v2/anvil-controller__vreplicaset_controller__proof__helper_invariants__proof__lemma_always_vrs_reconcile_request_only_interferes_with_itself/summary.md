# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__helper_invariants__proof__lemma_always_vrs_reconcile_request_only_interferes_with_itself/original.rs`
**Date:** 2026-03-24T05:18:39Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate is a false positive. The property holds trivially because ReconcileIdAllocator is a single-field struct, so structural inequality directly implies counter inequality and thus distinct allocated IDs.

## All Candidates

### φ1: controller_state_exists_without_model
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** lemma_always_there_is_the_controller_state requires controller_models.contains_key(controller_id) — without that precondition, an unregistered controller should NOT be guaranteed to have state

### φ2: unmarshal_spec_roundtrip_on_arbitrary_value
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** only marshal→unmarshal roundtrip is axiomatized (via external_body), not the reverse direction unmarshal→marshal — uninterpreted functions should not entail this stronger bijectivity

### φ3: cr_states_unmarshallable_for_wrong_reconciler
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** lemma_always_cr_states_are_unmarshallable requires the reconcile_model to match the installed one — without that coupling, arbitrary local states need not be unmarshallable as S

### φ4: marshal_preserves_integrity_implies_unmarshal_total
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** marshal_preserves_integrity is external_body and only guarantees unmarshal(marshal(o)) succeeds — it does NOT make unmarshal total on all DynamicObjectViews with matching kind since unmarshal_spec/unmarshal_status may fail

### φ5: allocate_produces_unique_ids_across_allocators
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** allocate returns the current counter value — two distinct allocators may differ only in other fields or have the same counter, so distinct allocators do NOT imply distinct allocated IDs
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `ReconcileIdAllocator` has only one field (`reconcile_id_counter`), so `a1 != a2` implies `a1.reconcile_id_counter != a2.reconcile_id_counter`. Since `allocate` returns `self.reconcile_id_counter`, distinct allocators necessarily produce distinct IDs.

