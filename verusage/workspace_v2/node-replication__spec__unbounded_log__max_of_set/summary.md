# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec__unbounded_log/spec__unbounded_log__max_of_set.rs`
**Date:** 2026-03-24T12:02:00Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: max_of_empty_is_zero
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The "maximum" of an empty set is defined as 0 — this is an arbitrary choice that could lead to unsound reasoning if callers assume the max is meaningful for empty sets

### φ2: max_geq_any_element
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Every element is bounded by the max — tests the core ensures clause on a concrete element

### φ3: max_not_tight
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The max is at least as large as the chosen element — but the spec does not guarantee the max is actually IN the set, so it could be arbitrarily larger

### φ4: singleton_max_is_element
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** For a singleton set, the max equals the element — but this relies on `choose()` picking the only element and the recursive structure working out

### φ5: max_monotone_subset
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Subset monotonicity of max — if max_of_set returns values larger than any actual element, a subset could have a larger "max" than its superset

