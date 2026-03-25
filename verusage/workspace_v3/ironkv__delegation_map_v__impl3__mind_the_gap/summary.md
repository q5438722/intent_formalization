# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl3__mind_the_gap/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: end_allows_get → `end`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, end() produces an iterator that is NOT considered "end", contradicting its purpose as a sentinel

### φ2: is_end_always_true → `is_end`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, is_end returns true for every iterator, making it impossible to distinguish live iterators from the end sentinel

### φ3: get_collapses_keys → `get`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If provable, get returns the same key for all non-end iterators, meaning the spec fails to distinguish different iterator positions

### φ4: end_has_some_key → `end`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If provable, end() yields an iterator carrying a key despite its spec promising k.is_None(), indicating an inconsistent postcondition

### φ5: no_non_end_exists → `get`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If provable, every iterator is end so get's precondition is unsatisfiable, making get vacuously correct but useless

