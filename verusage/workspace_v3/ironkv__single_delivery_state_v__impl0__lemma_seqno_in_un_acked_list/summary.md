# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__single_delivery_state_v__impl0__lemma_seqno_in_un_acked_list/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: always_valid → `valid_physical_address`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean every endpoint is considered valid, making the check vacuous

### φ2: never_valid → `valid_physical_address`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Would mean no endpoint is ever valid, making the system completely unusable

### φ3: boundary_inclusive_at_limit → `valid_physical_address`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Off-by-one error — an id of exactly 0x100000 bytes should be rejected by the strict < bound

### φ4: empty_id_invalid → `valid_physical_address`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Would mean a zero-length endpoint id is not a valid address, contradicting 0 < 0x100000

### φ5: valid_implies_very_short → `valid_physical_address`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Would mean the spec is far more restrictive than intended, capping ids at 256 bytes instead of ~1MB

