# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__host_impl_v__impl2__host_model_next_shard/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: send_shard_empty_output → `next_shard`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If a shard with should_send=true produces empty output, recipients never receive delegated data, breaking liveness.

### φ2: send_always_produces_message → `send_single_message`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If send_single_message always yields Some, the max_seqno overflow guard is vacuous and unbounded sends are possible.

### φ3: missing_ack_nonzero → `ack_state_lookup`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A missing key returning nonzero acked count would fabricate acknowledgment history, corrupting retransmission logic.

### φ4: reject_modifies_hashtable → `next_shard_wrapper`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If a rejected shard still mutates the hashtable, data is silently lost without any corresponding delegation transfer.

### φ5: no_send_changes_hashtable → `next_shard`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If should_send=false still removes keys from h, data is destroyed without being delegated, causing permanent key loss.

