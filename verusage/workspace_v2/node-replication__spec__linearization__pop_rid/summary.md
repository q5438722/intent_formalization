# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/node-replication/verified/spec_linearization/spec__linearization__pop_rid.rs`
**Date:** 2026-03-24T12:06:53Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: pop_rid_choose_deterministic
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** `choose()` is deterministic in Verus specs — calling `pop_rid` twice on the same set always picks the same element, but the spec doesn't document which element is chosen

### φ2: pop_rid_result_strictly_smaller
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The result set has exactly one fewer element — tests that `remove` on a contained element decrements length by exactly 1

### φ3: pop_rid_singleton_empty
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Popping from a singleton set yields the empty set and the only element — tests the minimal case

### φ4: pop_rid_removed_not_in_result
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** The popped element is absent from the result set — tests that `remove` actually removes the element

### φ5: pop_rid_preserves_other_elements
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** All elements other than the popped one are preserved in the result — tests the frame condition of `remove`

