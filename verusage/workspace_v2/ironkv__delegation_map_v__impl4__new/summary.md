# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/delegation_map_v/delegation_map_v__impl4__new.rs`
**Date:** 2026-03-24T08:43:56Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate is a false positive. The property that every key is in the domain follows directly from the `is_full()` invariant in `DelegationMap::valid()`, which is the intentional design for a delegation map that must assign every key in the keyspace to a responsible endpoint.

## All Candidates

### φ1: new_map_all_keys_same_endpoint
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** After construction, every key maps to the same endpoint — the delegation map has no ability to distinguish keys at creation, meaning the initial state is maximally degenerate and callers cannot set per-key delegations at construction time

### φ2: set_external_body_gap_false_oracle
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `StrictlyOrderedMap::set` is external_body and its gap postcondition claims inserting key `k` breaks any gap containing `k` — but since this is unverified, a buggy implementation could claim gaps exist where they don't

### φ3: valid_physical_address_large_bound
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** An endpoint with nearly 1MB of ID data (0xFFFFF - 1 = 1048574 bytes) is considered a valid physical address — the 0x100000 bound is very large and may not correspond to actual network address constraints

### φ4: delegation_map_valid_no_key_trait_constraints
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `valid()` requires `self@.dom().is_full()` meaning every possible key is in the domain — there is no way to have a valid delegation map that doesn't map every key, which means the spec cannot express partial delegation
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A delegation map is explicitly designed to be a total function — every key maps to some endpoint. This is the intended semantics for a sharded hash table's delegation map, where every key in the keyspace must be assigned to some node. The `is_full()` requirement is by design, not a spec weakness.

### φ5: new_external_body_empty_map_trusted
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `StrictlyOrderedMap::new` is external_body and asserts the ghost map is empty — if the implementation returns a non-empty map, the ghost state would be desynchronized from actual state, silently introducing unsoundness in all downstream gap and containment reasoning

