# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__impl4__new/original.rs`
**Date:** 2026-03-24T21:49:14Z

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property `new_valid_implies_lows_empty` is a false positive. It is structurally invalid because it attempts to call an exec function (`DelegationMap::new`) from a proof function, which Verus rejects. Even ignoring that, the claim that `lows@.dom().len() == 0` after construction is factually wrong: the `new` implementation inserts `k_zero` into `lows` via `set`, so the backing map has exactly one entry. The postcondition of `new` correctly ensures `s.valid()` (which requires `lows@.contains_key(K::zero_spec())`) and `s@ == Map::total(|k| id_zero@)`, both of which are consistent with the implementation. No real spec issue exists here.

## All Candidates

### φ1: get_vacuous_precondition → `get`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If all KeyIterators are always at end, get() can never be called and its ensures is never tested

### φ2: is_end_always_true → `is_end`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If is_end always returns true then no KeyIterator ever holds a valid key, making all key-based operations unreachable

### φ3: is_end_always_false → `is_end`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If is_end always returns false then the end-of-iteration sentinel is unrepresentable, breaking termination of key iteration

### φ4: new_delegation_map_empty → `new`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If the total map disagrees with id_zero@ at any key, new()'s ensures that every key maps to id_zero@ would be contradictory or wrong

### φ5: new_valid_implies_lows_empty → `new`
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If the backing lows map is empty after construction, the invariant that lows contains zero_spec() is violated, meaning valid() is trivially false or broken
- **Verdict:** FALSE_POSITIVE (high)

