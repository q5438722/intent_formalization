# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__host_impl_v__impl2__effect_of_delegation_map_set/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: get_conflates_distinct_keys → `get`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If get returns the same value for two strictly-ordered iterators, it fails to distinguish distinct keys.

### φ2: get_reconstructs_to_end → `get`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A key retrieved via get, when wrapped back into an iterator, should never appear ended.

### φ3: new_get_not_identity → `get`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** get on an iterator freshly built from key k must return k; otherwise the spec breaks the new/get round-trip.

### φ4: get_reverses_order → `get`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If ki1 < ki2 in iterator ordering, their retrieved keys must preserve that order, not reverse it.

### φ5: get_between_always_includes → `get`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A key from an iterator strictly before lo should not land between lo and hi; would mean get ignores ordering.

