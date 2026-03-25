# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/marshal_ironsht_specific_v/marshal_ironsht_specific_v__sorted_keys.rs`
**Date:** 2026-03-24T09:16:59Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 0
- False positives: 0

## All Candidates

### φ1: sorted_keys_empty_trivially_true
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** An empty vector is vacuously sorted — callers expecting a non-empty sorted collection get no guarantee of any elements

### φ2: sorted_keys_no_duplicate_exclusion
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Two entries with the same key but different values cannot be sorted (strict `<` prevents equal keys) — but the spec implicitly forbids duplicate keys through `ckeykvlt`'s strict ordering rather than an explicit uniqueness constraint, which is fragile

### φ3: ckeykvlt_ignores_value
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `ckeykvlt` compares only keys and completely ignores values — two entries with the same key but different values are considered equal by the ordering, so a sorted key-value list could silently drop value distinctions

### φ4: sorted_singleton_any_key
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Any single-element vector is vacuously sorted regardless of the key value — there is no well-formedness check on individual keys (e.g., no range constraint on `ukey`)

### φ5: view_loses_key_identity
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `CKeyKV::view` returns the key by value (not wrapped) and the value as `Seq<u8>` — the view always preserves exact key identity, meaning the abstract model conflates `CKey` with `AbstractKey` with no abstraction barrier, so any internal key representation change would break all downstream specs

