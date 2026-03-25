# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec_cyclicbuffer/spec__cyclicbuffer__map_min_value_smallest.rs`
**Date:** 2026-03-24T12:04:54Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. The base case, minimality, map-membership, and prefix-bounded behavior are all correct and intended properties of the fully verified `map_min_value` function and its `map_min_value_smallest` lemma. No external_body or trust gaps exist in this file.

## All Candidates

### φ1: map_min_value_base_case
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Base case returns `m[0]` directly — tests that `map_min_value` at index 0 equals the value at key 0
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The base case `idx == 0` returning `m[0]` is directly stated in the open spec definition. This is correct by design.

### φ2: map_min_value_leq_all
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The minimum is at most every element in range — direct consequence of the verified lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a direct instantiation of the first ensures clause of the fully verified `map_min_value_smallest` lemma. The minimum being ≤ all elements in range is the core correctness property.

### φ3: map_min_value_is_in_map
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The minimum value exists in the map — tests that `map_min_value` returns an actual value from the map, not a fabricated one
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the second ensures clause of `map_min_value_smallest` — the minimum value actually exists in the map. Correct and expected for a well-defined min function.

### φ4: map_min_value_ignores_keys_beyond_idx
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** `map_min_value(m, idx)` only examines keys `[0, idx]` — inserting a smaller value at `idx+1` doesn't change the result, so the "minimum" over the map can miss the actual global minimum

### φ5: map_min_value_not_global_min
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The minimum over `[0, idx]` can be strictly larger than `m[idx+1]` — the function name suggests "map minimum" but it only covers a prefix, potentially misleading callers about what range is covered
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The φ's precondition explicitly assumes `m[idx+1] < map_min_value(m, idx)`, so the ensures is a trivial consequence. The function is designed to compute the minimum over `[0, idx]`, not the entire map — callers pass the appropriate `idx` parameter.

