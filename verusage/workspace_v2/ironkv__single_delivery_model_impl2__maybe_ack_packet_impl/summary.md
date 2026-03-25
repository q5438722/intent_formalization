# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/single_delivery_model_v/single_delivery_model_impl2__maybe_ack_packet_impl.rs`
**Date:** 2026-03-24T09:52:53Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 1
- False positives: 2

## Summary

One true positive: `CKeyHashMap` has two uninterpreted spec functions (`view` and `spec_to_vec`) with no connecting axiom, leaving the abstract map interpretation completely unconstrained relative to the concrete representation. The other two candidates are false positives — the redundant `abstractable()` check in `valid_ack` is defensive but harmless, and the tombstone default of 0 is correct IronFleet protocol semantics for unknown senders.

## True Positives (Spec Issues)

### ckeyhasmap_view_unconstrained
- **Confidence:** medium
- **Reasoning:** `CKeyHashMap::view` and `spec_to_vec` are both uninterpreted with no axiom relating them. This means the abstract map view is completely unconstrained relative to the vector serialization, allowing the verifier to assume arbitrary abstract states that don't correspond to the actual HashMap contents.

## All Candidates

### φ1: valid_ack_redundant_abstractable
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `valid_ack` requires both `ack.abstractable()` and `outbound_packet_is_valid(&ack)` which itself requires `cpacket.abstractable()` — the redundant conjunct suggests the spec author was unsure about the relationship (noted in the comment "how does this relate to abstractable?")
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The redundancy is harmless — `valid_ack` requiring `ack.abstractable()` explicitly alongside `outbound_packet_is_valid(&ack)` (which also requires it) is just defensive specification style. The comment expresses a code-quality concern, not a soundness gap.

### φ2: tombstone_lookup_default_zero
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Defaulting to 0 for unknown sources means any new sender is treated as having no tombstone — if the system relies on tombstones for duplicate suppression, a fresh endpoint could replay old sequence numbers
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Defaulting to 0 for unknown sources is the standard IronFleet/SHT protocol design — a fresh endpoint with no tombstone entry legitimately has received no packets, so sequence number 0 is correct. This is intentional protocol semantics.

### φ3: ckeyhasmap_view_unconstrained
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `CKeyHashMap::view` and `spec_to_vec` are both uninterpreted — there is no axiom relating them, so the view of a CKeyHashMap is completely unconstrained relative to its vector representation, allowing inconsistent abstract states
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `CKeyHashMap::view` and `spec_to_vec` are both uninterpreted with no axiom relating them. This means the abstract map view is completely unconstrained relative to the vector serialization, allowing the verifier to assume arbitrary abstract states that don't correspond to the actual HashMap contents.

### φ4: clone_up_to_view_external_body
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `clone_up_to_view` is `external_body` with `unimplemented!()` — the exec body panics at runtime while its ensures clause is trusted without verification

### φ5: cack_state_new_ack_valid
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If a freshly constructed CAckState with empty un_acked list and zero packets_acked is not valid, the initial state construction would be broken — but if it IS valid without checking `max_seqno >= 0`, the validity predicate may be vacuously satisfied for degenerate parameters

