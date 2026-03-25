# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__host_impl_v__impl2__real_next_impl/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: lookup_present_key_returns_none → `hashtable_lookup`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A key present in the hashtable should return Some, not None; this would break all GET request lookups.

### φ2: bulk_update_loses_original_keys → `bulk_update_domain`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Keys in the original hashtable outside the updated range must be preserved; losing them would silently drop data during delegation.

### φ3: none_value_is_invalid → `valid_optional_value`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** None represents a key deletion and must be valid; rejecting it would prevent any SetRequest from removing keys.

### φ4: retransmit_modifies_hashtable → `spontaneously_retransmit`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Retransmission only resends un-acked packets; it must never mutate the hashtable or it would corrupt data.

### φ5: no_invalid_sends_allows_invalid → `no_invalid_sends`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The no_invalid_sends guard must actually prevent InvalidMessage sends; if it doesn't, the trusted marshalling invariant is violated.

