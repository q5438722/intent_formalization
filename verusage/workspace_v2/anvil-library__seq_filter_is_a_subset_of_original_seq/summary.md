# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/seq_filter_is_a_subset_of_original_seq.rs`
**Date:** 2026-03-24T04:55:12Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives — standard properties of `Seq::filter` proved from its definition. The lemma under test is fully verified (no external_body) and all candidates confirm expected behavior. No spec issues found.

## All Candidates

### φ1: filter_subset_containment
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** direct instantiation of the first ensures clause — tests that the universally quantified containment property holds at a specific element
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct instantiation of the lemma's first ensures clause. Correct and expected.

### φ2: filter_elements_satisfy_pred
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** filtered elements should satisfy the predicate — tests that filter's definition correctly restricts membership
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved from `reveal(Seq::filter)` without the lemma. Standard property of filter's definition.

### φ3: filter_len_leq_original
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** filter should never produce a longer sequence than the original
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved by induction on filter's definition. Standard property.

### φ4: filter_empty_is_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** filtering an empty sequence should produce empty — tests base case of filter definition
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Base case of filter's definition. Correct.

### φ5: filter_preserves_order
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** filter should preserve relative ordering of elements — if it doesn't, the subset relationship is weaker than expected

