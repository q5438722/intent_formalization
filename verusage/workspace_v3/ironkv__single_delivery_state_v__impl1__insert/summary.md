# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__single_delivery_state_v__impl1__insert/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: insert_removes_unrelated_key → `insert`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Insert should preserve all pre-existing keys unrelated to src; losing them would silently drop tombstone entries

### φ2: insert_max_seqno_wraps_to_zero → `insert`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If u64::MAX as nat wraps to zero the tombstone table would record a wrong sequence number, enabling replayed messages

### φ3: insert_key_missing_after_insert → `insert`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The inserted key must be present afterward; its absence means the tombstone was silently discarded

### φ4: insert_corrupts_other_value → `insert`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Inserting one key must not mutate another key's sequence number, or unrelated tombstone records become corrupted

### φ5: insert_value_independent_of_arg → `insert`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The stored value must equal the provided last_seqno; if any other value satisfies the spec, sequence-number tracking is broken

