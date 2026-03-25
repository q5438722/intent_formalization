# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec_utils/spec__utils__map_new_rec_dom_finite.rs`
**Date:** 2026-03-24T12:10:03Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives. They are direct consequences of the fully verified `map_new_rec` open spec and its `map_new_rec_dom_finite` lemma. The function intentionally creates a constant map over the closed interval `[0, dom]`, and all tested properties (key 0 present, keys beyond dom absent, singleton at dom=0) are correct by design.

## All Candidates

### φ1: map_new_rec_contains_zero
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The map always contains key 0 regardless of `dom` — the base case inserts `0 => val` so the domain is `[0, dom]` not `[1, dom]` or empty
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The function is designed to create a map over `[0, dom]`. Always containing key 0 is the intended base case behavior, directly proved by the verified `map_new_rec_dom_finite` lemma.

### φ2: map_new_rec_all_same_value
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Every key in the map maps to the same value — the map is a constant function, so it cannot represent heterogeneous initialization

### φ3: map_new_rec_dom_size
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The domain has `dom + 1` elements (0 through dom inclusive) — callers might expect `dom` elements if they think `dom` is a count rather than an upper bound

### φ4: map_new_rec_dom_is_closed_interval
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Keys beyond `dom` are absent — the domain is exactly `[0, dom]` with no stray keys, tests the upper bound exclusion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Keys beyond `dom` being absent is directly ensured by the second postcondition of `map_new_rec_dom_finite`: `0 <= n <= dom <==> contains_key(n)`. Correct by design.

### φ5: map_new_rec_zero_is_singleton
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `map_new_rec(0, val)` is a singleton map `{0 => val}` — the base case creates a non-empty map, so there is no way to construct an empty map with this function
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The base case `dom == 0` explicitly returns `map![dom => val]` which is `map![0 => val]`. This is the intended definition — the function creates a constant map over `[0, dom]`, and `dom=0` gives a singleton.

