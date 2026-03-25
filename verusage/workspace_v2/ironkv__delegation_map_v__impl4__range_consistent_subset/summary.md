# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/delegation_map_v/delegation_map_v__impl4__range_consistent_subset.rs`
**Date:** 2026-03-24T08:45:05Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 0

## All Candidates

### φ1: range_consistent_vacuous_equal_bounds
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** When lo == hi, `between` requires `!ki.lt(lo) && ki.lt(hi)` which is unsatisfiable for ki=lo since that requires `ki.lt(ki)`, making range_consistent vacuously true for any destination — callers cannot distinguish "all keys agree" from "empty range"

### φ2: range_consistent_subset_widens_to_empty
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `range_consistent_subset` allows shrinking a range by moving lo forward and hi backward — but if you swap lo and hi (making lo >= hi), the between predicate becomes vacuously true, so any range_consistent claim can be "reversed" into a vacuously true claim about the swapped empty range

### φ3: range_consistent_any_dst_for_end_end
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** End iterators (None) make `lt_spec` false in both directions since `lt_spec` requires `!self.k.is_None()`, so between is vacuously false for any ki, making range_consistent trivially true for ANY two destinations simultaneously

### φ4: range_consistent_contradictory_dsts
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** A degenerate range [k, k) is simultaneously consistent with two different destinations that have different abstract views — this means range_consistent provides zero information about what `dm@[k]` actually is at the boundary point `k`

### φ5: range_consistent_subset_to_point
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Any key strictly within a range_consistent interval must map to the destination — this is correct by definition but confirms that range_consistent is NOT vacuous for proper intervals where lo < hi, providing a sanity counterpoint to the degenerate cases

