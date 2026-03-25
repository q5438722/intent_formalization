# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/host_impl_v/host_impl_v__impl2__host_noreceive_noclock_next.rs`
**Date:** 2026-03-24T08:54:18Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. The tuple serialization PHI merely confirms definitional equality given matching serialized bytes. The ghost_serialize concern doesn't apply because Verus resolves the concrete u64 impl. The duplicate-related PHI identifies an intentional design choice where sequences and sets serve different abstraction purposes.

## All Candidates

### φ1: option_none_some_empty_serialize_differ
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** None serializes to `[0]` (1 byte) while Some(empty_vec) serializes to `[1] + len_prefix(0)` (9 bytes) — the tag byte distinguishes them, but the asymmetric lengths mean deserializing from a fixed-size buffer could silently truncate or misparse

### φ2: tuple_serialize_no_length_prefix
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Tuple serialization is simple concatenation `a.serialize() + b.serialize()` — two distinct pairs with different first elements can produce identical serialized bytes if the length-prefixed Vec<u8> fields happen to align, violating injectivity of tuple serialization at the composite level
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The ensures simply restates the definition: `(a1, a2).ghost_serialize() == a1.ghost_serialize() + a2.ghost_serialize()`. The requires already gives `a1.ghost_serialize() + a2.ghost_serialize() == b1.ghost_serialize() + b2.ghost_serialize()`, so the conclusion follows trivially. The concern about injectivity is real in principle, but `Vec<u8>` serialization includes a length prefix, making the concatenation unambiguous for this specific type — and regardless, the PHI itself just confirms definitional equality, not an actual collision.

### φ3: csingle_message_seqno_truncation
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** CSingleMessage::view casts `ack_seqno` from u64 to nat via `as nat` — the abstract SingleMessage uses nat for seqno, so two different protocol implementations could disagree on seqno overflow semantics since the concrete type silently caps at u64::MAX

### φ4: ghost_serialize_trait_uninterpreted
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The trait-level `ghost_serialize` is `external_body` with an uninterpreted default — the u64 impl overrides it with `spec_u64_to_le_bytes`, but any code calling through the trait (not the concrete impl) could get the uninterpreted version, silently losing the 8-byte length guarantee
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The u64 impl provides a concrete `open spec fn ghost_serialize` that overrides the trait default. When called on a value of type `u64`, Verus resolves to the impl's definition (`spec_u64_to_le_bytes`), not the external_body default. The 8-byte length is a correct mathematical property of `spec_u64_to_le_bytes`.

### φ5: abstractify_cmessage_seq_loses_duplicates
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `abstractify_cmessage_seq` preserves sequence length including duplicates, but `abstractify_seq_of_cpackets_to_set_of_sht_packets` converts to a Set which deduplicates — two functions on the same conceptual data use different collection types, so duplicate-counting invariants silently disagree between Seq-based and Set-based abstractions
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `abstractify_cmessage_seq` is a Seq→Seq map that preserves length and duplicates — this is its intended behavior. The separate function `abstractify_seq_of_cpackets_to_set_of_sht_packets` operates on CPackets (not CSingleMessages) and deliberately converts to a Set for a different use case. These are distinct abstractions for different purposes, not an inconsistency.

