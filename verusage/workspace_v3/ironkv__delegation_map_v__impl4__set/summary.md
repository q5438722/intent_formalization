# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl4__set/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: greater_is_lt → `is_lt`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If Greater were classified as Less, the ordering comparison used throughout StrictlyOrderedMap and DelegationMap would be fundamentally broken

### φ2: new_spec_is_end → `is_end`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A KeyIterator wrapping Some(k) should never report as end; if it did, get() would be unreachable and DelegationMap range operations would silently skip all keys

### φ3: above_self → `above`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A key should not be strictly above itself in its own iterator; this would break the greatest-lower-bound logic used by DelegationMap::get_internal

### φ4: get_roundtrip_broken → `get`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Extracting a key via get and re-wrapping with new_spec must roundtrip to the same iterator; failure would break between/gap reasoning in DelegationMap::set

### φ5: set_misses_interior → `set`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A key strictly between lo and hi must be inside the between range; otherwise DelegationMap::set would fail to update interior keys in the assigned range

