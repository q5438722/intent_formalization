# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl4__almost_all_keys_agree/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: new_iterator_always_end → `is_end`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If a freshly constructed Some(k) iterator reports is_end, then is_end cannot distinguish live iterators from exhausted ones

### φ2: none_iterator_not_end → `is_end`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If a None-valued iterator is not considered at-end, the is_end spec inverts the intended semantics

### φ3: get_conflates_distinct_keys → `get`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If get_spec returns the same value for two distinct keys, the spec fails to preserve key identity

### φ4: get_returns_wrong_value → `get`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If get_spec on Some(k) can equal an arbitrary different key k2, the ensures clause is too weak to pin the return value

### φ5: get_precondition_vacuous → `get`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the requires of get is unsatisfiable, then get's contract is vacuously true and can never be usefully called

