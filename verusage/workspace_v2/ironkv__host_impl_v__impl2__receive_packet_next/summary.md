# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/host_impl_v/host_impl_v__impl2__receive_packet_next.rs`
**Date:** 2026-03-24T09:04:26Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate is a false positive. The `between` predicate correctly implements half-open interval semantics `[lhs, rhs)`, and the empty range `[x, x)` is standard and intentional behavior, not a spec gap.

## All Candidates

### φ1: delegate_abstractable_unchecked_hashmap
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `Delegate` falls through to `_ => true` in `abstractable()` without validating the CKeyHashMap, so any hash map — including corrupted or inconsistent ones — is considered abstractable

### φ2: get_set_reply_abstractable_trivially
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** GetRequest, SetRequest, and Reply are unconditionally abstractable — no validation on the key `k` or optional value `v`, so malformed keys or values silently enter the abstract model

### φ3: between_empty_at_equal_bounds
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `between(x, x, x)` is false because `x.lt_spec(x)` is false for strict ordering — a degenerate range `[x, x)` is empty, so point queries using `between` silently match nothing
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `between(lhs, ki, rhs)` implements the standard half-open interval `[lhs, rhs)` where `ki >= lhs && ki < rhs`. When all three are equal, `ki < ki` is correctly false — this is the expected semantics of a half-open interval where `[x, x)` is empty. Point queries in delegation maps use exact key lookups, not `between`.

### φ4: invalid_message_abstractable_and_viewable
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `InvalidMessage` is unconditionally abstractable and maps directly to `SingleMessage::InvalidMessage` — corrupted/malformed network messages pass the abstraction barrier with no validation, entering verified protocol logic

### φ5: vec_t_serialize_element_order_dependent
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `Vec<T>::ghost_serialize` serializes elements in sequence order via `fold_left`, so reordering elements produces different byte sequences — but there is no spec-level guarantee that `ghost_serialize` is injective (no `lemma_serialize_injective` for `Vec<T>`), meaning two semantically-different vectors could theoretically be treated as equal by downstream code relying on serialization identity

