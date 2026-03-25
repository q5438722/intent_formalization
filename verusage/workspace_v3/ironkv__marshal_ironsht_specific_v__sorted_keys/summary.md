# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__marshal_ironsht_specific_v__sorted_keys/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: clone_always_zero → `clone`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Undesirable if clone always produces a key with ukey 0 regardless of input

### φ2: clone_collapses_distinct_keys → `clone`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Undesirable if cloning two distinct keys yields equal outputs, meaning clone loses information

### φ3: sorted_keys_empty_is_unsorted → `sorted_keys`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Undesirable if an empty vector is reported as unsorted; the vacuous forall should be true

### φ4: sorted_keys_descending_accepted → `sorted_keys`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Undesirable if a strictly descending pair is accepted as sorted

### φ5: sorted_keys_always_true → `sorted_keys`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Undesirable if sorted_keys is trivially true for all inputs, meaning it never rejects unsorted vectors

