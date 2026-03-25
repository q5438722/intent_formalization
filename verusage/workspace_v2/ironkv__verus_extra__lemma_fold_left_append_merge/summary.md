# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/ironkv/verified/verus_extra/verus_extra__lemma_fold_left_append_merge.rs`
**Date:** 2026-03-24T10:02:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 3

## Summary

All three candidates are false positives — they are correct, expected mathematical properties of fold_left with sequence concatenation (empty identity on both sides, and associative three-way decomposition). No spec gaps found.

## All Candidates

### φ1: fold_left_empty_left
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If folding an empty left sequence didn't produce the identity element, the merge lemma's split would lose or inject elements at the boundary
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Folding an empty sequence with `Seq::empty()` as the initial accumulator trivially returns `Seq::empty()`, which is the identity for sequence concatenation. This is a correct boundary property.

### φ2: fold_left_empty_right
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If appending an empty sequence on the right changed the fold result, the base case of the induction would be broken
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `s + Seq::empty() =~= s` by sequence extensionality, so the fold result is unchanged. Standard identity property.

### φ3: fold_left_singleton_split
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If folding two singletons didn't produce the simple concatenation f(a1)+f(a2), the fold_left would be reordering or duplicating mapped elements

### φ4: fold_left_three_way_associative
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If the merge lemma didn't compose associatively for three segments, multi-way splits would silently lose or duplicate elements
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This follows directly from applying the two-way merge lemma twice, combined with sequence concatenation associativity. Correct compositional property.

### φ5: fold_left_flatmap_len
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the output length didn't equal the sum of mapped lengths, the fold_left concatenation would be silently dropping or adding elements

