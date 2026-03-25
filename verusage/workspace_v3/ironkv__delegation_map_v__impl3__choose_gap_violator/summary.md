# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl3__choose_gap_violator/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: is_end_always_true → `is_end`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, every KeyIterator would be considered at-end, making get() unreachable and the iterator useless.

### φ2: some_reported_as_end → `is_end`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If entailed, a KeyIterator holding a valid key would be classified as at-end, contradicting the Some/None semantics.

### φ3: get_mismatches_key → `get`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If entailed, get() would return a value different from the stored key, breaking the fundamental accessor contract.

### φ4: non_end_implies_end → `is_end`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, the is_end spec would be self-contradictory, meaning the KeyIterator type is uninhabitable in non-end states.

### φ5: get_conflates_distinct_keys → `get`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If entailed, get() could not distinguish different keys, meaning the spec collapses all key values to a single equivalence class.

