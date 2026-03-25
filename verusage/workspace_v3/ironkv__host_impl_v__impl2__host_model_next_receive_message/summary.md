# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__host_impl_v__impl2__host_model_next_receive_message/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: get_request_always_redirects → `next_get_request_reply`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean the host can never serve a GetRequest locally, always redirecting even for owned keys.

### φ2: set_request_no_hashtable_mutation → `next_set_request_complete`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean SetRequests never actually modify the hashtable, rendering writes completely inert.

### φ3: process_message_no_requests_logged → `process_message`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean the host never appends to received_requests, breaking application-level request tracking.

### φ4: shard_send_produces_empty_output → `next_shard`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Would mean shard operations never transmit Delegate packets to recipients even when should_send is true.

### φ5: delegate_no_delegation_count_increase → `next_delegate`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean accepted Delegate messages never increment the delegation counter, breaking the max-delegations safety bound.

