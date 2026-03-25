# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl4__all_keys_agree/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: is_end_true_for_some → `is_end`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If an iterator wrapping Some(k) reports is_end as true, non-end iterators would be indistinguishable from end iterators

### φ2: is_end_false_for_none → `is_end`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If a None-iterator reports is_end as false, callers would attempt to dereference a nonexistent key

### φ3: new_get_roundtrip_fails → `get`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If get_spec on new_spec(k) does not return k, the accessor loses the stored key value

### φ4: get_conflates_distinct_keys → `get`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If get_spec maps distinct keys to the same value, the iterator cannot faithfully distinguish different keys

### φ5: is_end_indistinguishable_some_none → `is_end`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If is_end_spec returns the same value for Some and None variants, the end-of-iteration check is completely broken

