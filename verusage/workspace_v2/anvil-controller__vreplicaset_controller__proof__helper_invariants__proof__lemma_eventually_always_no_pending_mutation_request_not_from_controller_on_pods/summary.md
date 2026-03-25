# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v2/anvil-controller__vreplicaset_controller__proof__helper_invariants__proof__lemma_eventually_always_no_pending_mutation_request_not_from_controller_on_pods/original.rs`
**Date:** 2026-03-24T05:19:21Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate is a false positive. The property correctly reflects the initial-state semantics: with zero allocated RPC IDs, no messages can be in flight. This is the intended behavior of the monotonic ID allocator combined with the in-flight message invariant.

## All Candidates

### φ1: unmarshal_total_for_any_value
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** no marshal_preserves_integrity proof exists in this version — unmarshal_spec/unmarshal_status are uninterpreted with zero axioms, so matching kind alone cannot guarantee successful unmarshalling

### φ2: in_flight_unique_ids_implies_singleton
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** unique IDs imply msg1==msg2 but NOT that the multiset count is 1 — a Multiset can contain duplicate entries of the same message, and the spec only constrains distinct messages

### φ3: crash_disabled_implies_controller_exists
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** crash_disabled indexes into controller_and_externals without a contains_key guard — satisfying crash_disabled should NOT independently prove the controller exists (it would require the precondition or trigger an out-of-domain access)

### φ4: allocator_bound_implies_empty_flight
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** if counter is 0 then all messages need rpc_id < 0 (negative) — this would mean no messages can be in flight, which is only valid if rpc_id is int; if the system allows counter=0 as initial state this proves emptiness which may be too strong
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `RPCId` is `int` and `rpc_id_counter` starts at 0 in the initial state. The invariant `msg.rpc_id < s.rpc_id_allocator.rpc_id_counter` with counter==0 correctly implies no messages in flight, which is exactly the expected initial condition — no messages have been allocated yet.

### φ5: req_drop_and_pod_monkey_independent
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** req_drop_enabled and pod_monkey_enabled are independent boolean fields on ClusterState — disabling request dropping should NOT entail that the pod monkey is also disabled

