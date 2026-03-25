# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/verus_extra/verus_extra__lemma_flatten_set_seq_spec.rs`
**Date:** 2026-03-24T10:01:28Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives — they verify correct, expected mathematical properties of set union (empty base case, commutativity, idempotency, monotonicity). No spec gaps found.

## All Candidates

### φ1: flatten_empty_is_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If flattening an empty sequence produced a non-empty set, the fold_left base case would be unsound
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct definitional consequence of `fold_left` with `Set::empty()` base case on an empty sequence.

### φ2: flatten_singleton_equals_element
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If flattening a singleton sequence didn't equal the contained set, the fold_left would be incorrectly accumulating

### φ3: flatten_order_independent
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If flatten depended on ordering, the set union semantics would be broken — `fold_left` with union should be commutative
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Set union is commutative, so flattening two sets in either order produces the same result. This is a correct mathematical property.

### φ4: flatten_duplicate_sets_idempotent
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If duplicating a set in the sequence enlarged the result, union idempotency would be violated
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Set union is idempotent (`s ∪ s = s`), so flattening duplicate sets equals the original. Correct mathematical property.

### φ5: flatten_subset_of_append
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If appending a set could shrink the flattened result, the union-based fold_left would be losing elements
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Appending a set can only add elements via union, never remove them. This is a correct monotonicity property of union-based flattening.

