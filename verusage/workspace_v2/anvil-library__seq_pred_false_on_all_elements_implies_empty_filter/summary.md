# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/seq_lib/seq_pred_false_on_all_elements_implies_empty_filter.rs`
**Date:** 2026-03-24T04:56:10Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives — standard properties of `Seq::filter` proved from its definition or direct applications of the fully verified lemma. No spec issues found.

## All Candidates

### φ1: filter_empty_seq
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** filtering an empty sequence should produce empty — tests base case of filter
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Base case of filter's definition. Correct and expected.

### φ2: all_false_implies_no_contains
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** direct instantiation of the precondition — tests that the universal quantifier triggers correctly
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct instantiation of a universal quantifier. Trivially correct.

### φ3: singleton_false_filter_empty
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** singleton with pred false should filter to empty — minimal nontrivial test
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct application of the lemma on a singleton. Correct.

### φ4: singleton_true_filter_nonempty
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** singleton with pred true should filter to nonempty — tests contrapositive direction not proven by the lemma
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved from filter's definition without the lemma. Correct contrapositive direction.

### φ5: filter_all_false_pred_empty
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** always-false predicate should yield empty filter on any sequence — tests converse with concrete predicate
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Proved by induction on filter's definition with concrete always-false predicate. Correct.

