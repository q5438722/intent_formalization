# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__delegation_map_v__vec_erase/original.rs`
**Date:** 2026-03-24T21:50:19Z

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidate properties are false positives. The postcondition `v@ == old(v)@.subrange(0, start) + old(v)@.subrange(end, old(v)@.len())` is a complete and correct specification for erasing elements in the half-open range `[start, end)`. Each φ asserts a mathematically false consequence of this spec—claiming unchanged length after a non-empty erase (φ1), non-emptiness after full erase (φ2), emptiness when elements survive (φ3), persistence of erased elements (φ4), and length change on a no-op (φ5). None of these can be verified by Verus's SMT solver because they directly contradict the arithmetic and sequence-algebra implications of the correct postcondition. The `ensures true` conjunct is harmless (redundant), and the real spec is both sound and sufficiently strong.

## All Candidates

### φ1: length_unchanged_after_erase → `vec_erase`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Erasing a non-empty range must reduce length; unchanged length means elements were not actually removed
- **Verdict:** FALSE_POSITIVE (high)

### φ2: full_erase_leaves_elements → `vec_erase`
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Erasing all elements (start=0, end=len) must yield an empty vector; non-empty result means spec is too weak
- **Verdict:** FALSE_POSITIVE (high)

### φ3: any_erase_empties_vector → `vec_erase`
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** When elements remain after the erased range, result must be non-empty; emptiness means spec lost surviving elements
- **Verdict:** FALSE_POSITIVE (high)

### φ4: erased_element_still_present → `vec_erase`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** After erasing [start,end), position start should hold old_v[end], not old_v[start]; equality means the erased element persists
- **Verdict:** FALSE_POSITIVE (high)

### φ5: noop_erase_changes_length → `vec_erase`
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Erasing an empty range (start==end) must be a no-op; a length change means the spec incorrectly modifies the vector
- **Verdict:** FALSE_POSITIVE (high)

