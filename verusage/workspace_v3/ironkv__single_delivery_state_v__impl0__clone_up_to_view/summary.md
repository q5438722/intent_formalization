# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__single_delivery_state_v__impl0__clone_up_to_view/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: clone_key_always_zero → `SHTKey::clone (inherent)`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, clone would collapse all keys to zero, destroying key identity

### φ2: ack_clone_empties_unacked → `CAckState::clone_up_to_view`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, view-equality would force the un_acked list to always be empty after clone, losing pending messages

### φ3: range_clone_collapses_lo_hi → `KeyRange::clone (VerusClone impl)`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, the clone spec would force every key range to be empty, making sharding impossible

### φ4: ack_clone_resets_seqno → `CAckState::clone_up_to_view`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, view-equality would force the ack counter to zero, losing acknowledgement progress

### φ5: iter_clone_loses_some → `KeyIterator::clone (VerusClone impl)`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, cloning a valid iterator would lose its key, breaking iteration over key ranges

