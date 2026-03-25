# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__host_impl_v__impl2__host_noreceive_noclock_next/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: lookup_always_none → `hashtable_lookup`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If a present key always maps to None, all get-request replies would incorrectly report missing values

### φ2: retransmit_sends_nothing → `spontaneously_retransmit`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If retransmit never produces packets, unacknowledged messages are silently lost and reliability is broken

### φ3: set_request_no_h_change → `next_set_request`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If set requests never modify the hashtable, the key-value store cannot be written to at all

### φ4: bulk_update_excludes_new_keys → `bulk_update_domain`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If bulk_update_domain drops keys present in the update table within the range, delegate operations silently lose data

### φ5: send_always_suppressed → `send_single_message`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If send_single_message always returns None, no replies or delegates are ever transmitted, making the protocol non-functional

