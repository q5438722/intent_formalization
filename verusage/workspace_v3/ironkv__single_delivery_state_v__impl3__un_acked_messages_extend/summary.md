# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__single_delivery_state_v__impl3__un_acked_messages_extend/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_valid_address → `valid_physical_address`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, the spec is vacuously true and valid_physical_address never rejects any input.

### φ2: boundary_len_exact_valid → `valid_physical_address`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, the boundary is off-by-one and addresses of length exactly 0x100000 are accepted.

### φ3: empty_id_invalid → `valid_physical_address`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, zero-length addresses are rejected, which would be undesirable since empty IDs should be valid physical addresses.

### φ4: new_hashmap_nonempty → `new`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, the empty map postcondition of HashMap::new is inconsistent and the map already contains keys.

### φ5: zero_count_nonempty_set → `un_acked_messages_for_dest_up_to`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, requesting zero un-acked messages still yields packets, meaning the count parameter has no filtering effect.

