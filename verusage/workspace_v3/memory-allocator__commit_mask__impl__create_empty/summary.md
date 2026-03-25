# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/memory-allocator__commit_mask__impl__create_empty/original.rs`
**Date:** 2026-03-24T21:56:58Z

## Stats

- Candidates generated: 5
- Entailed (verified): 1
- True positives: 0
- False positives: 1

## Summary

The single candidate property `empty_mask_nonzero` is a false positive. It attempts to derive `false` from the postcondition of `create_empty` (`self@ == Set::<int>::empty()`), which would indicate the spec is vacuously true. However, the `proof_from_false()` call cannot succeed because `is_bit_set` is opaque and the phi invokes no lemmas — the solver has no way to connect abstract view emptiness to concrete `usize` values, so no contradiction is derivable. The spec of `create_empty` is sound: it correctly ensures the abstract view is empty, and the implementation achieves this by zeroing all mask words and bridging to the abstraction via the trusted `lemma_is_bit_set` and `lemma_view`.

## All Candidates

### φ1: empty_contains_zero → `create_empty`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** An empty set must not contain element 0; provability would mean create_empty's postcondition is unsound or Set::empty is mis-specified.

### φ2: empty_contains_max_index → `create_empty`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** 511 is the maximum representable bit index (8*64-1); the empty set must not contain it, otherwise the boundary of the mask domain is broken.

### φ3: empty_contains_negative → `create_empty`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Negative indices are outside the valid bit range; if the empty set view contains -1, the view abstraction leaks invalid indices.

### φ4: empty_has_element → `create_empty`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** The empty set must have no elements; if an existential witness can be found, the spec of create_empty or the view is inconsistent with Set::empty.

### φ5: empty_mask_nonzero → `create_empty`
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If the view is empty the underlying mask word should be zero; provability would mean the abstraction allows non-canonical representations that silently hide set members.
- **Verdict:** FALSE_POSITIVE (high)

