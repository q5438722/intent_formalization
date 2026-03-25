# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__host_impl_v__impl2__host_model_next_get_request/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: get_reply_empty_output_when_should_send → `next_get_request_reply`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A successful get-request reply (should_send=true) must emit exactly one packet; an empty output set would mean the response is silently dropped

### φ2: send_exceeds_max_seqno_still_sends → `send_single_message`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** When the sequence number exceeds max_seqno the protocol must suppress sending; allowing Some(sm) would violate the flow-control bound

### φ3: redirect_modifies_received_requests → `next_get_request_reply`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A redirected request is not served locally, so received_requests must stay unchanged; mutation here would miscount application-level requests

### φ4: lookup_none_despite_key_present → `hashtable_lookup`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Looking up a key known to be in the hashtable must return Some; returning None would lose stored data and break get-request correctness

### φ5: get_request_changes_hashtable → `next_get_request`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A read-only GetRequest must never mutate the hashtable; allowing h to change would turn a get into an implicit write

