# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/host_impl_v/host_impl_v__impl2__real_next_impl.rs`
**Date:** 2026-03-24T09:03:01Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 1
- False positives: 3

## Summary

Three of four candidates are false positives reflecting correct design choices: `Ordering::lt` correctly implements strict less-than, `between` uses standard half-open interval semantics, and tuple serialization works because inner types like `Vec<u8>` are self-delimiting via length prefixes. The one true positive is `cmessage_delegate_no_abstractability_check_on_h` — the `Delegate` variant's `CKeyHashMap` field bypasses the abstractability check that other complex fields (endpoints in `Redirect` and `Shard`) undergo, creating an inconsistency in the abstraction barrier.

## True Positives (Spec Issues)

### cmessage_delegate_no_abstractability_check_on_h
- **Confidence:** medium
- **Reasoning:** `CMessage::abstractable` checks `id@.abstractable()` for `Redirect` and `recipient@.abstractable()` for `Shard`, but the `Delegate` variant falls through to `_ => true` without checking `h` (the CKeyHashMap). This is inconsistent with the pattern of checking abstractability on complex fields, allowing an unchecked hash map into the abstract model.

## All Candidates

### φ1: ordering_lt_only_less
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `Ordering::lt` returns true only for `Less` — there is no `le`, `ge`, or `gt` helper, so any comparison-based spec using `Ordering` can only express strict less-than, silently preventing less-than-or-equal range queries in delegation map lookups
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `Ordering::lt` correctly implements strict less-than by matching only `Ordering::Less`. The absence of `le`/`ge`/`gt` helpers is simply incomplete API surface, not a spec gap — `between` uses `lt_spec` which is the appropriate comparator for half-open interval semantics used throughout the delegation map.

### φ2: between_excludes_lower_bound
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `between(lhs, ki, rhs)` requires `!ki.lt_spec(lhs) && ki.lt_spec(rhs)` — when all three are equal, `ki.lt_spec(rhs)` is false (not strictly less than itself), so a single-element range `[x, x)` is empty, meaning point lookups using `between` silently return nothing
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `between` implements a standard half-open interval `[lhs, rhs)` — `!ki.lt_spec(lhs)` means `ki >= lhs` and `ki.lt_spec(rhs)` means `ki < rhs`. When all three are equal, the range `[x, x)` is correctly empty. This is the standard convention for range iteration in key-value stores and delegation maps.

### φ3: cmessage_delegate_no_abstractability_check_on_h
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `CMessage::abstractable` falls through to `_ => true` for `Delegate` without checking `h` — an arbitrary (potentially invalid) CKeyHashMap enters the abstract protocol model unchecked
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `CMessage::abstractable` checks `id@.abstractable()` for `Redirect` and `recipient@.abstractable()` for `Shard`, but the `Delegate` variant falls through to `_ => true` without checking `h` (the CKeyHashMap). This is inconsistent with the pattern of checking abstractability on complex fields, allowing an unchecked hash map into the abstract model.

### φ4: vec_marshalable_empty_always_valid
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** An empty `Vec<T>` is always marshalable regardless of T's properties — `fold_left` on an empty sequence produces 0 for size and the `forall` over `contains` is vacuously true, so `is_marshalable` provides no guarantee about the element type's serializability for empty collections

### φ5: tuple_serialize_ambiguous_concatenation
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Tuple serialization is bare concatenation with no delimiter or length prefix between components — if `a.ghost_serialize()` has variable length, a deserializer cannot determine where the first element ends and the second begins without external framing, breaking injectivity of the serialization format
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The PHI merely confirms the definitional equality of tuple serialization. While bare concatenation could theoretically cause ambiguity, `Vec<u8>::ghost_serialize` includes a length prefix, so `(Vec<u8>, Vec<u8>)` tuples are in fact unambiguously parseable. The serialization format's injectivity depends on the inner types being self-delimiting, which length-prefixed types satisfy.

