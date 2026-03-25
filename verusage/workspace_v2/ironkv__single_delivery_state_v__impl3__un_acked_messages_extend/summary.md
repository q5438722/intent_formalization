# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/single_delivery_state_v/single_delivery_state_v__impl3__un_acked_messages_extend.rs`
**Date:** 2026-03-24T09:59:58Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive: `CKeyHashMap` has two uninterpreted spec functions (`view` and `spec_to_vec`) with no connecting axiom, leaving the abstract map interpretation unconstrained relative to the vector serialization. The empty list vacuous satisfaction is a standard base case for the ack protocol, not a spec gap.

## True Positives (Spec Issues)

### ckeyhasmap_view_spec_to_vec_unrelated
- **Confidence:** medium
- **Reasoning:** `CKeyHashMap::view` and `spec_to_vec` are both uninterpreted spec functions with no axiom relating them. This means the abstract map view is completely unconstrained relative to the vector representation, allowing the verifier to assume arbitrary abstract states that may not correspond to the actual HashMap contents.

## All Candidates

### φ1: hashmap_insert_non_injective_key
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Two distinct `EndPoint` values with the same abstract view map to the same key in the abstract `Map<AbstractEndPoint, V>`, but the underlying `collections::HashMap<EndPoint, V>` could store them separately — the ensures on `insert` forces abstract deduplication without verifying the concrete implementation agrees

### φ2: ckeyhasmap_view_spec_to_vec_unrelated
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `CKeyHashMap::view` and `spec_to_vec` are both uninterpreted with no connecting axiom — the abstract map contents are completely unconstrained relative to the vector serialization, allowing inconsistent abstract states
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `CKeyHashMap::view` and `spec_to_vec` are both uninterpreted spec functions with no axiom relating them. This means the abstract map view is completely unconstrained relative to the vector representation, allowing the verifier to assume arbitrary abstract states that may not correspond to the actual HashMap contents.

### φ3: csendstate_abstractable_trigger_mismatch
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `CSendState::abstractable` quantifies over `ep: EndPoint` with trigger `self@.contains_key(ep@)` but `self@` is `SendState<Message>` (mapped values) while `self.epmap@` is `Map<AbstractEndPoint, CAckState>` — the trigger references the derived map rather than the source, potentially mismatching domains

### φ4: cmessage_delegate_abstractable_unchecked_hashmap
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `CMessage::abstractable` for `Delegate` falls through to `_ => true` without checking whether `CKeyHashMap` is abstractable — any `Delegate` message is considered abstractable regardless of its hashmap contents

### φ5: un_acked_list_sequential_empty_vacuous
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** An empty un_acked list vacuously satisfies both `un_acked_list_sequential` and `un_acked_list_valid` (including the `un_acked_valid` quantifier), meaning empty lists bypass all message validity checks
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** An empty un_acked list vacuously satisfying sequential and valid predicates is standard and expected. The initial `AckState::new()` creates an empty `un_acked: seq![]`, and the protocol builds up the list incrementally — empty is the correct base case.

