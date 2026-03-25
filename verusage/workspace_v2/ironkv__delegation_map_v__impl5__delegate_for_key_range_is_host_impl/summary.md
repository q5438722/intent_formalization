# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/delegation_map_v/delegation_map_v__impl5__delegate_for_key_range_is_host_impl.rs`
**Date:** 2026-03-24T08:46:24Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. The first two exploit vacuous truth over empty/degenerate key ranges, which is standard logical behavior and not a spec weakness. The third simply restates invariants that are explicitly part of the `DelegationMap::valid` definition. No real spec issues were identified.

## All Candidates

### φ1: range_consistent_impl_external_body_trusted
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `range_consistent_impl` is external_body — if the implementation returns true for a range where some key actually disagrees, the trusted ensures would introduce unsoundness in all delegation lookups

### φ2: delegate_for_key_range_vacuous_end_end
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** With both lo and hi as end iterators (None), `between` is vacuously false since `ki.lt_spec(None)` requires `!ki.k.is_None()` but `None.lt_spec(ki)` is always false — so `delegate_for_key_range_is_host` holds for any destination, making empty-range delegation claims meaningless
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Vacuous truth over an empty range is standard. When both lo and hi are end iterators, no key satisfies `between`, so `delegate_for_key_range_is_host` holds trivially for any destination. This is expected behavior for empty ranges.

### φ3: delegate_for_key_range_contradictory_dsts
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A degenerate range [k, k) is simultaneously "delegated" to two different hosts with different abstract endpoints, because `between(new_spec(k), new_spec(k), new_spec(k))` requires `k < k` which is false — callers cannot distinguish "all keys agree" from "empty range"
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The range [k, k) is empty because `between(new_spec(k), new_spec(k), new_spec(k))` requires `k < k` which is false. Vacuous truth over an empty domain simultaneously satisfying contradictory predicates is standard logic, not a spec gap.

### φ4: sht_key_between_excludes_lo_equal
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `between(lo, ki, hi)` should include the lo endpoint (since `!ki.lt(lo)` when ki == lo), so key 5 should be between [5, 10) — if this verifies, the lower bound is unexpectedly exclusive and range_consistent misses boundary keys

### φ5: abstract_delegation_map_complete_unchecked
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `DelegationMap::valid` requires `self@.dom().is_full()` and all values have valid physical addresses — but the ghost field `m` has no runtime enforcement, so any external_body function that constructs a DelegationMap could silently violate these invariants
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This directly follows from `DelegationMap::valid` requiring `self@.dom().is_full()` and `forall |k| self@[k].valid_physical_address()`. These are the intended invariants of a valid delegation map — every key must be delegated to a valid endpoint. The ghost field concern is the standard Verus pattern, not a spec issue.

