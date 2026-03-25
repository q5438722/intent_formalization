# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__host_impl_v__impl2__host_model_next_set_request/original.rs`
**Date:** 2026-03-24T21:51:00Z

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 1
- False positives: 0

## Summary

The single candidate property exposes a real spec issue in `next_set_request`. The function's specification unconditionally allows (or requires) modification of `received_requests` regardless of whether the host owns the key, which conflates two semantically distinct paths: local processing and redirection. In a correctly specified sharded key-value protocol, a host redirecting a request to another host should not record that request as "received," since it was never processed locally. This is a meaningful soundness concern for any refinement proof that relies on `received_requests` accurately reflecting the set of requests a host has actually serviced.

## True Positives

### redirect_modifies_received_requests
- **Confidence:** medium
- **Reasoning:** In a sharded key-value store, when a host receives a `SetRequest` for a key it doesn't own (`delegation_map[k] != me`), the correct behavior is to redirect the request without recording it locally. The verified property proves that `next_set_request` mutates `received_requests` even in the redirect case, meaning the spec fails to distinguish between locally-processed requests and redirected ones—logging phantom requests the host never actually handled. This is a genuine spec weakness that could cause incorrect accounting in refinement proofs against the abstract distributed protocol.

## All Candidates

### φ1: local_set_no_hashmap_update → `next_set_request_complete`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the hashmap is unchanged after a local SetRequest with Some(v), the key-value store silently drops writes

### φ2: none_value_no_remove → `next_set_request_complete`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the key persists after a SetRequest with None value, delete operations are broken and stale data remains

### φ3: send_success_no_state_change → `send_single_message`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If state is unchanged after a successful send, the un_acked queue never grows and retransmission is impossible

### φ4: ack_lookup_ignores_stored → `ack_state_lookup`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If lookup always returns zero acked packets for a known endpoint, acknowledgements are lost and delivery breaks

### φ5: redirect_modifies_received_requests → `next_set_request`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If received_requests changes on a redirect (non-local key), the host logs phantom requests it never processed
- **Verdict:** TRUE_POSITIVE (medium)

